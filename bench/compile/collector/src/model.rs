use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const CASES: &[&str] = &["minimal", "poly", "deser-many", "requests", "webhook"];
pub const SCHEMA_VERSION: u32 = 1;
// Keep the stored warning compatible with reports collected before this notice
// was consolidated in human-readable output.
pub const RSS_UNAVAILABLE_WARNING: &str =
    "Maximum RSS unavailable on this host; wall time is still collected.";
pub const RSS_UNAVAILABLE_NOTICE: &str = "Maximum RSS unavailable; wall time is still collected.";

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub schema_version: u32,
    pub started_unix_seconds: u64,
    pub finished: bool,
    #[serde(default)]
    pub collector: Option<Collector>,
    pub source: Source,
    pub environment: Environment,
    pub options: Options,
    pub measurements: Vec<Measurement>,
    pub library_ir: Vec<LibraryIr>,
    pub errors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Collector {
    pub version: String,
    pub source_sha256: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    #[serde(default)]
    pub repository: Option<PathBuf>,
    pub commit: String,
    pub dirty: bool,
    pub lockfile_sha256: String,
    pub workload_sha256: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Environment {
    pub rustc: String,
    pub cargo: String,
    pub platform: String,
    pub target: String,
    pub rustflags: Vec<String>,
    pub build_environment: BTreeMap<String, String>,
    pub cargo_configuration: BTreeMap<String, String>,
    pub workspace_profiles: serde_json::Value,
    pub policy: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub cases: Vec<String>,
    pub profiles: Vec<String>,
    pub timing_runs: Option<usize>,
    pub llvm_lines: Option<String>,
    pub offline: bool,
    #[serde(default)]
    pub keep_artifacts: KeepArtifacts,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum KeepArtifacts {
    #[default]
    None,
    Executable,
    Target,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Measurement {
    pub case: String,
    pub profile: String,
    pub sizes: Option<Sizes>,
    pub timing_samples: Vec<TimingSample>,
    pub consumer_ir: Option<Ir>,
    pub warnings: Vec<String>,
    #[serde(default)]
    pub retained_artifacts: Vec<RetainedArtifact>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RetainedArtifact {
    pub build: usize,
    pub kind: String,
    /// Relative to the report directory, so a retained report can be moved.
    pub path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sizes {
    pub executable_bytes: u64,
    pub stripped_executable_bytes: Option<u64>,
    pub sections: Option<Vec<Section>>,
    pub libraries: Vec<Library>,
    #[serde(default)]
    pub consumer_incremental_bytes: Option<u64>,
    #[serde(default)]
    pub library_incremental_bytes: Option<u64>,
    pub artifact_profile: cargo_metadata::ArtifactProfile,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub name: String,
    pub segment: Option<String>,
    pub kind: String,
    pub file_bytes: u64,
    pub memory_bytes: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    pub package: String,
    pub features: Vec<String>,
    pub rlib_bytes: u64,
    pub rmeta_bytes: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimingSample {
    pub wall_seconds: f64,
    pub max_rss_bytes: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ir {
    pub lines: u64,
    pub copies: u64,
    pub log: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryIr {
    pub package: String,
    pub profile: String,
    pub totals: Ir,
}
