use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::types::{ComponentPath, RustIdent};
use crate::STRIPE_TYPES;

/// The kind of prefix an id is required to follow
#[derive(Deserialize)]
#[serde(untagged)]
enum IdPrefix {
    /// Must start with given prefix
    Single(String),
    /// Must start with any of the given prefixes
    Multi(Vec<String>),
}

fn load_id_prefixes() -> anyhow::Result<HashMap<String, IdPrefix>> {
    let prefixes = serde_json::from_reader(File::open("id_prefixes.json")?)?;
    Ok(prefixes)
}

lazy_static! {
    /// Map of component_path to what the prefix for this id should be.
    static ref ID_PREFIXES: HashMap<String, IdPrefix> =
        load_id_prefixes().expect("Invalid id prefix file");
}

pub fn write_object_id(out: &mut String, path: &ComponentPath, ident: &RustIdent) {
    match ID_PREFIXES.get(path.as_ref()) {
        Some(IdPrefix::Single(prefix)) => {
            let _ = writeln!(out, r#"{STRIPE_TYPES}::def_id!({ident}, "{prefix}_");"#);
        }
        Some(IdPrefix::Multi(prefixes)) => {
            let prefix_arg =
                prefixes.iter().map(|p| format!(r#""{p}_""#)).collect::<Vec<_>>().join("|");
            let _ = writeln!(out, "{STRIPE_TYPES}::def_id!({ident}, {prefix_arg});");
        }
        None => {
            let _ = writeln!(out, "{STRIPE_TYPES}::def_id!({ident});");
        }
    }
}
