use std::fmt::Write as _;

use heck::SnakeCase;
use indoc::writedoc;

use crate::rust_object::{EnumVariant, RustEnum};
use crate::templates::derives::{write_derives_line, Derives};
use crate::templates::utils::write_serde_rename;
use crate::types::RustIdent;

impl RustEnum {
    /// Generate the enum definition, along with the methods `as_str`, `as_ref`, `impl Display`,
    /// and `impl Default` is a default is specified
    pub fn write_definition_and_methods(
        &self,
        out: &mut String,
        enum_name: &RustIdent,
        additional_derives: Derives,
    ) {
        // Build the body of the enum definition
        let mut enum_def_body = String::with_capacity(128);
        for EnumVariant { wire_name, variant_name } in &self.variants {
            if &variant_name.to_snake_case() != wire_name {
                write_serde_rename(&mut enum_def_body, wire_name);
            }
            let _ = writeln!(enum_def_body, "{variant_name},");
        }

        // Build the body of the `as_str` implementation
        let mut as_str_body = String::with_capacity(32);
        for EnumVariant { wire_name, variant_name } in &self.variants {
            let _ = writeln!(as_str_body, r#"Self::{variant_name} => "{wire_name}","#);
        }

        let derives = additional_derives.copy(true).eq(true);
        let derives = write_derives_line(derives);
        let _ = writedoc!(
            out,
            r#"
            {derives}
            #[serde(rename_all = "snake_case")]
            pub enum {enum_name} {{
            {enum_def_body}
            }}

            impl {enum_name} {{
                pub fn as_str(self) -> &'static str {{
                    match self {{
            {as_str_body}
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
                    self.as_str().fmt(f)
                }}
            }}
        "#
        );
    }
}
