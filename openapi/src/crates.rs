use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::anyhow;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Crate {
    base_name: &'static str,
}

impl Crate {
    pub const SHARED: Self = Self { base_name: "shared" };

    pub fn generated_out_path(self) -> String {
        format!("crates/{}", self.crate_name())
    }

    pub fn base_name(self) -> &'static str {
        self.base_name
    }

    pub fn lib_name(self) -> String {
        format!("stripe_{}", self.base_name)
    }

    pub fn crate_name(self) -> String {
        format!("async-stripe-{}", self.base_name)
    }

    fn generate_to(self) -> String {
        let out_path = self.generated_out_path();
        format!("{out_path}/src")
    }

    pub fn get_path(self) -> PathBuf {
        PathBuf::from(self.generate_to())
    }

    pub fn get_path_to_mod(self) -> PathBuf {
        self.get_path().join("mod.rs")
    }
}

/// Information about an automatically generated crate.
#[derive(Deserialize)]
pub struct CrateGen {
    /// Name of the crate
    pub name: String,
    /// Component paths which should live in this crate
    #[serde(default)]
    pub paths: Vec<String>,
    /// Package names which should live in this crate
    #[serde(default)]
    pub packages: Vec<String>,
    /// Used to generate a top-level comment for the crate.
    pub description: String,
}

#[derive(Deserialize)]
pub struct CrateInfo {
    pub crates: Vec<CrateGen>,
}

fn load_crate_info() -> anyhow::Result<CrateInfo> {
    let toml_str = read_to_string("gen_crates.toml")?;
    let loaded = toml::from_str(&toml_str)?;
    Ok(loaded)
}

lazy_static! {
    /// Content of `gen_crates.toml`
    pub static ref CRATE_INFO: CrateInfo = load_crate_info().expect("Could not load crate info");
    /// All crate names
    pub static ref ALL_CRATES: Vec<Crate> =
        CRATE_INFO.crates.iter().map(|c| Crate {base_name: &c.name}).collect();
}

#[track_caller]
fn crate_info_unwrapped(krate: Crate) -> &'static CrateGen {
    CRATE_INFO.crates.iter().find(|k| k.name == krate.base_name).expect("Crate not found")
}

pub fn get_crate_doc_comment(krate: Crate) -> &'static str {
    &crate_info_unwrapped(krate).description
}

/// Assign the crate a package should live in. Each package is expected to be
/// specified in `gen_crates.toml`, so the assignment is hardcoded.
#[tracing::instrument]
pub fn get_crate_for_package(package: &str) -> anyhow::Result<Crate> {
    for krate in &CRATE_INFO.crates {
        if krate.packages.iter().any(|p| p == package) {
            return Ok(Crate { base_name: &krate.name });
        }
    }
    Err(anyhow!("no crate mapping found for package {package}. please add it to gen_crates.toml"))
}

/// Use a hardcoded assignment if available to determine the crate we should assign
/// to this component.
#[tracing::instrument]
pub fn maybe_use_hardcoded_crate_assignment(path: &str) -> Option<Crate> {
    // Make sure deleted variants end up in the same place as the non-deleted version
    let path = path.trim_start_matches("deleted_");
    for krate in &CRATE_INFO.crates {
        if krate.paths.iter().any(|p| p == path) {
            return Some(Crate { base_name: &krate.name });
        }
    }
    // No mapping here is expected in most cases since can infer crates using
    // dependency graphs
    tracing::trace!("no crate mapping found for path {path}");
    None
}
