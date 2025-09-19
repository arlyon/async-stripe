use std::fmt::Write as _;

use indoc::writedoc;

use crate::printable::PrintableWithLifetime;
use crate::rust_object::{EnumOfObjects, EnumVariant, FieldlessVariant};
use crate::templates::miniserde::gen_enum_of_objects_miniserde;
use crate::templates::object_writer::{ObjectWriter, write_derives_line};
use crate::templates::utils::{SerdeDeriveState, ShouldDerive};

impl ObjectWriter<'_> {
    /// Fallback case if no more specific structure inferred
    pub fn write_arbitrary_enum_variants(&self, out: &mut String, variants: &[EnumVariant]) {
        if !self.usage.used_as_request_param {
            unimplemented!();
        }
        let mut serde_derive = SerdeDeriveState::default();
        serde_derive.serialize(ShouldDerive::Always);

        let enum_name = self.ident;

        // Build the body of the enum definition
        let mut enum_body = String::with_capacity(64);
        let mut variants = variants.iter().collect::<Vec<_>>();
        variants.sort_by_key(|v| v.rust_type.is_some());
        for EnumVariant { variant, rust_type } in variants {
            if let Some(typ) = rust_type {
                let printable = self.components.construct_printable_type(typ);
                let typ = PrintableWithLifetime::new(&printable, self.lifetime);
                let _ = writeln!(enum_body, "#[serde(untagged)]");
                let _ = writeln!(enum_body, "{variant}({typ}),");
            } else {
                let _ = writeln!(enum_body, "{variant},");
            }
        }
        write_derives_line(out, self.derives);
        serde_derive.write_derives(out);
        let lifetime_str = self.lifetime_param();
        let _ = writedoc!(
            out,
            r#"
            #[serde(rename_all = "snake_case")]
            pub enum {enum_name}{lifetime_str} {{
            {enum_body}
            }}
        "#
        );
    }

    pub fn write_enum_of_objects(&self, out: &mut String, objects: &EnumOfObjects) {
        if self.lifetime.is_some() || self.usage.used_as_request_param {
            unimplemented!();
        }
        let mut serde_derive = SerdeDeriveState::default();

        serde_derive.serialize(ShouldDerive::Gated).deserialize(ShouldDerive::Gated);

        let enum_name = self.ident;

        // Build the body of the enum definition
        let mut enum_body = String::with_capacity(64);
        match objects {
            EnumOfObjects::MaybeDeleted(maybe_deleted) => {
                for item in [&maybe_deleted.base, &maybe_deleted.deleted] {
                    let name = &item.ident;
                    let printable = self.components.construct_printable_type_from_path(&item.path);
                    let _ = writeln!(enum_body, "{name}({printable}),");
                }
            }
            EnumOfObjects::ObjectUnion(objects) => {
                for (obj_name, obj) in objects {
                    let name = &obj.ident;
                    let printable = self.components.construct_printable_type_from_path(&obj.path);
                    serde_derive.maybe_write_rename(&mut enum_body, obj_name);
                    let _ = writeln!(enum_body, "{name}({printable}),");
                }
            }
        }
        if self.provide_unknown_variant {
            serde_derive.maybe_write_tag(&mut enum_body, "other");
            let _ = writeln!(enum_body, "Unknown");
        }

        write_derives_line(out, self.derives);
        serde_derive.write_derives(out);
        self.write_nonexhaustive_attr(out);

        match objects {
            EnumOfObjects::MaybeDeleted(_) => serde_derive.maybe_write_tag(out, "untagged"),
            EnumOfObjects::ObjectUnion(_) => serde_derive.maybe_write_tag(out, r#"tag = "object""#),
        }

        let miniserde_impl = gen_enum_of_objects_miniserde(enum_name, objects);
        let _ = writedoc!(
            out,
            r#"
            pub enum {enum_name} {{
            {enum_body}
            }}

            {miniserde_impl}
        "#
        );
    }

    /// Generate the enum definition and core methods
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
            Unknown(String),
        "
            );
        }

        // Build the body of the `as_str` implementation
        let mut as_str_body = String::with_capacity(32);
        for FieldlessVariant { wire_name, variant_name } in variants {
            let _ = writeln!(as_str_body, r#"{variant_name} => "{wire_name}","#);
        }
        if self.provide_unknown_variant {
            let _ = writeln!(as_str_body, r"Unknown(v) => v,");
        }
        let as_str_fn = if self.provide_unknown_variant {
            "pub fn as_str(&self) -> &str"
        } else {
            "pub fn as_str(self) -> &'static str"
        };

        // Build the body of the `from_str` implementation
        let mut from_str_body = String::with_capacity(32);
        for FieldlessVariant { wire_name, variant_name } in variants {
            let _ = writeln!(from_str_body, r#""{wire_name}" => Ok({variant_name}),"#);
        }

        let from_str_err = if self.provide_unknown_variant {
            let _ = writeln!(from_str_body, "v => Ok(Unknown(v.to_owned()))");
            "std::convert::Infallible"
        } else {
            let _ = writeln!(from_str_body, "_ => Err(stripe_types::StripeParseError)");
            "stripe_types::StripeParseError"
        };

        // NB: we manually implement Debug, Serialize, Deserialize to avoid duplicating
        // the (potentially many) strings in `as_str` and `from_str` used with the default derive.
        // These derived implementations often show up running `llvm-lines`, so easy
        // binary size + compile time win by doing this.
        write_derives_line(out, self.derives.eq(true).debug(false));
        self.write_nonexhaustive_attr(out);
        let _ = writedoc!(
            out,
            r#"
            pub enum {enum_name} {{
            {enum_def_body}
            }}
            impl {enum_name} {{
                {as_str_fn} {{
                    use {enum_name}::*;
                    match self {{
            {as_str_body}
                    }}
                }}
            }}
            
            impl std::str::FromStr for {enum_name} {{
                type Err = {from_str_err};
                fn from_str(s: &str) -> Result<Self, Self::Err> {{
                    use {enum_name}::*;
                    match s {{
                {from_str_body}
                    }}
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

        if !self.usage.should_impl_serialize() {
            let _ = writeln!(out, r#"#[cfg(feature = "serialize")]"#);
        }
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

        let miniserde_assign_line = if self.provide_unknown_variant {
            format!("Some({enum_name}::from_str(s).unwrap())")
        } else {
            format!("Some({enum_name}::from_str(s).map_err(|_| miniserde::Error)?)")
        };

        if self.usage.should_impl_deserialize() {
            let _ = writedoc!(
                out,
                r#"
            impl miniserde::Deserialize for {enum_name} {{
                fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {{
                    crate::Place::new(out)
                }}
            }}

            impl miniserde::de::Visitor for crate::Place<{enum_name}> {{
                fn string(&mut self, s: &str) -> miniserde::Result<()> {{
                    use std::str::FromStr;
                    self.out = {miniserde_assign_line};
                    Ok(())
                }}
            }}

            stripe_types::impl_from_val_with_from_str!({enum_name});
           "#
            );
        }

        let serde_ret_line = if self.provide_unknown_variant {
            "Ok(Self::from_str(&s).unwrap())".into()
        } else {
            format!(
                r#"Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for {enum_name}"))"#
            )
        };

        let _ = writedoc!(
            out,
            r#"
            #[cfg(feature = "deserialize")]
            impl<'de> serde::Deserialize<'de> for {enum_name} {{
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
                    use std::str::FromStr;
                    let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
                    {serde_ret_line}
                }}
            }}
            "#
        );
    }
}
