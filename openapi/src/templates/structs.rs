use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::printable::PrintableWithLifetime;
use crate::rust_object::{ObjectKind, Struct, StructField, Visibility};
use crate::rust_type::{ExtType, RustType, SimpleType};
use crate::templates::object_writer::write_derives_line;
use crate::templates::utils::{write_doc_comment, SerdeDeriveState, ShouldDerive};
use crate::templates::ObjectWriter;

impl<'a> ObjectWriter<'a> {
    pub fn write_struct(&self, out: &mut String, struct_: &Struct) {
        let mut serde_derive = SerdeDeriveState::default();
        // In theory this could be supported, but it would require not borrowing in request structs
        if !matches!(self.usage.kind, ObjectKind::RequestParam) {
            serde_derive.deserialize(ShouldDerive::Gated);
        }
        if self.usage.should_impl_serialize() {
            if struct_.object_field.is_some() {
                panic!("unexpected object marker in parameter type");
            }
            serde_derive.serialize(ShouldDerive::Always);
        } else if struct_.object_field.is_none() {
            // Otherwise we can just use the proc macro if there is no object sentinel field
            serde_derive.serialize(ShouldDerive::Gated);
        }

        let fields = &struct_.fields;
        let name = self.ident;

        let mut fields_str = String::with_capacity(64);
        for field in fields {
            self.write_struct_field(&mut fields_str, field, serde_derive);
        }

        let lifetime_str = self.lifetime_param();
        write_derives_line(out, self.derives);
        serde_derive.write_derives(out);

        let _ = writedoc!(
            out,
            r"
        pub struct {name}{lifetime_str} {{
        {fields_str}
        }}
    "
        );

        if self.usage.used_as_request_param {
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
                        // `Default::default()` would also evaluate to `None` for `Option` types, but nice to
                        // generate less code and maybe make things easier for the compiler since most
                        // of these types are `Option`.
                        let field_default_val =
                            if field.rust_type.is_option() { "None" } else { "Default::default()" };
                        let _ = write!(cons_inner, "{}: {field_default_val},", field.field_name);
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
        if self.usage.should_impl_deserialize() {
            self.gen_miniserde_struct_deserialize(out, fields)
        }
        if let Some(obj_name) = &struct_.object_field {
            self.write_serde_serialization_with_object_tag(out, &struct_.fields, obj_name);
        }
    }

    fn write_struct_field(
        &self,
        out: &mut String,
        field: &StructField,
        serde_derive: SerdeDeriveState,
    ) {
        if let Some(doc_comment) = &field.doc_comment {
            let _ = writeln!(out, "{}", write_doc_comment(doc_comment, 1).trim_end());
        }
        if let Some(renamer) = &field.rename_as {
            serde_derive.maybe_write_rename(out, renamer);
        }

        if !field.required && field.rust_type.is_option() && self.usage.used_as_request_param {
            serde_derive.maybe_write_tag(out, r#"skip_serializing_if = "Option::is_none""#);
        }

        if matches!(
            field.rust_type.with_option_stripped(),
            RustType::Simple(SimpleType::Ext(ExtType::Value { .. }))
        ) {
            let with =
                if field.rust_type.is_option() { "with_serde_json_opt" } else { "with_serde_json" };
            serde_derive.maybe_write_tag(out, format!(r#"with = "stripe_types::{with}""#));
        }

        // For the private `AlwaysTrue` or `ObjectName` field used as a serde discriminant. For `miniserde`, it
        // is unused
        if field.rust_type.implies_private_field() {
            let _ = writeln!(out, "#[allow(dead_code)]");
        }

        let printable = self.get_printable(&field.rust_type);
        let typ = PrintableWithLifetime::new(&printable, self.lifetime);
        let vis = match field.vis {
            Visibility::Public => "pub ",
            Visibility::Private => "",
        };
        let _ = writeln!(out, "{vis}{}: {typ},", field.field_name);
    }

    fn write_serde_serialization_with_object_tag(
        &self,
        out: &mut String,
        fields: &[StructField],
        object_name: &str,
    ) {
        let name = self.ident;
        let field_count = fields.len() + 1;

        let mut fields_inner = String::new();
        for field in fields {
            let wire_name = field.rename_as.as_ref().unwrap_or(&field.field_name);
            let field_name = &field.field_name;
            let _ =
                writeln!(fields_inner, r#"s.serialize_field("{wire_name}", &self.{field_name})?;"#);
        }

        let _ = writedoc!(
            out,
            r#"
        #[cfg(feature = "serialize")]
        impl serde::Serialize for {name} {{
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {{
                use serde::ser::SerializeStruct;
                let mut s = s.serialize_struct("{name}", {field_count})?;
                {fields_inner}
                s.serialize_field("object", "{object_name}")?;
                s.end()
            }}
        }}
        "#
        );
    }
}
