use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::printable::PrintableWithLifetime;
use crate::rust_object::{StructField, Visibility};
use crate::templates::utils::{write_doc_comment, write_serde_rename};
use crate::templates::ObjectWriter;

impl<'a> ObjectWriter<'a> {
    pub fn write_struct(
        &self,
        out: &mut String,
        fields: &[StructField],
        include_constructor: bool,
    ) {
        let name = self.ident;

        let mut fields_str = String::with_capacity(64);
        for field in fields {
            self.write_struct_field(&mut fields_str, field);
        }

        let lifetime_str = self.lifetime_param();
        self.write_derives_line(out);
        let _ = writedoc!(
            out,
            r"
        pub struct {name}{lifetime_str} {{
        {fields_str}
        }}
    "
        );

        if include_constructor {
            let cons_body = if self.derives.default {
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
                        let printable = self.get_printable(&field.rust_type);
                        let typ = PrintableWithLifetime::new(&printable, self.lifetime);
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

    fn write_struct_field(&self, out: &mut String, field: &StructField) {
        if let Some(doc_comment) = &field.doc_comment {
            let _ = writeln!(out, "{}", write_doc_comment(doc_comment, 1).trim_end());
        }
        if let Some(renamer) = &field.rename_as {
            write_serde_rename(out, renamer);
        }
        if !field.required {
            if let Some(skip_ser) = field.rust_type.as_skip_serializing() {
                let _ = writeln!(out, r#"#[serde(skip_serializing_if = "{skip_ser}")]"#);
            }
            if let Some(default) = field.rust_type.as_deser_default() {
                let _ = writeln!(out, "{}", default.to_serde_attr());
            }
        }

        let printable = self.get_printable(&field.rust_type);
        let typ = PrintableWithLifetime::new(&printable, self.lifetime);
        let vis = match field.vis {
            Visibility::Public => "pub ",
            Visibility::Private => "",
        };
        let _ = writeln!(out, "{vis}{}: {typ},", field.field_name);
    }
}
