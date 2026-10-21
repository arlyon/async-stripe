use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, ensure};

use crate::metrics::{debug_bytes, section_bytes};
use crate::model::*;

pub fn save(directory: &Path, report: &Report) -> Result<()> {
    // Replace whole files so interrupted writes don't corrupt earlier measurements.
    let json = directory.join("results.json.tmp");
    fs::write(&json, serde_json::to_string_pretty(report)? + "\n")?;
    fs::rename(json, directory.join("results.json"))?;
    let summary = directory.join("summary.md.tmp");
    fs::write(&summary, markdown(report))?;
    fs::rename(summary, directory.join("summary.md"))?;
    Ok(())
}

fn bytes(bytes: u64) -> String {
    format!("{:.2} MiB", bytes as f64 / 1_048_576.0)
}
fn optional(bytes: Option<u64>) -> String {
    bytes.map_or_else(|| "—".into(), self::bytes)
}

fn sizes(sizes: Option<&Sizes>) -> [Option<u64>; 6] {
    let Some(sizes) = sizes else { return [None; 6] };
    [
        Some(sizes.executable_bytes),
        sizes.stripped_executable_bytes,
        sizes.sections.as_ref().map(|s| section_bytes(s, &["Text"])),
        sizes.sections.as_ref().map(|s| debug_bytes(s)),
        Some(sizes.libraries.iter().map(|l| l.rlib_bytes).sum()),
        Some(sizes.libraries.iter().map(|l| l.rmeta_bytes).sum()),
    ]
}

fn incremental_sizes(sizes: Option<&Sizes>) -> [Option<u64>; 2] {
    let Some(sizes) = sizes else { return [None; 2] };
    [sizes.consumer_incremental_bytes, sizes.library_incremental_bytes]
}

fn markdown(report: &Report) -> String {
    let mut out = String::from("# Compile benchmark\n\n");
    let status = if !report.finished {
        "In progress / interrupted"
    } else if report.errors.is_empty() {
        "Complete"
    } else {
        "Finished with errors"
    };
    writeln!(
        out,
        "{status}. Commit: `{}`{}. Target: `{}`.\n",
        report.source.commit,
        if report.source.dirty { " (dirty)" } else { "" },
        report.environment.target
    )
    .unwrap();
    if let Some(repository) = &report.source.repository {
        writeln!(out, "Repository: `{}`.\n", repository.display()).unwrap();
    }
    if let Some(collector) = &report.collector {
        writeln!(
            out,
            "Collector: `{}`; source SHA-256: `{}`.\n",
            collector.version, collector.source_sha256
        )
        .unwrap();
    }
    writeln!(
        out,
        "`{}`. Platform: `{}`. [Raw data](results.json).\n",
        report.environment.rustc.lines().next().unwrap_or("unknown"),
        report.environment.platform
    )
    .unwrap();
    out.push_str("| Workload | Profile | Executable | Stripped | Text sections | Debug sections | Stripe rlibs | Stripe rmeta |\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    for row in &report.measurements {
        let columns = sizes(row.sizes.as_ref()).map(optional).join(" | ");
        writeln!(out, "| {} | {} | {columns} |", row.case, row.profile).unwrap();
    }
    if report.measurements.iter().any(|row| row.profile == "dev") {
        out.push_str("\n## Incremental caches\n\n| Workload | Profile | Consumer | Stripe libraries |\n|---|---|---:|---:|\n");
        for row in report.measurements.iter().filter(|row| row.profile == "dev") {
            let columns = incremental_sizes(row.sizes.as_ref()).map(optional).join(" | ");
            writeln!(out, "| {} | {} | {columns} |", row.case, row.profile).unwrap();
        }
    }
    for row in &report.measurements {
        for warning in &row.warnings {
            writeln!(out, "\n- {}/{}: {warning}", row.case, row.profile).unwrap();
        }
    }
    if report.options.timing_runs.is_some() {
        out.push_str("\n## Clean-build timing (noisy)\n\n| Workload | Profile | Samples | Mean | Min–max | Maximum RSS |\n|---|---|---:|---:|---:|---:|\n");
        for row in &report.measurements {
            if let Some(mean) = mean(row) {
                let min =
                    row.timing_samples.iter().map(|s| s.wall_seconds).fold(f64::INFINITY, f64::min);
                let max = row.timing_samples.iter().map(|s| s.wall_seconds).fold(0.0, f64::max);
                let rss = row.timing_samples.iter().filter_map(|s| s.max_rss_bytes).max();
                writeln!(
                    out,
                    "| {} | {} | {} | {mean:.2}s | {min:.2}–{max:.2}s | {} |",
                    row.case,
                    row.profile,
                    row.timing_samples.len(),
                    optional(rss)
                )
                .unwrap();
            }
        }
    }
    if report.options.llvm_lines.is_some() {
        out.push_str("\n## LLVM IR\n\n");
        writeln!(out, "Tool: `{}`.\n", report.options.llvm_lines.as_deref().unwrap()).unwrap();
        out.push_str("| Target | Profile | Scope | Lines | Copies | Raw report |\n|---|---|---|---:|---:|---|\n");
        for row in &report.measurements {
            if let Some(ir) = &row.consumer_ir {
                writeln!(
                    out,
                    "| bench-{} | {} | Consumer root | {} | {} | [IR]({}) |",
                    row.case, row.profile, ir.lines, ir.copies, ir.log
                )
                .unwrap();
            }
        }
        for row in &report.library_ir {
            writeln!(
                out,
                "| {} | {} | Standalone library | {} | {} | [IR]({}) |",
                row.package, row.profile, row.totals.lines, row.totals.copies, row.totals.log
            )
            .unwrap();
        }
    }
    out.push_str("\n## Library artifacts\n");
    for row in &report.measurements {
        let Some(sizes) = &row.sizes else { continue };
        writeln!(out, "\n<details>\n<summary>{}/{}</summary>\n", row.case, row.profile).unwrap();
        out.push_str("| Library | rlib | rmeta | Features |\n|---|---:|---:|---|\n");
        for library in &sizes.libraries {
            writeln!(
                out,
                "| {} | {} | {} | {} |",
                library.package,
                bytes(library.rlib_bytes),
                bytes(library.rmeta_bytes),
                library.features.join(", ")
            )
            .unwrap();
        }
        out.push_str("\n</details>\n");
    }
    if !report.errors.is_empty() {
        out.push_str("\n## Errors\n\n");
        for error in &report.errors {
            writeln!(out, "- {error}").unwrap();
        }
    }
    if report.measurements.iter().any(|row| !row.retained_artifacts.is_empty()) {
        out.push_str("\n## Retained artifacts\n\nPaths are relative to this report directory. Full targets can include incomplete failed builds.\n\n| Workload/profile | Build | Artifact | Path |\n|---|---:|---|---|\n");
        for row in &report.measurements {
            for artifact in &row.retained_artifacts {
                let path = artifact.path.display();
                writeln!(
                    out,
                    "| {}/{} | {} | {} | [{}](<{}>) |",
                    row.case, row.profile, artifact.build, artifact.kind, path, path
                )
                .unwrap();
            }
        }
    }
    if report.collector.is_some() && !report.measurements.is_empty() {
        out.push_str("\nBuild commands and explicit environment overrides are saved as `logs/<workload>-<profile>/build-<N>.command.json`. Inherited build inputs are recorded in `results.json`.\n");
    }
    out
}

fn mean(row: &Measurement) -> Option<f64> {
    (!row.timing_samples.is_empty()).then(|| {
        row.timing_samples.iter().map(|s| s.wall_seconds).sum::<f64>()
            / row.timing_samples.len() as f64
    })
}

pub fn compare_files(baseline: &Path, current: &Path, allow_mismatch: bool) -> Result<String> {
    fn read(path: &Path) -> Result<Report> {
        let path = if path.is_dir() { path.join("results.json") } else { path.to_owned() };
        let value: serde_json::Value = serde_json::from_slice(
            &fs::read(&path).with_context(|| format!("read {}", path.display()))?,
        )?;
        ensure!(
            value["schema_version"] == SCHEMA_VERSION,
            "unsupported report schema in {}",
            path.display()
        );
        serde_json::from_value(value).context("invalid report")
    }
    compare(&read(baseline)?, &read(current)?, allow_mismatch)
}

fn compare(baseline: &Report, current: &Report, allow_mismatch: bool) -> Result<String> {
    let mut notes = Vec::new();
    if baseline.environment != current.environment {
        let before = serde_json::to_value(&baseline.environment)?;
        let after = serde_json::to_value(&current.environment)?;
        let differences = before
            .as_object()
            .context("serialize build environment")?
            .iter()
            .filter(|(key, value)| after.get(key.as_str()) != Some(*value))
            .map(|(key, _)| key.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        ensure!(
            allow_mismatch,
            "build environments differ: {differences}; use matching settings or pass --allow-mismatch"
        );
        notes.push(format!("WARNING: build environments differ ({differences}); these deltas are not a controlled comparison."));
    }
    if baseline.source.lockfile_sha256 != current.source.lockfile_sha256 {
        notes.push(format!(
            "Dependency lockfile changed: `{}` → `{}`.",
            baseline.source.lockfile_sha256, current.source.lockfile_sha256
        ));
    }
    if let (Some(a), Some(b)) = (&baseline.collector, &current.collector)
        && (a.version != b.version || a.source_sha256 != b.source_sha256)
    {
        notes.push(format!(
            "Collector changed: `{}` / `{}` → `{}` / `{}`.",
            a.version, a.source_sha256, b.version, b.source_sha256
        ));
    }
    for (label, report) in [("Baseline", baseline), ("Current", current)] {
        if !report.finished || !report.errors.is_empty() {
            notes.push(format!("WARNING: {label} report is unfinished or contains errors; only available measurements are compared."));
        }
        for row in &report.measurements {
            for warning in &row.warnings {
                notes.push(format!("WARNING: {label} {}/{}: {warning}", row.case, row.profile));
            }
        }
    }
    let before: BTreeMap<_, _> =
        baseline.measurements.iter().map(|m| ((m.case.as_str(), m.profile.as_str()), m)).collect();
    let after: BTreeMap<_, _> =
        current.measurements.iter().map(|m| ((m.case.as_str(), m.profile.as_str()), m)).collect();
    let keys: BTreeSet<_> = before.keys().chain(after.keys()).copied().collect();
    let mut table = String::from(
        "| Workload | Profile | Executable Δ | Stripped Δ | Text Δ | Debug Δ | Stripe rlibs Δ | Stripe rmeta Δ |\n|---|---|---:|---:|---:|---:|---:|---:|\n",
    );
    let mut cache_table = String::new();
    for key in &keys {
        let (Some(a), Some(b)) = (before.get(key), after.get(key)) else {
            notes.push(format!(
                "{}/{}: {} measurement.",
                key.0,
                key.1,
                if before.contains_key(key) { "removed" } else { "added" }
            ));
            continue;
        };
        if let (Some(a), Some(b)) = (&a.sizes, &b.sizes)
            && a.artifact_profile != b.artifact_profile
        {
            ensure!(
                allow_mismatch,
                "Cargo artifact profile fields differ for {}/{}; pass --allow-mismatch to compare",
                key.0,
                key.1
            );
            notes.push(format!(
                "WARNING: {}/{} has different Cargo artifact profile fields.",
                key.0, key.1
            ));
        }
        if baseline.source.workload_sha256.get(key.0) != current.source.workload_sha256.get(key.0) {
            let note = format!(
                "WARNING: workload {} changed; its deltas also reflect consumer-code changes.",
                key.0
            );
            if !notes.contains(&note) {
                notes.push(note);
            }
        }
        if let (Some(a), Some(b)) = (&a.sizes, &b.sizes) {
            let libraries_before: BTreeMap<_, _> =
                a.libraries.iter().map(|l| (l.package.as_str(), l)).collect();
            let libraries_after: BTreeMap<_, _> =
                b.libraries.iter().map(|l| (l.package.as_str(), l)).collect();
            for package in libraries_before
                .keys()
                .chain(libraries_after.keys())
                .copied()
                .collect::<BTreeSet<_>>()
            {
                match (libraries_before.get(package), libraries_after.get(package)) {
                    (Some(a), Some(b)) => {
                        let before: BTreeSet<_> = a.features.iter().collect();
                        let after: BTreeSet<_> = b.features.iter().collect();
                        if before != after {
                            notes.push(format!(
                                "{}/{}: `{package}` features changed: [{}] → [{}].",
                                key.0,
                                key.1,
                                a.features.join(", "),
                                b.features.join(", ")
                            ));
                        }
                    }
                    (Some(_), None) => {
                        notes.push(format!("{}/{}: library `{package}` removed.", key.0, key.1))
                    }
                    (None, Some(_)) => {
                        notes.push(format!("{}/{}: library `{package}` added.", key.0, key.1))
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        let columns = sizes(a.sizes.as_ref())
            .into_iter()
            .zip(sizes(b.sizes.as_ref()))
            .map(|(a, b)| delta(a, b))
            .collect::<Vec<_>>()
            .join(" | ");
        writeln!(table, "| {} | {} | {columns} |", key.0, key.1).unwrap();
        if key.1 == "dev" {
            let columns = incremental_sizes(a.sizes.as_ref())
                .into_iter()
                .zip(incremental_sizes(b.sizes.as_ref()))
                .map(|(a, b)| delta(a, b))
                .collect::<Vec<_>>()
                .join(" | ");
            writeln!(cache_table, "| {} | {} | {columns} |", key.0, key.1).unwrap();
        }
    }
    let mut out = format!(
        "# Compile benchmark comparison\n\nBaseline: `{}`{}. Current: `{}`{}.\n\n",
        baseline.source.commit,
        if baseline.source.dirty { " (dirty)" } else { "" },
        current.source.commit,
        if current.source.dirty { " (dirty)" } else { "" }
    );
    for (label, report) in [("Baseline", baseline), ("Current", current)] {
        if let Some(repository) = &report.source.repository {
            writeln!(out, "{label} repository: `{}`.\n", repository.display()).unwrap();
        }
    }
    for note in notes {
        writeln!(out, "- {note}").unwrap();
    }
    out.push('\n');
    out.push_str(&table);
    if !cache_table.is_empty() {
        out.push_str("\n## Incremental caches\n\n| Workload | Profile | Consumer Δ | Stripe libraries Δ |\n|---|---|---:|---:|\n");
        out.push_str(&cache_table);
    }
    let timed: Vec<_> = keys
        .iter()
        .filter_map(|key| {
            let a = mean(before.get(key)?)?;
            let b = mean(after.get(key)?)?;
            Some((key, a, b))
        })
        .collect();
    if !timed.is_empty() {
        out.push_str("\n## Optional wall-time comparison (noisy)\n\n| Workload | Profile | Baseline mean | Current mean | Δ |\n|---|---|---:|---:|---:|\n");
        for (key, a, b) in timed {
            writeln!(out, "| {} | {} | {a:.2}s | {b:.2}s | {:+.2}s |", key.0, key.1, b - a)
                .unwrap();
        }
    }
    Ok(out)
}

fn delta(before: Option<u64>, after: Option<u64>) -> String {
    let (Some(before), Some(after)) = (before, after) else { return "—".into() };
    let difference = i128::from(after) - i128::from(before);
    if difference == 0 {
        return "0 B (0%)".into();
    }
    let percent = if before == 0 {
        String::new()
    } else {
        format!(" ({:+.2}%)", difference as f64 / before as f64 * 100.0)
    };
    format!("{difference:+} B{percent}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Report {
        Report {
            schema_version: SCHEMA_VERSION,
            started_unix_seconds: 0,
            finished: true,
            collector: None,
            source: Source {
                repository: None,
                commit: "commit".into(),
                dirty: false,
                lockfile_sha256: "lock".into(),
                workload_sha256: BTreeMap::new(),
            },
            environment: Environment {
                rustc: "rustc".into(),
                cargo: "cargo".into(),
                platform: "linux/x86_64".into(),
                target: "target".into(),
                rustflags: vec![],
                build_environment: BTreeMap::new(),
                cargo_configuration: BTreeMap::new(),
                workspace_profiles: serde_json::json!({}),
                policy: "v1".into(),
            },
            options: Options {
                cases: vec![],
                profiles: vec![],
                timing_runs: None,
                llvm_lines: None,
                offline: true,
                keep_artifacts: KeepArtifacts::None,
            },
            measurements: vec![Measurement {
                case: "minimal".into(),
                profile: "dev".into(),
                sizes: None,
                timing_samples: vec![],
                consumer_ir: None,
                warnings: vec![],
                retained_artifacts: vec![],
            }],
            library_ir: vec![],
            errors: vec![],
        }
    }

    #[test]
    fn comparisons_preserve_unavailable_and_zero_baseline_semantics() {
        assert_eq!(delta(None, Some(0)), "—");
        assert_eq!(delta(Some(5), None), "—");
        assert_eq!(delta(Some(0), Some(10)), "+10 B");
        assert_eq!(delta(Some(100), Some(90)), "-10 B (-10.00%)");
        assert_eq!(delta(Some(u64::MAX), Some(0)), "-18446744073709551615 B (-100.00%)");
    }

    #[test]
    fn comparisons_reject_mismatches_and_identify_changed_or_missing_workloads() -> Result<()> {
        let before = sample();
        let mut after = sample();
        after.environment.target = "different".into();
        assert!(
            compare(&before, &after, false)
                .unwrap_err()
                .to_string()
                .contains("build environments differ: target;")
        );
        assert!(compare(&before, &after, true)?.contains("not a controlled comparison"));
        after.environment.target = before.environment.target.clone();
        after.measurements[0].warnings.push("consumer incremental cache unavailable".into());
        assert!(
            compare(&before, &after, false)?
                .contains("WARNING: Current minimal/dev: consumer incremental cache unavailable")
        );
        after.source.workload_sha256.insert("minimal".into(), "new".into());
        assert!(compare(&before, &after, false)?.contains("consumer-code changes"));
        after.measurements[0].case = "webhook".into();
        let output = compare(&before, &after, false)?;
        assert!(output.contains("minimal/dev: removed measurement"));
        assert!(output.contains("webhook/dev: added measurement"));
        Ok(())
    }

    #[test]
    fn report_directories_and_json_files_compare_identically() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let json = directory.path().join("results.json");
        let report = sample();
        fs::write(&json, serde_json::to_vec(&report)?)?;
        assert_eq!(
            compare_files(directory.path(), &json, false)?,
            compare(&report, &report, false)?
        );
        Ok(())
    }

    #[test]
    fn dependency_changes_are_informational_and_features_ignore_order() -> Result<()> {
        fn sizes(features: &[&str]) -> Sizes {
            Sizes {
                executable_bytes: 100,
                stripped_executable_bytes: None,
                sections: None,
                libraries: vec![Library {
                    package: "async-stripe-shared".into(),
                    features: features.iter().map(|s| (*s).into()).collect(),
                    rlib_bytes: 10,
                    rmeta_bytes: 5,
                }],
                consumer_incremental_bytes: None,
                library_incremental_bytes: None,
                artifact_profile: serde_json::from_value(serde_json::json!({
                    "opt_level": "0", "debuginfo": 2, "debug_assertions": true,
                    "overflow_checks": true, "test": false,
                }))
                .unwrap(),
            }
        }
        let mut before = sample();
        let mut after = sample();
        before.measurements[0].sizes = Some(sizes(&["serialize", "deserialize"]));
        after.measurements[0].sizes = Some(sizes(&["deserialize", "serialize"]));
        assert!(!compare(&before, &after, false)?.contains("features changed"));
        after.source.lockfile_sha256 = "new-lock".into();
        after.measurements[0].sizes = Some(sizes(&["serialize"]));
        let output = compare(&before, &after, false)?;
        assert!(output.contains("Dependency lockfile changed"));
        assert!(output.contains("async-stripe-shared` features changed"));
        assert!(!output.contains("WARNING:"));
        after.measurements[0].sizes.as_mut().unwrap().libraries[0].package =
            "async-stripe-webhook".into();
        let output = compare(&before, &after, false)?;
        assert!(output.contains("library `async-stripe-shared` removed"));
        assert!(output.contains("library `async-stripe-webhook` added"));
        Ok(())
    }

    #[test]
    fn old_reports_without_provenance_or_retention_still_compare() -> Result<()> {
        let mut old = serde_json::to_value(sample())?;
        old.as_object_mut().unwrap().remove("collector");
        old["source"].as_object_mut().unwrap().remove("repository");
        old["options"].as_object_mut().unwrap().remove("keep_artifacts");
        old["measurements"][0].as_object_mut().unwrap().remove("retained_artifacts");
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("old.json");
        fs::write(&path, serde_json::to_vec(&old)?)?;
        assert!(compare_files(&path, &path, false)?.contains("Compile benchmark comparison"));
        Ok(())
    }
}
