use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail, ensure};
use cargo_metadata::Metadata;
use sha2::{Digest, Sha256};
use tracing::{info, warn};

use crate::Collect;
use crate::metrics;
use crate::model::*;
use crate::{process, report};

struct Build {
    cargo: PathBuf,
    rustc: std::ffi::OsString,
    repo: PathBuf,
    target: String,
    flags: Vec<String>,
    offline: bool,
}

impl Build {
    fn command(
        &self,
        subcommand: &str,
        profile: &str,
        target_dir: &Path,
        rss: Option<&Path>,
    ) -> Command {
        let mut command = if let Some(log) = rss {
            let mut command = time_command(log);
            command.arg(&self.cargo);
            command
        } else {
            Command::new(&self.cargo)
        };
        command.arg(subcommand).args(["--locked", "--profile", profile, "--target", &self.target]);
        if self.offline {
            command.arg("--offline");
        }
        let mut flags = self.flags.clone();
        for (path, replacement) in [
            (self.repo.clone(), "/workspace"),
            (target_dir.to_owned(), "/build"),
            (cargo_home(), "/cargo"),
        ] {
            flags.push(format!("--remap-path-prefix={}={replacement}", path.display()));
        }
        command
            .current_dir(&self.repo)
            .env("CARGO_TARGET_DIR", target_dir)
            .env("CARGO_INCREMENTAL", if profile == "dev" { "1" } else { "0" })
            .env("RUSTC", &self.rustc)
            .env("RUSTC_WRAPPER", "")
            .env("RUSTC_WORKSPACE_WRAPPER", "")
            .env_remove("RUSTFLAGS")
            .env("CARGO_ENCODED_RUSTFLAGS", flags.join("\u{1f}"))
            .env(format!("CARGO_PROFILE_{}_SPLIT_DEBUGINFO", profile.to_uppercase()), "off")
            .env(format!("CARGO_PROFILE_{}_STRIP", profile.to_uppercase()), "none");
        command
    }
}

pub fn collect(args: Collect) -> Result<()> {
    let cases: Vec<String> = if args.cases == "all" {
        CASES.iter().map(|s| (*s).to_owned()).collect()
    } else {
        args.cases.split(',').map(str::to_owned).collect()
    };
    validate_selection(&cases, CASES, "workload")?;
    validate_selection(&args.profiles, &["dev", "release"], "profile")?;
    let cargo = env::var_os("CARGO").map(PathBuf::from).unwrap_or_else(|| "cargo".into());
    let repo = repository(&cargo, args.repo.as_deref(), &env::current_dir()?)?;
    let commit =
        process::output(Command::new("git").current_dir(&repo).args(["rev-parse", "HEAD"]))?;
    let dirty =
        !process::output(Command::new("git").current_dir(&repo).args(["status", "--porcelain"]))?
            .is_empty();
    info!(
        "Measuring repository: {} at {}{}",
        repo.display(),
        commit,
        if dirty { " (dirty)" } else { "" }
    );
    let output = if let Some(output) = args.output {
        let output = if output.is_absolute() { output } else { repo.join(output) };
        fs::create_dir_all(output.parent().context("output directory has no parent")?)?;
        fs::create_dir(&output).with_context(|| {
            format!(
                "create {}; choose a new --output directory for each collection",
                output.display()
            )
        })?;
        output
    } else {
        let parent = repo.join("target/compile-benchmarks");
        fs::create_dir_all(&parent)?;
        tempfile::Builder::new().prefix("run-").tempdir_in(parent)?.keep()
    };
    fs::create_dir(output.join("logs"))?;
    info!("Reports and logs: {}", output.display());
    // Pin the compiler we inspect, rather than reimplement Cargo's config resolution.
    let rustc_program = env::var_os("RUSTC")
        .or_else(|| env::var_os("CARGO_BUILD_RUSTC"))
        .unwrap_or_else(|| "rustc".into());
    info!("Compiler for measured builds: {rustc_program:?}");
    let rustc = process::output(Command::new(&rustc_program).arg("-vV").current_dir(&repo))?;
    let target = args.target.clone().unwrap_or(
        rustc
            .lines()
            .find_map(|line| line.strip_prefix("host: "))
            .context("rustc has no host triple")?
            .to_owned(),
    );
    let flags = rustflags();
    let mut build = Build {
        cargo,
        rustc: rustc_program,
        repo: repo.clone(),
        target: target.clone(),
        flags: flags.clone(),
        offline: args.offline,
    };
    let metadata: Metadata = serde_json::from_str(&process::output(
        Command::new(&build.cargo).current_dir(&repo).args([
            "metadata",
            "--locked",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
        ]),
    )?)?;
    let llvm_version = if args.llvm_lines {
        Some(process::output(
            Command::new(&build.cargo).current_dir(&repo).args(["llvm-lines", "--version"]),
        ))
    } else {
        None
    };
    let mut results = Report {
        schema_version: SCHEMA_VERSION,
        started_unix_seconds: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        finished: false,
        collector: Some(collector_identity()),
        source: Source {
            repository: Some(repo.clone()),
            commit,
            dirty,
            lockfile_sha256: hash(&fs::read(repo.join("Cargo.lock"))?),
            workload_sha256: cases.iter().map(|case| Ok((case.clone(), workload_hash(&repo, case)?))).collect::<Result<_>>()?,
        },
        environment: Environment {
            rustc,
            cargo: process::output(Command::new(&build.cargo).current_dir(&repo).arg("--version"))?,
            platform: format!("{}/{}", env::consts::OS, env::consts::ARCH),
            target,
            rustflags: flags,
            build_environment: build_environment(),
            cargo_configuration: cargo_configuration(&repo)?,
            workspace_profiles: workspace_profiles(&repo)?,
            policy: "explicit RUSTC; isolated target; incremental=dev only; wrappers=off; split-debuginfo=off; strip=none; remap repo/target/CARGO_HOME paths".into(),
        },
        options: Options {
            cases,
            profiles: args.profiles.clone(),
            timing_runs: args.timings.map(usize::from),
            llvm_lines: llvm_version.as_ref().and_then(|version| version.as_ref().ok().cloned()),
            offline: args.offline,
            keep_artifacts: args.keep_artifacts,
        },
        measurements: Vec::new(),
        library_ir: Vec::new(),
        errors: Vec::new(),
    };
    if let Some(Err(error)) = llvm_version {
        record_error(&mut results, format!("LLVM diagnostics unavailable: {error:#}"));
    }
    report::save(&output, &results)?;
    if args.timings.is_some() && !args.offline {
        // Cargo fetch is workspace-wide. Keep all download time outside timing samples.
        let fetch = process::run(
            Command::new(&build.cargo).current_dir(&repo).args([
                "fetch",
                "--locked",
                "--target",
                &build.target,
            ]),
            "Fetching dependencies before timing",
            &output.join("logs/fetch.stdout.log"),
            &output.join("logs/fetch.stderr.log"),
        );
        if let Err(error) = fetch {
            record_error(&mut results, format!("dependency fetch: {error:#}"));
            results.finished = true;
            report::save(&output, &results)?;
            bail!(
                "dependency fetch failed; partial report: {}",
                output.join("summary.md").display()
            );
        }
        build.offline = true;
    }
    let rss = args.timings.is_some() && rss_available(&output);
    let mut shared_profiles = BTreeSet::new();
    for case in results.options.cases.clone() {
        for profile in &args.profiles {
            let index = results.measurements.len();
            info!(
                "[{}/{}] {case}/{profile}",
                index + 1,
                results.options.cases.len() * args.profiles.len()
            );
            results.measurements.push(Measurement {
                case: case.clone(),
                profile: profile.clone(),
                sizes: None,
                timing_samples: Vec::new(),
                consumer_ir: None,
                warnings: Vec::new(),
                retained_artifacts: Vec::new(),
            });
            if args.timings.is_some() && !rss {
                results.measurements[index].warnings.push(
                    "Maximum RSS unavailable on this host; wall time is still collected.".into(),
                );
            }
            report::save(&output, &results)?;
            if let Err(error) =
                measure(&build, &output, &metadata, &mut results, index, rss, &mut shared_profiles)
            {
                record_error(&mut results, format!("{case}/{profile}: {error:#}"));
            }
            report::save(&output, &results)?;
        }
    }
    results.finished = true;
    report::save(&output, &results)?;
    info!(
        "Wrote {} and {}",
        output.join("summary.md").display(),
        output.join("results.json").display()
    );
    ensure!(
        results.errors.is_empty(),
        "collection finished with errors; see {}",
        output.join("summary.md").display()
    );
    Ok(())
}

fn measure(
    build: &Build,
    output: &Path,
    metadata: &Metadata,
    results: &mut Report,
    index: usize,
    rss: bool,
    shared_profiles: &mut BTreeSet<String>,
) -> Result<()> {
    let case = results.measurements[index].case.clone();
    let profile = results.measurements[index].profile.clone();
    let name = format!("bench-{case}");
    let logs = output.join("logs").join(format!("{case}-{profile}"));
    fs::create_dir_all(&logs)?;
    let keep = results.options.keep_artifacts;
    let first_target =
        build_target(output, &case, &profile, 1, keep, &mut results.measurements[index])?;
    let runs = results.options.timing_runs.unwrap_or(1);
    for run in 1..=runs {
        let extra_target;
        let target_dir = if run == 1 {
            &first_target.path
        } else {
            extra_target =
                build_target(output, &case, &profile, run, keep, &mut results.measurements[index])?;
            &extra_target.path
        };
        // Persist retained targets before starting, so failed builds remain discoverable.
        report::save(output, results)?;
        let stdout = logs.join(format!("build-{run}.jsonl"));
        let stderr = logs.join(format!("build-{run}.stderr.log"));
        let rss_log = logs.join(format!("build-{run}.rss.log"));
        let mut command =
            build.command("build", &profile, target_dir, rss.then_some(rss_log.as_path()));
        command.args(["-p", &name, "--bin", &name, "--message-format=json-render-diagnostics"]);
        let seconds = process::run(
            &mut command,
            &format!("{case}/{profile}: clean build {run}/{runs}"),
            &stdout,
            &stderr,
        )?;
        if run == 1 {
            let row = &mut results.measurements[index];
            row.sizes = Some(metrics::collect(
                &stdout,
                &metadata.packages,
                &name,
                target_dir,
                profile == "dev",
                &mut row.warnings,
            )?);
            if keep != KeepArtifacts::None {
                let source = metrics::executable(&stdout, &metadata.packages, &name)?;
                let retained = if keep == KeepArtifacts::Target && source.starts_with(output) {
                    source
                } else {
                    let path = output
                        .join("artifacts")
                        .join(format!("{case}-{profile}"))
                        .join("executable");
                    fs::create_dir_all(path.parent().unwrap())?;
                    fs::copy(source, &path)?;
                    path
                };
                row.retained_artifacts.push(RetainedArtifact {
                    build: run,
                    kind: "executable".into(),
                    path: retained.strip_prefix(output)?.to_owned(),
                });
            }
        }
        if results.options.timing_runs.is_some() {
            let max_rss_bytes = if rss {
                metrics::max_rss(&fs::read_to_string(&rss_log)?, env::consts::OS)
            } else {
                None
            };
            results.measurements[index]
                .timing_samples
                .push(TimingSample { wall_seconds: seconds, max_rss_bytes });
        }
        report::save(output, results)?;
    }
    if results.options.llvm_lines.is_some() {
        // IR diagnostics may rebuild/replace artifacts; preserve the measured target.
        let diagnostic_target = tempfile::Builder::new().prefix("stripe-bench-ir-").tempdir()?;
        let consumer = llvm(
            build,
            output,
            diagnostic_target.path(),
            &profile,
            &name,
            false,
            &logs.join("consumer-ir.txt"),
        );
        match consumer {
            Ok(totals) => results.measurements[index].consumer_ir = Some(totals),
            Err(error) => record_error(results, format!("{case}/{profile} consumer IR: {error:#}")),
        }
        report::save(output, results)?;
        if shared_profiles.insert(profile.clone()) {
            let shared = llvm(
                build,
                output,
                diagnostic_target.path(),
                &profile,
                "async-stripe-shared",
                true,
                &logs.join("standalone-shared-ir.txt"),
            );
            match shared {
                Ok(totals) => results.library_ir.push(LibraryIr {
                    package: "async-stripe-shared".into(),
                    profile,
                    totals,
                }),
                Err(error) => {
                    record_error(results, format!("standalone shared-library IR: {error:#}"))
                }
            }
        }
    }
    for warning in &results.measurements[index].warnings {
        warn!("{case}: {warning}");
    }
    Ok(())
}

struct BuildTarget {
    path: PathBuf,
    _temporary: Option<tempfile::TempDir>,
}

fn build_target(
    output: &Path,
    case: &str,
    profile: &str,
    run: usize,
    keep: KeepArtifacts,
    row: &mut Measurement,
) -> Result<BuildTarget> {
    if keep == KeepArtifacts::Target {
        let relative = PathBuf::from("artifacts")
            .join(format!("{case}-{profile}"))
            .join(format!("build-{run}"))
            .join("target");
        let path = output.join(&relative);
        fs::create_dir_all(&path)?;
        row.retained_artifacts.push(RetainedArtifact {
            build: run,
            kind: "target".into(),
            path: relative,
        });
        Ok(BuildTarget { path, _temporary: None })
    } else {
        let temporary = tempfile::Builder::new().prefix("stripe-bench-").tempdir()?;
        Ok(BuildTarget { path: temporary.path().to_owned(), _temporary: Some(temporary) })
    }
}

fn repository(cargo: &Path, explicit: Option<&Path>, invocation_dir: &Path) -> Result<PathBuf> {
    let path = if let Some(path) = explicit {
        path.to_owned()
    } else {
        let manifest = process::output(Command::new(cargo).current_dir(invocation_dir).args([
            "locate-project",
            "--workspace",
            "--message-format=plain",
        ]))
        .context("find current Cargo workspace; use --repo to select a checkout explicitly")?;
        Path::new(&manifest).parent().context("workspace manifest has no parent")?.to_owned()
    };
    let path = if path.is_absolute() { path } else { invocation_dir.join(path) };
    path.canonicalize().with_context(|| format!("find repository {}", path.display()))
}

fn collector_identity() -> Collector {
    let mut digest = Sha256::new();
    for (name, bytes) in [
        ("Cargo.toml", include_bytes!("../Cargo.toml").as_slice()),
        ("main.rs", include_bytes!("main.rs").as_slice()),
        ("collect.rs", include_bytes!("collect.rs").as_slice()),
        ("metrics.rs", include_bytes!("metrics.rs").as_slice()),
        ("model.rs", include_bytes!("model.rs").as_slice()),
        ("process.rs", include_bytes!("process.rs").as_slice()),
        ("report.rs", include_bytes!("report.rs").as_slice()),
    ] {
        digest.update(name);
        digest.update([0]);
        digest.update(bytes);
        digest.update([0]);
    }
    Collector {
        version: env!("CARGO_PKG_VERSION").into(),
        source_sha256: format!("{:x}", digest.finalize()),
    }
}

fn llvm(
    build: &Build,
    output: &Path,
    target: &Path,
    profile: &str,
    package: &str,
    library: bool,
    log: &Path,
) -> Result<Ir> {
    let mut command = build.command("llvm-lines", profile, target, None);
    command.args(["-p", package]);
    if library {
        command.arg("--lib");
    } else {
        command.args(["--bin", package]);
    }
    process::run(
        &mut command,
        &format!("{package}/{profile}: LLVM IR (selected crate only)"),
        log,
        &log.with_extension("stderr.log"),
    )?;
    metrics::ir_totals(
        &fs::read_to_string(log)?,
        log.strip_prefix(output)?.to_string_lossy().into_owned(),
    )
}

fn record_error(results: &mut Report, error: String) {
    warn!("{error}");
    results.errors.push(error);
}

fn validate_selection(values: &[String], allowed: &[&str], kind: &str) -> Result<()> {
    ensure!(!values.is_empty(), "select at least one {kind}");
    let mut seen = BTreeSet::new();
    for value in values {
        ensure!(
            allowed.contains(&value.as_str()),
            "unknown {kind}: {value}; choose {}",
            allowed.join(", ")
        );
        ensure!(seen.insert(value), "duplicate {kind}: {value}");
    }
    Ok(())
}

fn cargo_home() -> PathBuf {
    env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".cargo"))
}

fn rustflags() -> Vec<String> {
    if let Ok(flags) = env::var("CARGO_ENCODED_RUSTFLAGS") {
        flags.split('\u{1f}').filter(|s| !s.is_empty()).map(str::to_owned).collect()
    } else {
        env::var("RUSTFLAGS").unwrap_or_default().split_whitespace().map(str::to_owned).collect()
    }
}

fn build_environment() -> BTreeMap<String, String> {
    env::vars()
        .filter(|(key, _)| {
            matches!(
                key.as_str(),
                "RUSTC"
                    | "CARGO_BUILD_RUSTC"
                    | "RUSTC_BOOTSTRAP"
                    | "CC"
                    | "CXX"
                    | "AR"
                    | "CFLAGS"
                    | "CXXFLAGS"
                    | "LDFLAGS"
                    | "SDKROOT"
                    | "MACOSX_DEPLOYMENT_TARGET"
                    | "CARGO_BUILD_JOBS"
            ) || key.starts_with("CARGO_PROFILE_")
                || (key.starts_with("CARGO_TARGET_") && key.ends_with("_LINKER"))
        })
        .collect()
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn workspace_profiles(repo: &Path) -> Result<serde_json::Value> {
    // Record inputs, including package/build overrides; let Cargo resolve them.
    // Artifact messages contain only a subset of the effective profile settings.
    let manifest: toml::Table = toml::from_str(&fs::read_to_string(repo.join("Cargo.toml"))?)?;
    Ok(manifest
        .get("profile")
        .map(serde_json::to_value)
        .transpose()?
        .unwrap_or_else(|| serde_json::json!({})))
}

fn cargo_configuration(repo: &Path) -> Result<BTreeMap<String, String>> {
    let mut hashes = BTreeMap::new();
    for (index, directory) in repo.ancestors().enumerate() {
        for name in ["config", "config.toml"] {
            let path = directory.join(".cargo").join(name);
            if path.is_file() {
                hashes.insert(format!("ancestor-{index}/.cargo/{name}"), hash(&fs::read(path)?));
            }
        }
    }
    for name in ["config", "config.toml"] {
        let path = cargo_home().join(name);
        if path.is_file() {
            hashes.insert(format!("CARGO_HOME/{name}"), hash(&fs::read(path)?));
        }
    }
    Ok(hashes)
}

fn workload_hash(repo: &Path, case: &str) -> Result<String> {
    fn files(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                files(&entry.path(), paths)?;
            } else {
                paths.push(entry.path());
            }
        }
        Ok(())
    }
    let root = repo.join("bench/compile/slices").join(case);
    let mut paths = vec![root.join("Cargo.toml")];
    files(&root.join("src"), &mut paths)?;
    paths.sort();
    let mut digest = Sha256::new();
    for path in paths {
        digest.update(path.strip_prefix(&root)?.as_os_str().as_encoded_bytes());
        digest.update([0]);
        digest.update(fs::read(path)?);
        digest.update([0]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn time_command(log: &Path) -> Command {
    let mut command = Command::new("/usr/bin/time");
    match env::consts::OS {
        "macos" => {
            command.arg("-l");
        }
        "linux" => {
            command.args(["-f", "%M"]);
        }
        _ => {}
    }
    command.arg("-o").arg(log);
    command
}

fn rss_available(output: &Path) -> bool {
    if !matches!(env::consts::OS, "macos" | "linux") {
        return false;
    }
    let log = output.join("logs/rss-probe.txt");
    match time_command(&log).arg("true").output() {
        Ok(result) if result.status.success() => fs::read_to_string(log)
            .ok()
            .and_then(|text| metrics::max_rss(&text, env::consts::OS))
            .is_some(),
        _ => {
            warn!("/usr/bin/time RSS measurement unavailable; collecting wall time only");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_follows_the_invocation_workspace_and_explicit_overrides() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let workspace = temp.path().join("workspace");
        let nested = workspace.join("nested/directory");
        fs::create_dir_all(&nested)?;
        fs::write(workspace.join("Cargo.toml"), "[workspace]\nmembers = []\n")?;
        let cargo = env::var_os("CARGO").map(PathBuf::from).unwrap_or_else(|| "cargo".into());
        assert_eq!(repository(&cargo, None, &nested)?, workspace.canonicalize()?);
        let explicit = temp.path().join("other-checkout");
        fs::create_dir(&explicit)?;
        assert_eq!(repository(&cargo, Some(&explicit), &nested)?, explicit.canonicalize()?);
        assert_eq!(
            repository(&cargo, Some(Path::new("../..")), &nested)?,
            workspace.canonicalize()?
        );
        assert!(
            repository(&cargo, None, temp.path()).unwrap_err().to_string().contains("use --repo")
        );
        Ok(())
    }
}
