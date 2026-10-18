use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use tracing::{debug, info};

/// Retain command output in logs and report progress without streaming compiler noise.
pub fn run(command: &mut Command, label: &str, stdout: &Path, stderr: &Path) -> Result<f64> {
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
