use std::fmt::Write as _;

use heck::SnakeCase;
use indoc::{formatdoc, writedoc};

use crate::util::{write_doc_comment, write_serde_rename};

/// Data used to write the definition and methods for a field-less enum
#[derive(Debug, Clone)]
pub struct EnumTemplateData<'a> {
    /// The identifier for this enum
    pub enum_name: &'a str,
    /// All variants of this enum
    pub variants: Vec<EnumVariant<'a>>,
    /// If `Some`, the variant used to impl `Default` for the enum
    pub default_variant: Option<String>,
    /// Used to document this enum
    pub doc_comment: String,
}

impl<'a> EnumTemplateData<'a> {
    pub fn new(enum_name: &'a str, doc_comment: String) -> Self {
        Self { enum_name, variants: vec![], default_variant: None, doc_comment }
    }

    pub fn add_variant(&mut self, variant: EnumVariant<'a>) {
        self.variants.push(variant);
    }

    pub fn with_default_variant(mut self, variant: String) -> Self {
        self.default_variant = Some(variant);
        self
    }

    pub fn gen_definition_and_methods(&self) -> String {
        let enum_name = self.enum_name;

        // Build the body of the enum definition
        let mut enum_def_body = String::new();
        for EnumVariant { wire_name, variant_name } in &self.variants {
            if &variant_name.to_snake_case() != wire_name {
                let _ = writeln!(enum_def_body, "{}", write_serde_rename(wire_name));
            }
            let _ = writeln!(enum_def_body, "{variant_name},");
        }

        // Build the body of the `as_str` implementation
        let as_str_body = self
            .variants
            .iter()
            .map(|EnumVariant { wire_name, variant_name }| {
                format!(r#"{enum_name}::{variant_name} => "{wire_name}","#)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let doc_comment = write_doc_comment(&self.doc_comment, 0);
        let trimmed_doc_comment = doc_comment.trim_end();
        let mut out = formatdoc!(
            r#"
            
            {trimmed_doc_comment}
            #[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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

        if let Some(default_variant) = &self.default_variant {
            let _ = writedoc!(
                out,
                r"
            impl std::default::Default for {enum_name} {{
                fn default() -> Self {{
                    Self::{default_variant}
                }}
            }}"
            );
        }
        out
    }
}

/// Specification of a single field-less variant for an enum
#[derive(Debug, Clone)]
pub struct EnumVariant<'a> {
    /// Raw string association of the enum, used for renaming and `as_str` implementations
    pub wire_name: &'a str,
    /// Identifier for this variant
    pub variant_name: String,
}
