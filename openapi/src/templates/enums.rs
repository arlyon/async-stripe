use std::fmt::Write as _;

use indexmap::IndexMap;
use indoc::writedoc;

use crate::components::Components;
use crate::printable::PrintableWithLifetime;
use crate::rust_object::{EnumVariant, FieldlessVariant, ObjectRef};
use crate::templates::object_writer::{write_derives_line, ObjectWriter};
use crate::templates::utils::write_serde_rename;

impl<'a> ObjectWriter<'a> {
    pub fn write_enum_variants(&self, out: &mut String, variants: &[EnumVariant]) {
        let enum_name = self.ident;

        // Build the body of the enum definition
        let mut enum_body = String::with_capacity(64);
        for EnumVariant { variant, rust_type } in variants {
            if let Some(typ) = rust_type {
                let printable = self.components.construct_printable_type(typ);
                let typ = PrintableWithLifetime::new(&printable, self.lifetime);
                let _ = writeln!(enum_body, "{variant}({typ}),");
            } else {
                let _ = writeln!(enum_body, "{variant},");
            }
        }
        self.write_derives_line(out);
        let lifetime_str = self.lifetime_param();
        let _ = writedoc!(
            out,
            r#"
            #[serde(untagged)]
            pub enum {enum_name}{lifetime_str} {{
            {enum_body}
            }}
        "#
        );
    }

    pub fn write_enum_of_objects(
        &self,
        out: &mut String,
        components: &Components,
        objects: &IndexMap<&str, ObjectRef>,
    ) {
        if self.lifetime.is_some() {
            unimplemented!();
        }
        let enum_name = self.ident;

        // Build the body of the enum definition
        let mut enum_body = String::with_capacity(64);
        for (obj_name, obj_ref) in objects {
            let comp = components.get(&obj_ref.path);
            let name = comp.ident();
            let printable = components.construct_printable_type_from_path(&obj_ref.path);
            if let Some(feature_gate) = &obj_ref.feature_gate {
                let _ = writeln!(enum_body, r#"#[cfg(feature = "{feature_gate}")]"#);
            }
            write_serde_rename(&mut enum_body, obj_name);
            let _ = writeln!(enum_body, "{name}({printable}),");
        }
        if self.provide_unknown_variant {
            let _ = writeln!(enum_body, r"#[serde(other)]");
            let _ = writeln!(enum_body, r"Unknown");
        }

        self.write_derives_line(out);
        self.write_nonexhaustive_attr(out);
        let _ = writedoc!(
            out,
            r#"
            #[serde(tag = "object")]
            pub enum {enum_name} {{
            {enum_body}
            }}
        "#
        );
    }

    /// Generate the enum definition, along with the methods `as_str`, `as_ref`, `impl Display`,
    /// and `impl Default`.
    pub fn write_fieldless_enum_variants(&self, out: &mut String, variants: &[FieldlessVariant]) {
        let enum_name = self.ident;
        // Build the body of the enum definition
        let mut enum_def_body = String::with_capacity(128);
        for FieldlessVariant { variant_name, .. } in variants {
            let _ = writeln!(enum_def_body, "{variant_name},");
        }
        if self.provide_unknown_variant {
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
        if self.provide_unknown_variant {
            let _ = writeln!(as_str_body, r#"Unknown => "unknown","#);
        }

        // Build the body of the `from_str` implementation
        let mut from_str_body = String::with_capacity(32);
        for FieldlessVariant { wire_name, variant_name } in variants {
            let _ = writeln!(from_str_body, r#""{wire_name}" => Ok({variant_name}),"#);
        }
        let _ = writeln!(from_str_body, "_ => Err(())");

        let mut derives = self.derives;
        let derive_deser = derives.deserialize;
        let derive_serialize = derives.serialize;

        // NB: we unset the derive flags for serialize + deserialize + debug to avoid duplicating
        // the (potentially many) strings in `as_str` and `from_str` through the default derive.
        // These derived implementations often show up running `llvm-lines`, so easy
        // binary size + compile time win by doing this.
        derives.copy(true).eq(true).serialize(false).deserialize(false).debug(false);

        write_derives_line(out, derives);
        self.write_nonexhaustive_attr(out);
        let _ = writedoc!(
            out,
            r#"
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
            let ret_line = if self.provide_unknown_variant {
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
}
