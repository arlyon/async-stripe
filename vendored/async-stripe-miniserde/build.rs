// Kept as a build script only to avoid disturbing existing `include!`
// machinery in src/lib.rs. The version-suffixed `__private` dance from
// upstream miniserde existed so derive-emitted token streams could
// disambiguate multiple miniserde versions in one binary; we removed
// the derive crate, so the suffix is unnecessary.
use std::env;
use std::fs;
use std::path::PathBuf;

const PRIVATE: &str = "\
#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use crate::export::*;
}
";

fn main() {
    println!("cargo:rerun-if-changed=src/place.rs");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    fs::write(out_dir.join("private.rs"), PRIVATE).unwrap();

    let mod_place = fs::read_to_string(manifest_dir.join("src").join("place.rs")).unwrap();
    fs::write(out_dir.join("place.rs"), mod_place).unwrap();
}
