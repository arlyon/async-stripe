use std::fmt::Write as _;

use indexmap::IndexSet;
use indoc::writedoc;

use crate::printable::{Lifetime, PrintableEnumVariant, PrintableWithLifetime};
use crate::rust_object::FieldlessVariant;
use crate::templates::derives::{write_derives_line, Derives};
use crate::templates::utils::write_serde_rename;
use crate::types::RustIdent;

pub fn write_enum_variants(
    out: &mut String,
    enum_name: &RustIdent,
    variants: &[PrintableEnumVariant],
    additional_derives: Derives,
    lifetime: Option<Lifetime>,
    deser_names: Option<IndexSet<&str>>,
) {
    let lifetime_str = lifetime.map(|l| format!("<{l}>")).unwrap_or_default();

    // Build the body of the enum definition
    let mut enum_body = String::with_capacity(64);
    for (ind, PrintableEnumVariant { variant, rust_type, feature_gate }) in
        variants.iter().enumerate()
    {
        if let Some(feature_gate) = feature_gate {
            let _ = writeln!(enum_body, r#"#[cfg(feature = "{feature_gate}")]"#);
        }
        if let Some(typ) = rust_type {
            if let Some(names) = &deser_names {
                let rename_to = names.get_index(ind).unwrap();
                write_serde_rename(&mut enum_body, rename_to);
            }
            let typ = PrintableWithLifetime::new(typ, lifetime);
            let _ = writeln!(enum_body, "{variant}({typ}),");
        } else {
            let _ = writeln!(enum_body, "{variant},");
        }
    }
    let derives = write_derives_line(additional_derives);
    let serde_tag_line = if deser_names.is_some() { r#"tag = "object""# } else { "untagged" };
    let _ = writedoc!(
        out,
        r#"
            {derives}
            #[serde({serde_tag_line})]
            pub enum {enum_name}{lifetime_str} {{
            {enum_body}
            }}
        "#
    );
}

const ADD_UNKNOWN_VARIANT_THRESHOLD: usize = 12;

/// Generate the enum definition, along with the methods `as_str`, `as_ref`, `impl Display`,
/// and `impl Default`.
pub fn write_fieldless_enum_variants(
    out: &mut String,
    variants: &[FieldlessVariant],
    enum_name: &RustIdent,
    additional_derives: Derives,
) {
    let provide_unknown_variant = variants.len() > ADD_UNKNOWN_VARIANT_THRESHOLD
        && !variants.iter().any(|v| v.variant_name.as_ref() == "Unknown");
    let nonexhaustive = if provide_unknown_variant { "#[non_exhaustive]" } else { "" };
    let derive_deser = additional_derives.derives_deserialize();
    let derive_serialize = additional_derives.derives_serialize();

    // Build the body of the enum definition
    let mut enum_def_body = String::with_capacity(128);
    for FieldlessVariant { variant_name, .. } in variants {
        let _ = writeln!(enum_def_body, "{variant_name},");
    }
    if provide_unknown_variant {
        let _ = writedoc!(
            enum_def_body,
            r"
            /// An unrecognized value from Stripe. Should not be used as a request parameter.
            Unknown,
        "
        );
    }

    // Build the body of the `as_str` implementation
    let mut as_str_body = String::with_capacity(32);
    for FieldlessVariant { wire_name, variant_name } in variants {
        let _ = writeln!(as_str_body, r#"{variant_name} => "{wire_name}","#);
    }
    if provide_unknown_variant {
        let _ = writeln!(as_str_body, r#"Unknown => "unknown","#);
    }

    // Build the body of the `from_str` implementation
    let mut from_str_body = String::with_capacity(32);
    for FieldlessVariant { wire_name, variant_name } in variants {
        let _ = writeln!(from_str_body, r#""{wire_name}" => Ok({variant_name}),"#);
    }
    let _ = writeln!(from_str_body, "_ => Err(())");

    // NB: we unset the derive flags for serialize + deserialize + debug to avoid duplicating
    // the (potentially many) strings in `as_str` and `from_str` through the default derive.
    // These derived implementations often show up running `llvm-lines`, so easy
    // binary size + compile time win by doing this.
    let derives =
        additional_derives.copy(true).eq(true).serialize(false).deserialize(false).debug(false);

    let derives = write_derives_line(derives);
    let _ = writedoc!(
        out,
        r#"
            {derives}{nonexhaustive}
            pub enum {enum_name} {{
            {enum_def_body}
            }}

            impl {enum_name} {{
                pub fn as_str(self) -> &'static str {{
                    use {enum_name}::*;
                    match self {{
            {as_str_body}
                    }}
                }}
            }}
            
            impl std::str::FromStr for {enum_name} {{
                type Err = ();
                fn from_str(s: &str) -> Result<Self, Self::Err> {{
                    use {enum_name}::*;
                    match s {{
                {from_str_body}
                    }}
                }}
            }}

            impl AsRef<str> for {enum_name} {{
                fn as_ref(&self) -> &str {{
                    self.as_str()
                }}
            }}

            impl std::fmt::Display for {enum_name} {{
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                    f.write_str(self.as_str())
                }}
            }}
            
            impl std::fmt::Debug for {enum_name} {{
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                    f.write_str(self.as_str())
                }}
            }}
        "#
    );

    if derive_serialize {
        let _ = writedoc!(
            out,
            r#"
            impl serde::Serialize for {enum_name} {{
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{
                    serializer.serialize_str(self.as_str())
                }}
            }}
            "#
        );
    }

    if derive_deser {
        let ret_line = if provide_unknown_variant {
            format!("Ok(Self::from_str(&s).unwrap_or({enum_name}::Unknown))")
        } else {
            format!(
                r#"Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for {enum_name}"))"#
            )
        };
        let _ = writedoc!(
            out,
            r#"
            impl<'de> serde::Deserialize<'de> for {enum_name} {{
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
                    use std::str::FromStr;
                    let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
                    {ret_line}
                }}
            }}
            "#
        );
    }
}
