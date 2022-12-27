use std::fmt::Write as _;

use indoc::formatdoc;

pub struct CrateImports<'a> {
    pub path: &'static str,
    pub imports: Vec<&'a str>,
}

pub fn write_prelude(imports: Vec<CrateImports>) -> String {
    let mut import_lines = String::new();
    for import_spec in imports {
        let path = import_spec.path;
        if !import_spec.imports.is_empty() {
            let imports = import_spec.imports.join(", ");
            writeln!(import_lines, "use crate::{path}::{{{imports}}};").unwrap();
        }
    }
    formatdoc!(
        r"
        // ======================================
        // This file was automatically generated.
        // ======================================
        
        {import_lines}
        use serde::{{Deserialize, Serialize}};
    
    "
    )
}
