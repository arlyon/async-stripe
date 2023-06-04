use std::fmt::Write as _;

use indoc::writedoc;

use crate::object_context::{Lifetime, PrintableWithLifetime};
use crate::rust_object::PrintableFieldedEnumVariant;
use crate::templates::derives::{write_derives_line, Derives};
use crate::types::RustIdent;

pub fn write_fielded_enum(
    out: &mut String,
    enum_name: &RustIdent,
    variants: &[PrintableFieldedEnumVariant],
    additional_derives: Derives,
    lifetime: Option<Lifetime>,
) {
    let lifetime_str = lifetime.map(|l| format!("<{l}>")).unwrap_or_default();
    // Build the body of the enum definition
    let mut enum_body = String::with_capacity(64);
    for PrintableFieldedEnumVariant { variant, rust_type } in variants {
        if let Some(typ) = rust_type {
            let typ = PrintableWithLifetime::new(typ, lifetime);
            let _ = writeln!(enum_body, "{variant}({typ}),");
        } else {
            let _ = writeln!(enum_body, "{variant},");
        }
    }
    let derives = write_derives_line(additional_derives);
    let _ = writedoc!(
        out,
        r#"
            {derives}
            #[serde(untagged, rename_all = "snake_case")]
            pub enum {enum_name}{lifetime_str} {{
            {enum_body}
            }}
        "#
    );
}
