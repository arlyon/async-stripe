use std::fs;

use anyhow::Context;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use serde_json::Value;

const VERSION_FILE_PATH: &str = "version.json";

#[derive(Debug, Clone)]
pub enum SpecVersion {
    Latest,
    Current,
    Version(String),
}

pub fn parse_spec_version(arg: &str) -> anyhow::Result<SpecVersion> {
    let as_lower = arg.to_lowercase();
    match as_lower.as_str() {
        "latest" => Ok(SpecVersion::Latest),
        "current" => Ok(SpecVersion::Current),
        version => {
            let re = Regex::new(r#"v[1-9]\d*"#).unwrap();
            if re.is_match(version) {
                Ok(SpecVersion::Version(as_lower))
            } else {
                Err(anyhow::anyhow!("Spec version should match `v###`, for example `v172`"))
            }
        }
    }
}

fn get_latest_openapi_tag(client: &Client) -> anyhow::Result<String> {
    let tags: Value = client
        .get("https://api.github.com/repos/stripe/openapi/releases/latest")
        // Github requires a user agent with the following recommendations:
        // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
        .header(USER_AGENT, "async-stripe")
        .send()?
        .error_for_status()?
        .json()?;
    Ok(tags.as_object().context("Unexpected response format")?["tag_name"]
        .as_str()
        .context("Unexpected tag name")?
        .to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionFile {
    version: String,
}

fn get_current_openapi_tag() -> anyhow::Result<String> {
    let raw = fs::File::open(VERSION_FILE_PATH).context("Could not find version file")?;
    let version_info: VersionFile =
        serde_json::from_reader(&raw).context("Failed to read JSON from version file")?;
    Ok(version_info.version)
}

pub fn fetch_spec(version: SpecVersion, in_path: &str) -> anyhow::Result<Value> {
    let client = Client::new();
    let version = match version {
        SpecVersion::Latest => get_latest_openapi_tag(&client)?,
        SpecVersion::Current => get_current_openapi_tag()?,
        SpecVersion::Version(v) => v,
    };
    log::info!("Fetching OpenAPI spec version {}", version);

    let url =
        format!("https://raw.githubusercontent.com/stripe/openapi/{}/openapi/spec3.json", version);

    let spec: Value = client.get(url).send()?.error_for_status()?.json()?;
    let writer = fs::File::create(in_path)?;
    serde_json::to_writer_pretty(writer, &spec)?;
    log::info!("Wrote OpenAPI spec to {}", in_path);

    let version_file_writer = fs::File::create(VERSION_FILE_PATH)?;
    serde_json::to_writer_pretty(version_file_writer, &VersionFile { version })?;

    Ok(spec)
}
