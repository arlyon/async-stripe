#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use anyhow::Result;
use serde_json::json;

#[test]
fn completed_sizes_and_logs_survive_later_build_or_diagnostic_failure() -> Result<()> {
    let metadata = Command::new("cargo")
        .args(["metadata", "--no-deps", "--offline", "--locked", "--format-version", "1"])
        .output()?;
    assert!(metadata.status.success());
    let packages: serde_json::Value = serde_json::from_slice(&metadata.stdout)?;
    let package = packages["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "bench-minimal")
        .unwrap();
    let shared = packages["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "async-stripe-shared")
        .unwrap();
    for (phase, profile) in [("second-build", "release"), ("llvm", "release"), ("llvm", "dev")] {
        let temp = tempfile::tempdir()?;
        let script = temp.path().join("cargo");
        fs::write(
            &script,
            r#"#!/bin/sh
case "$1" in
    -vV) cat "$FAKE_RUSTC" ;;
    --version) echo 'cargo test fixture' ;;
    metadata) cat "$FAKE_METADATA" ;;
    build)
        if [ "$CARGO_INCREMENTAL" != "$EXPECTED_INCREMENTAL" ]; then
            echo 'incorrect incremental setting' >&2
            exit 8
        fi
        if [ "$FAIL_PHASE" = second-build ] && [ -f "$FAKE_MARKER" ]; then
            echo 'intentional build failure' >&2
            exit 9
        fi
        printf '%s\n' "$CARGO_TARGET_DIR" > "$FAKE_MARKER"
        if [ "$CARGO_INCREMENTAL" = 1 ]; then
            cache_dir="$FAKE_INCREMENTAL_DIR"
            mkdir -p "$cache_dir/bench_minimal-fixture/session" "$cache_dir/stripe_shared-fixture/session"
            printf '12345' > "$cache_dir/bench_minimal-fixture/session/query-cache.bin"
            printf '1234567' > "$cache_dir/stripe_shared-fixture/session/query-cache.bin"
        fi
        cat "$FAKE_ARTIFACT"
        echo 'fixture build progress' >&2
        ;;
    llvm-lines)
        if [ "$2" = --version ]; then echo 'cargo-llvm-lines test fixture';
        else echo 'intentional LLVM failure' >&2; exit 9; fi
        ;;
    *) echo 'unexpected command' >&2; exit 8 ;;
esac
"#,
        )?;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755))?;
        let version = Command::new("rustc").arg("-vV").output()?;
        assert!(version.status.success());
        fs::write(temp.path().join("rustc.txt"), version.stdout)?;
        fs::write(temp.path().join("metadata.json"), &metadata.stdout)?;
        // Use Cargo's reported output directory, independent of target-dir layout.
        let executable = temp.path().join("reported-output/bench-minimal");
        fs::create_dir(executable.parent().unwrap())?;
        fs::copy(std::env::current_exe()?, &executable)?;
        let artifact = json!({
            "reason": "compiler-artifact",
            "package_id": package["id"],
            "target": package["targets"][0],
            "filenames": [executable],
            "executable": executable,
            "features": [],
            "profile": { "opt_level": if profile == "dev" { "0" } else { "3" },
                "debuginfo": if profile == "dev" { 2 } else { 0 },
                "debug_assertions": profile == "dev", "overflow_checks": profile == "dev",
                "test": false },
            "fresh": false
        });
        let rlib = temp.path().join("libstripe_shared.rlib");
        fs::write(&rlib, [0; 3])?;
        let mut library_artifact = artifact.clone();
        library_artifact["package_id"] = shared["id"].clone();
        library_artifact["target"] = shared["targets"][0].clone();
        library_artifact["executable"] = serde_json::Value::Null;
        library_artifact["filenames"] = json!([rlib]);
        fs::write(temp.path().join("artifact.json"), format!("{artifact}\n{library_artifact}\n"))?;
        let output = temp.path().join("report");
        let mut command = Command::new(env!("CARGO_BIN_EXE_compile-bench"));
        command
            .args(["collect", "--cases", "minimal", "--profiles", profile, "--offline"])
            .arg("--output")
            .arg(&output)
            .env("CARGO", &script)
            .env("RUSTC", &script)
            .env("FAIL_PHASE", phase)
            .env("EXPECTED_INCREMENTAL", if profile == "dev" { "1" } else { "0" })
            .env("FAKE_INCREMENTAL_DIR", executable.parent().unwrap().join("incremental"))
            .env("FAKE_RUSTC", temp.path().join("rustc.txt"))
            .env("FAKE_METADATA", temp.path().join("metadata.json"))
            .env("FAKE_ARTIFACT", temp.path().join("artifact.json"))
            .env("FAKE_MARKER", temp.path().join("first-build"));
        if phase == "second-build" {
            command.args(["--timings", "2"]);
        } else {
            command.arg("--llvm-lines");
        }
        let result = command.output()?;
        assert!(!result.status.success());
        let report: serde_json::Value =
            serde_json::from_slice(&fs::read(output.join("results.json"))?)?;
        assert_eq!(report["finished"], true);
        assert!(report["measurements"][0]["sizes"]["executable_bytes"].as_u64().unwrap() > 0);
        assert!(!report["errors"].as_array().unwrap().is_empty());
        let sizes = &report["measurements"][0]["sizes"];
        if profile == "dev" {
            assert_eq!(sizes["consumer_incremental_bytes"], 5);
            assert_eq!(sizes["library_incremental_bytes"], 7);
        } else {
            assert!(sizes["consumer_incremental_bytes"].is_null());
            assert!(sizes["library_incremental_bytes"].is_null());
        }
        let target_dir = fs::read_to_string(temp.path().join("first-build"))?;
        assert!(!std::path::Path::new(target_dir.trim()).exists());
        let logs = output.join("logs").join(format!("minimal-{profile}"));
        assert!(logs.join("build-1.jsonl").is_file());
        assert!(fs::read_to_string(output.join("summary.md"))?.contains("Finished with errors"));
        let stderr = String::from_utf8(result.stderr)?;
        assert!(!stderr.contains("fixture build progress"));
        assert!(stderr.contains(": finished in "));
        assert!(
            fs::read_to_string(logs.join("build-1.stderr.log"))?.contains("fixture build progress")
        );
        if phase == "second-build" {
            assert_eq!(report["measurements"][0]["timing_samples"].as_array().unwrap().len(), 1);
            assert!(
                fs::read_to_string(logs.join("build-2.stderr.log"))?
                    .contains("intentional build failure")
            );
        } else {
            assert!(
                fs::read_to_string(logs.join("consumer-ir.stderr.log"))?
                    .contains("intentional LLVM failure")
            );
        }
        // Reusing an output path must not replace the useful partial report.
        let saved = fs::read(output.join("results.json"))?;
        assert!(!command.output()?.status.success());
        assert_eq!(fs::read(output.join("results.json"))?, saved);
    }
    Ok(())
}
