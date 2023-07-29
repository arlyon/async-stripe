use std::fmt::Write as _;

use indoc::writedoc;

use crate::rust_object::{EnumVariant, RustEnum};
use crate::templates::derives::{write_derives_line, Derives};
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
        for EnumVariant { variant_name, .. } in &self.variants {
            let _ = writeln!(enum_def_body, "{variant_name},");
        }

        // Build the body of the `as_str` implementation
        let mut as_str_body = String::with_capacity(32);
        for EnumVariant { wire_name, variant_name } in &self.variants {
            let _ = writeln!(as_str_body, r#"Self::{variant_name} => "{wire_name}","#);
        }

        // Build the body of the `from_str` implementation
        let mut from_str_body = String::with_capacity(32);
        for EnumVariant { wire_name, variant_name } in &self.variants {
            let _ = writeln!(from_str_body, r#""{wire_name}" => Ok(Self::{variant_name}),"#);
        }

        let derive_deser = additional_derives.derives_deserialize();
        let derive_serialize = additional_derives.derives_serialize();

        // NB: we set serialize to false since we implement manually using `as_str` to avoid
        // duplicating the `as_str` implementation. This also avoids generating some
        // unnecessary serialization methods. The same logic applies for deserialization
        let derives = additional_derives
            .copy(true)
            .eq(true)
            .serialize(false)
            .deserialize(false)
            .miniserde_deserialize(false);

        let derives = write_derives_line(derives);
        let _ = writedoc!(
            out,
            r#"
            {derives}
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
            
            impl std::str::FromStr for {enum_name} {{
                type Err = ();
                fn from_str(s: &str) -> Result<Self, Self::Err> {{
                    match s {{
                {from_str_body}
                        _ => Err(())
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
            let _ = writedoc!(
                out,
                r#"
            impl<'de> serde::Deserialize<'de> for {enum_name} {{
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
                    use std::str::FromStr;
                    let s: String = serde::Deserialize::deserialize(deserializer)?;
                    Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for {enum_name}"))
                }}
            }}

            #[cfg(feature = "min-ser")]
            impl miniserde::Deserialize for {enum_name} {{
                fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {{
                    crate::Place::new(out)
                }}
            }}
            
            #[cfg(feature = "min-ser")]
            impl miniserde::de::Visitor for crate::Place<{enum_name}> {{
                fn string(&mut self, s: &str) -> miniserde::Result<()> {{
                    use std::str::FromStr;
                    self.out = Some({enum_name}::from_str(s).map_err(|_| miniserde::Error)?);
                    Ok(())
                }}
            }}
            "#
            );
        }
    }
}
