use std::collections::BTreeMap;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use tracing::{debug, info};

#[derive(serde::Serialize)]
struct CommandManifest {
    program: String,
    arguments: Vec<String>,
    working_directory: Option<std::path::PathBuf>,
    /// Explicit overrides only; null means remove the inherited variable.
    environment_overrides: BTreeMap<String, Option<String>>,
}

/// Retain command output in logs and report progress without streaming compiler noise.
pub fn run(command: &mut Command, label: &str, stdout: &Path, stderr: &Path) -> Result<f64> {
    let manifest = CommandManifest {
        program: command.get_program().to_string_lossy().into_owned(),
        arguments: command.get_args().map(|s| s.to_string_lossy().into_owned()).collect(),
        working_directory: command.get_current_dir().map(Path::to_owned),
        environment_overrides: command
            .get_envs()
            .map(|(key, value)| {
                (
                    key.to_string_lossy().into_owned(),
                    value.map(|v| v.to_string_lossy().into_owned()),
                )
            })
            .collect(),
    };
    fs::write(stdout.with_extension("command.json"), serde_json::to_vec_pretty(&manifest)?)?;
    debug!(?command, "running command");
    info!("{label}");
    let log = File::create(stdout)?;
    let errors = File::create(stderr)?;
    let started = Instant::now();
    let mut child =
        command.stdout(log).stderr(errors).spawn().with_context(|| format!("start {label}"))?;
    let mut heartbeat = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if heartbeat.elapsed() >= Duration::from_secs(10) {
            info!("{label}: still running ({:.0}s)", started.elapsed().as_secs_f64());
            heartbeat = Instant::now();
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    let elapsed = started.elapsed().as_secs_f64();
    if !status.success() {
        bail!("{label} failed ({status}); see {} and {}", stdout.display(), stderr.display());
    }
    info!("{label}: finished in {elapsed:.1}s");
    Ok(elapsed)
}

pub fn output(command: &mut Command) -> Result<String> {
    let output = command.output().with_context(|| format!("run {command:?}"))?;
    if !output.status.success() {
        bail!("{command:?}: {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}
