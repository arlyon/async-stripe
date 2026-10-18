#![cfg(any(target_os = "macos", target_os = "linux"))]

use std::process::{Command, Output, Stdio};

use anyhow::{Context, Result};

#[test]
fn startup_reports_errors_with_terminal_and_redirected_stderr() -> Result<()> {
    let executable = env!("CARGO_BIN_EXE_compile-bench");
    // Unlike --help, an invalid workload reaches logging initialization.
    let arguments = ["collect", "--cases", "invalid-smoke-case"];
    assert_usage_error(Command::new(executable).args(arguments).output()?);

    let mut terminal = Command::new("script");
    terminal.stdin(Stdio::null());
    #[cfg(target_os = "macos")]
    terminal.args(["-q", "/dev/null"]).arg(executable).args(arguments);
    #[cfg(target_os = "linux")]
    terminal
        .args([
            "-q",
            "-e",
            "-c",
            "exec \"$COMPILE_BENCH_SMOKE_BIN\" collect --cases invalid-smoke-case",
            "/dev/null",
        ])
        .env("COMPILE_BENCH_SMOKE_BIN", executable);
    assert_usage_error(terminal.output().context("run terminal smoke test with script")?);
    Ok(())
}

fn assert_usage_error(output: Output) {
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.status.code(), Some(1), "{text}");
    assert!(text.contains("unknown workload: invalid-smoke-case"), "{text}");
    assert!(!text.contains("panicked"), "{text}");
}
