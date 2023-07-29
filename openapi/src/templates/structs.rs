use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::object_context::{Lifetime, PrintableWithLifetime};
use crate::rust_object::{PrintableStructField, Visibility};
use crate::templates::derives::{write_derives_line, Derives};
use crate::templates::utils::{write_doc_comment, write_serde_rename};
use crate::types::RustIdent;

pub fn write_struct(
    out: &mut String,
    name: &RustIdent,
    additional_derives: Derives,
    fields: &[PrintableStructField],
    include_constructor: bool,
    opt_lifetime: Option<Lifetime>,
) {
    let derives = write_derives_line(additional_derives);
    let lifetime_str = opt_lifetime.map(|l| format!("<{l}>")).unwrap_or_default();

    let mut fields_str = String::with_capacity(64);
    for field in fields {
        field.write_code(&mut fields_str, opt_lifetime);
    }
    let _ = writedoc!(
        out,
        r"
        {derives}
        pub struct {name}{lifetime_str} {{
        {fields_str}
        }}
    "
    );

    if include_constructor {
        let cons_body = if additional_derives.derives_default() {
            r"
            pub fn new() -> Self {
                Self::default()
            }
            "
            .into()
        } else {
            let mut cons_inner = String::new();
            let mut params = String::new();
            for field in fields {
                if field.required {
                    let typ = PrintableWithLifetime::new(&field.rust_type, opt_lifetime);
                    let _ = write!(params, "{}: {typ},", field.field_name);
                    let _ = write!(cons_inner, "{},", field.field_name);
                } else {
                    let _ = write!(cons_inner, "{}: Default::default(),", field.field_name);
                }
            }
            formatdoc! {
                r"
                pub fn new({params}) -> Self {{
                    Self {{
                        {cons_inner}
                    }}
                }} 
                "
            }
        };
        let _ = writedoc!(
            out,
            r"
            impl{lifetime_str} {name}{lifetime_str} {{
                {cons_body}
            }}
                "
        );
    }
}

impl<'a> PrintableStructField<'a> {
    fn write_code(&self, out: &mut String, lifetime: Option<Lifetime>) {
        if let Some(doc_comment) = &self.doc_comment {
            let _ = writeln!(out, "{}", write_doc_comment(doc_comment, 1).trim_end());
        }
        if let Some(renamer) = &self.rename_as {
            write_serde_rename(out, renamer);
        }
        if let Some(skip_serializing) = self.skip_serializing {
            let _ = writeln!(out, r#"#[serde(skip_serializing_if = "{skip_serializing}")]"#);
        }

        if let Some(serde_default) = self.deser_default {
            let _ = writeln!(out, "{}", serde_default.to_serde_attr());
        }

        let typ = PrintableWithLifetime::new(&self.rust_type, lifetime);
        let vis = match self.vis {
            Visibility::Public => "pub ",
            Visibility::Private => "",
        };
        let _ = writeln!(out, "{vis}{}: {typ},", self.field_name);
    }
}
