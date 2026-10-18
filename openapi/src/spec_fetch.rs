use std::fs;
use std::path::Path;

use anyhow::Context;
use openapiv3::OpenAPI;
use regex_lite::Regex;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use serde_json::Value;
use tracing::info;

const SPEC_FILE_PATH: &str = "spec3.sdk.json";
const VERSION_FILE_PATH: &str = "version.json";
const FIXTURES_FILE_PATH: &str = "fixtures.json";
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/stripe/openapi/releases/latest";

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpecUpdate {
    Latest,
    Version(String),
}

pub fn parse_spec_update(arg: &str) -> anyhow::Result<SpecUpdate> {
    let as_lower = arg.to_lowercase();
    match as_lower.as_str() {
        "latest" => Ok(SpecUpdate::Latest),
        version if Regex::new(r"^v[1-9]\d*$").unwrap().is_match(version) => {
            Ok(SpecUpdate::Version(as_lower))
        }
        _ => anyhow::bail!("Spec update must be `latest` or match `v###`, for example `v172`"),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionFile {
    version: String,
}

pub fn load_pinned_spec() -> anyhow::Result<OpenAPI> {
    let root = Path::new(".");
    let version = read_pinned_tag(root)?;
    load_spec_version(root, &version, &mut fetch_json)
}

pub fn load_generation_spec(update: Option<SpecUpdate>) -> anyhow::Result<OpenAPI> {
    load_generation_spec_with(Path::new("."), update, &mut fetch_json)
}

fn load_generation_spec_with(
    root: &Path,
    update: Option<SpecUpdate>,
    fetch: &mut impl FnMut(&str) -> anyhow::Result<Value>,
) -> anyhow::Result<OpenAPI> {
    let updates_pin = update.is_some();
    let desired_version = match update {
        Some(SpecUpdate::Latest) => {
            let release = fetch(LATEST_RELEASE_URL)?;
            let tag = release
                .get("tag_name")
                .and_then(Value::as_str)
                .context("Latest release response has no tag name")?;
            let SpecUpdate::Version(version) = parse_spec_update(tag)? else {
                anyhow::bail!("Latest release tag must be an exact OpenAPI version");
            };
            version
        }
        Some(SpecUpdate::Version(version)) => version,
        None => read_pinned_tag(root)?,
    };

    let spec = load_spec_version(root, &desired_version, fetch)?;
    ensure_fixtures(root, &desired_version, fetch)?;
    if updates_pin && read_pinned_tag(root).ok().as_deref() != Some(&desired_version) {
        write_json(root.join(VERSION_FILE_PATH), &VersionFile { version: desired_version })?;
    }
    Ok(spec)
}

fn read_pinned_tag(root: &Path) -> anyhow::Result<String> {
    let file =
        fs::File::open(root.join(VERSION_FILE_PATH)).context("Could not find version file")?;
    let pinned: VersionFile =
        serde_json::from_reader(file).context("Failed to read version file")?;
    Ok(pinned.version)
}

fn load_spec_version(
    root: &Path,
    version: &str,
    fetch: &mut impl FnMut(&str) -> anyhow::Result<Value>,
) -> anyhow::Result<OpenAPI> {
    let path = root.join(SPEC_FILE_PATH);
    if let Some(spec) = fs::File::open(&path)
        .ok()
        .and_then(|file| serde_json::from_reader::<_, OpenAPI>(file).ok())
        .filter(|spec| {
            spec.info.extensions.get("x-stripeTag").and_then(Value::as_str) == Some(version)
        })
    {
        info!("Using cached OpenAPI spec version {version}");
        return Ok(spec);
    }

    info!("Fetching OpenAPI spec version {version}");
    let url = format!(
        "https://raw.githubusercontent.com/stripe/openapi/{version}/openapi/spec3.sdk.json"
    );
    let mut spec: OpenAPI =
        serde_json::from_value(fetch(&url)?).context("Failed to parse downloaded OpenAPI spec")?;
    spec.info.extensions.insert("x-stripeTag".to_string(), version.into());
    write_json(path, &spec)?;
    Ok(spec)
}

fn ensure_fixtures(
    root: &Path,
    version: &str,
    fetch: &mut impl FnMut(&str) -> anyhow::Result<Value>,
) -> anyhow::Result<()> {
    let path = root.join(FIXTURES_FILE_PATH);
    let matches = fs::File::open(&path)
        .ok()
        .and_then(|file| serde_json::from_reader::<_, VersionFile>(file).ok())
        .is_some_and(|fixtures| fixtures.version == version);
    if matches {
        return Ok(());
    }

    info!("Fetching fixtures for OpenAPI spec version {version}");
    let url = format!(
        "https://raw.githubusercontent.com/stripe/openapi/{version}/openapi/fixtures3.json"
    );
    let mut fixtures = fetch(&url)?;
    fixtures
        .as_object_mut()
        .context("Downloaded fixtures must be an object")?
        .insert("version".into(), Value::String(version.into()));
    write_json(path, &fixtures)
}

fn fetch_json(url: &str) -> anyhow::Result<Value> {
    // Construct the HTTP client only on a cache miss.
    Client::new()
        .get(url)
        .header(USER_AGENT, "async-stripe")
        .send()?
        .error_for_status()?
        .json()
        .context("Failed to decode downloaded JSON")
}

fn write_json(path: impl AsRef<Path>, value: &impl serde::Serialize) -> anyhow::Result<()> {
    // Serialize before opening the destination so serialization errors cannot truncate it.
    fs::write(path.as_ref(), serde_json::to_vec_pretty(value)?)
        .with_context(|| format!("Could not write {}", path.as_ref().display()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde_json::json;

    use super::*;

    static NEXT_CACHE: AtomicUsize = AtomicUsize::new(0);

    struct Cache(PathBuf);

    impl Cache {
        fn new() -> Self {
            let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let sequence = NEXT_CACHE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "async-stripe-spec-cache-{}-{unique}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&root).unwrap();
            let cache = Self(root);
            cache.write(VERSION_FILE_PATH, json!({"version": "v100"}));
            cache.write(SPEC_FILE_PATH, spec("v100"));
            cache.write(FIXTURES_FILE_PATH, json!({"resources": {}, "version": "v100"}));
            cache
        }

        fn write(&self, file: &str, value: Value) {
            write_json(self.0.join(file), &value).unwrap();
        }

        fn load(
            &self,
            update: Option<SpecUpdate>,
            mut fetch: impl FnMut(&str) -> anyhow::Result<Value>,
        ) -> anyhow::Result<OpenAPI> {
            load_generation_spec_with(&self.0, update, &mut fetch)
        }
    }

    impl Drop for Cache {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn spec(version: &str) -> Value {
        json!({
            "openapi": "3.0.0",
            "info": {"title": "Fixture", "version": "1", "x-stripeTag": version},
            "paths": {}
        })
    }

    fn no_fetch(url: &str) -> anyhow::Result<Value> {
        panic!("unexpected download: {url}");
    }

    #[test]
    fn accepts_only_complete_release_selectors() {
        assert_eq!(parse_spec_update("LATEST").unwrap(), SpecUpdate::Latest);
        assert_eq!(parse_spec_update("V123").unwrap(), SpecUpdate::Version("v123".into()));
        for invalid in ["current", "v0", "v01", "prefix-v123", "v123-suffix", "123", ""] {
            assert!(parse_spec_update(invalid).is_err(), "{invalid}");
        }
    }

    #[test]
    fn matching_cached_inputs_need_no_download_or_pin_write() {
        let cache = Cache::new();
        // Preserve even the existing pin's formatting on an ordinary run.
        let pin = b"{\"version\":\"v100\"}\n";
        fs::write(cache.0.join(VERSION_FILE_PATH), pin).unwrap();
        cache.load(None, no_fetch).unwrap();
        assert_eq!(fs::read(cache.0.join(VERSION_FILE_PATH)).unwrap(), pin);
    }

    #[test]
    fn repairs_missing_malformed_and_mismatched_specs() {
        for cached in
            [None, Some("invalid JSON"), Some("{}"), Some("{\"info\":{\"x-stripeTag\":\"v100\"}}")]
        {
            let cache = Cache::new();
            let path = cache.0.join(SPEC_FILE_PATH);
            if let Some(cached) = cached {
                fs::write(&path, cached).unwrap();
            } else {
                fs::remove_file(&path).unwrap();
            }
            let mut downloads = 0;
            let loaded = cache
                .load(None, |url| {
                    assert!(url.ends_with("/v100/openapi/spec3.sdk.json"));
                    downloads += 1;
                    Ok(spec("upstream"))
                })
                .unwrap();
            assert_eq!(downloads, 1);
            assert_eq!(loaded.info.extensions["x-stripeTag"], "v100");
            let stored: OpenAPI = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
            assert_eq!(stored.info.extensions["x-stripeTag"], "v100");
        }
        let cache = Cache::new();
        cache.write(SPEC_FILE_PATH, spec("v99"));
        cache
            .load(None, |url| {
                assert!(url.ends_with("/v100/openapi/spec3.sdk.json"));
                Ok(spec("v100"))
            })
            .unwrap();
    }

    #[test]
    fn repairs_missing_malformed_and_mismatched_fixtures() {
        for cached in [None, Some("invalid JSON"), Some("{}"), Some("{\"version\":\"v99\"}")] {
            let cache = Cache::new();
            let path = cache.0.join(FIXTURES_FILE_PATH);
            if let Some(cached) = cached {
                fs::write(&path, cached).unwrap();
            } else {
                fs::remove_file(&path).unwrap();
            }
            let mut downloads = 0;
            cache
                .load(None, |url| {
                    assert!(url.ends_with("/v100/openapi/fixtures3.json"));
                    downloads += 1;
                    Ok(json!({"resources": {}}))
                })
                .unwrap();
            assert_eq!(downloads, 1);
            let fixtures: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
            assert_eq!(fixtures["version"], "v100");
        }
    }

    #[test]
    fn updates_pin_after_both_inputs_are_available() {
        let cache = Cache::new();
        let mut downloads = Vec::new();
        cache
            .load(Some(SpecUpdate::Version("v101".into())), |url| {
                assert_eq!(read_pinned_tag(&cache.0).unwrap(), "v100");
                downloads.push(url.to_owned());
                if url.ends_with("/spec3.sdk.json") {
                    Ok(spec("upstream"))
                } else {
                    Ok(json!({"resources": {}}))
                }
            })
            .unwrap();
        assert_eq!(downloads.len(), 2);
        assert!(downloads.iter().all(|url| url.contains("/v101/")));
        assert_eq!(read_pinned_tag(&cache.0).unwrap(), "v101");
        cache.load(None, no_fetch).unwrap();
    }

    #[test]
    fn updating_to_an_already_cached_release_still_advances_the_pin() {
        let cache = Cache::new();
        cache.write(SPEC_FILE_PATH, spec("v101"));
        cache.write(FIXTURES_FILE_PATH, json!({"resources": {}, "version": "v101"}));
        cache.load(Some(SpecUpdate::Version("v101".into())), no_fetch).unwrap();
        assert_eq!(read_pinned_tag(&cache.0).unwrap(), "v101");
    }

    #[test]
    fn failed_downloads_leave_the_pin_unchanged() {
        for failed_input in ["spec3.sdk.json", "fixtures3.json"] {
            let cache = Cache::new();
            let pin = fs::read(cache.0.join(VERSION_FILE_PATH)).unwrap();
            let result = cache.load(Some(SpecUpdate::Version("v101".into())), |url| {
                if url.ends_with(failed_input) {
                    anyhow::bail!("simulated download failure");
                }
                Ok(spec("upstream"))
            });
            assert!(result.is_err());
            assert_eq!(fs::read(cache.0.join(VERSION_FILE_PATH)).unwrap(), pin);
        }
    }

    #[test]
    fn latest_resolves_release_before_synchronizing_inputs() {
        let cache = Cache::new();
        let mut downloads = Vec::new();
        cache
            .load(Some(SpecUpdate::Latest), |url| {
                downloads.push(url.to_owned());
                if url == LATEST_RELEASE_URL {
                    Ok(json!({"tag_name": "v102"}))
                } else if url.ends_with("/spec3.sdk.json") {
                    Ok(spec("upstream"))
                } else {
                    Ok(json!({"resources": {}}))
                }
            })
            .unwrap();
        assert_eq!(downloads[0], LATEST_RELEASE_URL);
        assert!(downloads[1..].iter().all(|url| url.contains("/v102/")));
        assert_eq!(read_pinned_tag(&cache.0).unwrap(), "v102");
    }

    #[test]
    fn invalid_downloads_and_latest_responses_leave_the_pin_unchanged() {
        for update in [SpecUpdate::Latest, SpecUpdate::Version("v101".into())] {
            let cache = Cache::new();
            assert!(cache.load(Some(update), |_| Ok(json!({}))).is_err());
            assert_eq!(read_pinned_tag(&cache.0).unwrap(), "v100");
        }
        let cache = Cache::new();
        assert!(
            cache
                .load(Some(SpecUpdate::Version("v101".into())), |url| {
                    if url.ends_with("/spec3.sdk.json") { Ok(spec("v101")) } else { Ok(json!([])) }
                })
                .is_err()
        );
        assert_eq!(read_pinned_tag(&cache.0).unwrap(), "v100");
    }
}
