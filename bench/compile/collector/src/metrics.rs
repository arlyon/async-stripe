use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail, ensure};
use cargo_metadata::{Artifact, Message, Package, PackageId, TargetKind};
use object::{Object, ObjectSection};

use crate::model::{Ir, Library, Section, Sizes};

fn artifacts(log: &Path) -> Result<Vec<Artifact>> {
    let mut artifacts = Vec::new();
    for message in Message::parse_stream(BufReader::new(fs::File::open(log)?)) {
        if let Message::CompilerArtifact(artifact) = message? {
            artifacts.push(artifact);
        }
    }
    Ok(artifacts)
}

fn root_artifact<'a>(
    artifacts: &'a [Artifact],
    packages: &[Package],
    name: &str,
) -> Result<&'a Artifact> {
    let package =
        packages.iter().find(|p| p.name.as_ref() == name).context("fixture package missing")?;
    artifacts
        .iter()
        .find(|a| {
            a.package_id == package.id
                && a.target.name == name
                && a.target.kind.contains(&TargetKind::Bin)
                && a.executable.is_some()
        })
        .context("Cargo did not report the fixture executable")
}

pub fn executable(log: &Path, packages: &[Package], name: &str) -> Result<std::path::PathBuf> {
    Ok(root_artifact(&artifacts(log)?, packages, name)?
        .executable
        .as_ref()
        .unwrap()
        .as_std_path()
        .to_owned())
}

pub fn collect(
    log: &Path,
    packages: &[Package],
    name: &str,
    target_dir: &Path,
    incremental: bool,
    warnings: &mut Vec<String>,
) -> Result<Sizes> {
    let artifacts = artifacts(log)?;
    let root = root_artifact(&artifacts, packages, name)?;
    let executable = root.executable.as_ref().unwrap().as_std_path();
    let sections = match read_sections(executable) {
        Ok(sections) => Some(sections),
        Err(error) => {
            warnings.push(format!("executable sections unavailable: {error:#}"));
            None
        }
    };
    let stripped_executable_bytes = match stripped_size(executable, target_dir) {
        Ok(bytes) => Some(bytes),
        Err(error) => {
            warnings.push(format!("stripped executable size unavailable: {error:#}"));
            None
        }
    };
    let library_names = packages
        .iter()
        .filter(|p| p.name.starts_with("async-stripe"))
        .map(|p| (p.id.clone(), p.name.to_string()))
        .collect();
    let mut sizes = Sizes {
        executable_bytes: fs::metadata(executable)?.len(),
        stripped_executable_bytes,
        sections,
        libraries: libraries(&artifacts, &library_names)?,
        consumer_incremental_bytes: None,
        library_incremental_bytes: None,
        artifact_profile: root.profile.clone(),
    };
    if incremental {
        let directory =
            executable.parent().context("executable has no parent directory")?.join("incremental");
        let library_crates = artifacts
            .iter()
            .filter(|a| {
                library_names.contains_key(&a.package_id)
                    && a.target.kind.iter().any(|k| matches!(k, TargetKind::Lib | TargetKind::RLib))
            })
            .map(|a| a.target.name.replace('-', "_"))
            .collect();
        for (label, crates, bytes) in [
            (
                "consumer",
                BTreeSet::from([root.target.name.replace('-', "_")]),
                &mut sizes.consumer_incremental_bytes,
            ),
            ("Stripe library", library_crates, &mut sizes.library_incremental_bytes),
        ] {
            match incremental_bytes(&directory, &crates) {
                Ok(total) => *bytes = Some(total),
                Err(error) => {
                    warnings.push(format!("{label} incremental cache unavailable: {error:#}"))
                }
            }
        }
    }
    Ok(sizes)
}

fn incremental_bytes(directory: &Path, crates: &BTreeSet<String>) -> Result<u64> {
    if crates.is_empty() {
        return Ok(0);
    }
    let mut missing = crates.clone();
    let mut total = 0;
    for entry in fs::read_dir(directory).with_context(|| format!("read {}", directory.display()))? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name();
        // rustc uses <crate_name>-<id>; Cargo target names supply the crate name.
        // https://github.com/rust-lang/rust/blob/1.89.0/compiler/rustc_incremental/src/persist/fs.rs#L552-L564
        let Some((name, _)) = name.to_str().and_then(|name| name.split_once('-')) else {
            continue;
        };
        if crates.contains(name) {
            total += directory_bytes(&entry.path())?;
            missing.remove(name);
        }
    }
    ensure!(
        missing.is_empty(),
        "no incremental cache found for {}",
        missing.into_iter().collect::<Vec<_>>().join(", ")
    );
    Ok(total)
}

fn directory_bytes(directory: &Path) -> Result<u64> {
    let mut total = 0;
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_dir() {
            total += directory_bytes(&entry.path())?;
        } else if kind.is_file() {
            total += entry.metadata()?.len();
        }
    }
    Ok(total)
}

fn libraries(artifacts: &[Artifact], names: &BTreeMap<PackageId, String>) -> Result<Vec<Library>> {
    let mut libraries: BTreeMap<&str, Library> = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for artifact in artifacts {
        let Some(name) = names.get(&artifact.package_id) else { continue };
        if !artifact.target.kind.iter().any(|k| matches!(k, TargetKind::Lib | TargetKind::RLib)) {
            continue;
        }
        for path in &artifact.filenames {
            let extension = path.extension().unwrap_or("");
            if !matches!(extension, "rlib" | "rmeta") || !seen.insert(path) {
                continue;
            }
            let library = libraries.entry(name).or_insert_with(|| Library {
                package: name.clone(),
                features: artifact.features.clone(),
                rlib_bytes: 0,
                rmeta_bytes: 0,
            });
            let bytes = fs::metadata(path).with_context(|| format!("stat {path}"))?.len();
            if extension == "rlib" {
                library.rlib_bytes += bytes;
            } else {
                library.rmeta_bytes += bytes;
            }
        }
    }
    Ok(libraries.into_values().collect())
}

fn stripped_size(executable: &Path, target_dir: &Path) -> Result<u64> {
    // Preserve the original for all other measurements and diagnostics.
    let copy = target_dir.join("stripped-executable");
    fs::copy(executable, &copy)?;
    let output = Command::new("strip").arg(&copy).output().context("run strip")?;
    if !output.status.success() {
        bail!("strip: {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    Ok(fs::metadata(copy)?.len())
}

fn read_sections(path: &Path) -> Result<Vec<Section>> {
    let bytes = fs::read(path)?;
    let object = object::File::parse(&*bytes)?;
    object
        .sections()
        .map(|section| {
            Ok(Section {
                name: section.name()?.to_owned(),
                segment: section.segment_name()?.map(str::to_owned),
                kind: format!("{:?}", section.kind()),
                // size() is a memory size; file_range excludes zero-fill sections.
                file_bytes: section.file_range().map_or(0, |(_, size)| size),
                memory_bytes: section.size(),
            })
        })
        .collect()
}

/// Physical section bytes, never virtual address ranges or Mach-O's __PAGEZERO.
pub fn section_bytes(sections: &[Section], kinds: &[&str]) -> u64 {
    sections.iter().filter(|s| kinds.contains(&s.kind.as_str())).map(|s| s.file_bytes).sum()
}

pub fn debug_bytes(sections: &[Section]) -> u64 {
    sections
        .iter()
        // ELF's SectionKind does not identify DWARF. Count its standard section
        // names, including compressed .zdebug_ sections, using physical bytes.
        .filter(|s| {
            s.name.starts_with(".debug_")
                || s.name.starts_with(".zdebug_")
                || matches!(s.kind.as_str(), "Debug" | "DebugString")
        })
        .map(|s| s.file_bytes)
        .sum()
}

pub fn ir_totals(text: &str, log: String) -> Result<Ir> {
    for line in text.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.get(2) == Some(&"(TOTAL)") {
            return Ok(Ir { lines: fields[0].parse()?, copies: fields[1].parse()?, log });
        }
    }
    bail!("cargo llvm-lines output has no TOTAL row")
}

pub fn max_rss(text: &str, os: &str) -> Option<u64> {
    match os {
        "macos" => text.lines().find_map(|line| {
            let (bytes, rest) = line.trim().split_once(char::is_whitespace)?;
            (rest.trim() == "maximum resident set size").then(|| bytes.parse().ok()).flatten()
        }),
        "linux" => text.lines().find_map(|line| line.trim().parse::<u64>().ok()?.checked_mul(1024)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incremental_caches_match_exact_crate_names_and_require_all_expected_caches() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let directory = temp.path().join("incremental");
        for (file, bytes) in [
            ("bench_minimal-first/session/query-cache.bin", 11),
            ("bench_minimal-first/session/nested/object.o", 13),
            ("bench_minimal-second/session/object.o", 7),
            ("stripe_shared-id/session/dep-graph.bin", 17),
            ("stripe_types-id/session/dep-graph.bin", 19),
            ("bench_minimal_extra-id/session/object.o", 101),
            ("build_script_build-id/session/object.o", 103),
        ] {
            let file = directory.join(file);
            fs::create_dir_all(file.parent().unwrap())?;
            fs::write(file, vec![0; bytes])?;
        }
        #[cfg(unix)]
        std::os::unix::fs::symlink(
            directory.join("stripe_shared-id"),
            directory.join("bench_minimal-first/external-cache"),
        )?;
        let consumer = BTreeSet::from(["bench_minimal".into()]);
        let libraries = BTreeSet::from(["stripe_shared".into(), "stripe_types".into()]);
        assert_eq!(incremental_bytes(&directory, &consumer)?, 31);
        assert_eq!(incremental_bytes(&directory, &libraries)?, 36);
        let missing = BTreeSet::from(["stripe_shared".into(), "stripe_missing".into()]);
        assert!(incremental_bytes(&directory, &missing).is_err());
        assert!(incremental_bytes(&temp.path().join("missing"), &consumer).is_err());
        assert_eq!(incremental_bytes(&directory, &BTreeSet::new())?, 0);
        Ok(())
    }

    #[test]
    fn only_current_build_first_party_libraries_are_counted_once() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let rlib = dir.path().join("libstripe.rlib");
        let rmeta = dir.path().join("libstripe.rmeta");
        fs::write(&rlib, [0; 7])?;
        fs::write(&rmeta, [0; 3])?;
        fs::write(dir.path().join("stale.rlib"), [0; 99])?;
        let artifact = |id: &str, kind: &str| -> Artifact {
            serde_json::from_value(serde_json::json!({
                "package_id": id,
                "target": { "name": "stripe", "kind": [kind], "crate_types": ["lib"],
                            "src_path": "src/lib.rs", "edition": "2024", "doc": true,
                            "doctest": true, "test": true },
                "filenames": [rlib, rmeta],
                "executable": null,
                "features": ["customer"],
                "profile": { "opt_level": "0", "debuginfo": 2, "debug_assertions": true,
                             "overflow_checks": true, "test": false },
                "fresh": false
            }))
            .unwrap()
        };
        let names = BTreeMap::from([(
            serde_json::from_value(serde_json::json!("local"))?,
            "async-stripe-core".into(),
        )]);
        let artifacts = vec![
            artifact("local", "proc-macro"),
            artifact("registry", "lib"),
            artifact("local", "lib"),
            artifact("local", "lib"),
        ];
        let libraries = libraries(&artifacts, &names)?;
        assert_eq!(libraries.len(), 1);
        assert_eq!(libraries[0].rlib_bytes, 7);
        assert_eq!(libraries[0].rmeta_bytes, 3);
        assert_eq!(libraries[0].features, ["customer"]);
        Ok(())
    }

    #[test]
    fn diagnostics_require_real_totals_and_convert_platform_rss_units() -> Result<()> {
        let ir = ir_totals(" Lines Copies Function name\n 1234 56 (TOTAL)\n", "ir.txt".into())?;
        assert_eq!((ir.lines, ir.copies), (1234, 56));
        assert!(ir_totals("1234 56 some_function", "ir.txt".into()).is_err());
        assert_eq!(max_rss("  4096 maximum resident set size\n", "macos"), Some(4096));
        assert_eq!(
            max_rss("Command exited with non-zero status 1\n4096\n", "linux"),
            Some(4194304)
        );
        assert_eq!(max_rss("sysctl: Operation not permitted", "macos"), None);
        Ok(())
    }
}
