use std::fmt::{Display, Write as _};

use indoc::{formatdoc, writedoc};

use crate::util::{write_doc_comment, write_serde_rename};

/// Data used to write the definition and methods for an enum with fields
#[derive(Debug, Clone)]
pub struct EnumWithFieldData {
    /// The identifier for this enum
    pub enum_name: String,
    /// All variants of this enum
    pub variants: Vec<FieldedEnumVariant>,
    /// If `Some`, the variant used to impl `Default` for the enum
    pub default_variant: Option<String>,
    /// Used to document this enum if provided
    pub doc_comment: Option<String>,
}

impl EnumWithFieldData {
    pub fn new<T: Display>(enum_name: T) -> Self {
        Self {
            enum_name: enum_name.to_string(),
            default_variant: None,
            doc_comment: None,
            variants: vec![],
        }
    }

    pub fn add_variant(&mut self, variant: FieldedEnumVariant) {
        self.variants.push(variant);
    }

    pub fn with_default_variant(mut self, variant: String) -> Self {
        self.default_variant = Some(variant);
        self
    }

    pub fn doc<T: Display>(mut self, doc_comment: T) -> Self {
        self.doc_comment = Some(doc_comment.to_string());
        self
    }

    pub fn gen_definition_and_methods(&self) -> String {
        let enum_name = &self.enum_name;

        // Build the body of the enum definition
        let mut enum_body = String::new();
        for FieldedEnumVariant { variant_name, type_name, rename_as, .. } in &self.variants {
            if let Some(rename) = rename_as {
                let _ = writeln!(enum_body, "{}", write_serde_rename(rename));
            }
            let _ = writeln!(enum_body, "{variant_name}({type_name}),");
        }
        let doc_comment = write_doc_comment(self.doc_comment.as_deref().unwrap_or_default(), 0);
        let trimmed_doc = doc_comment.trim();
        let mut out = formatdoc!(
            r#"
            
            {trimmed_doc}
            #[derive(Clone, Debug, Deserialize, Serialize)]
            #[serde(untagged, rename_all = "snake_case")]
            pub enum {enum_name} {{
            {enum_body}
            }}
        "#
        );

        if let Some(variant) = &self.default_variant {
            let _ = writedoc!(
                out,
                r"
                impl std::default::Default for {enum_name} {{
                    fn default() -> Self {{
                        Self::{variant}(Default::default())
                    }}
                }}
                "
            );
        }
        out
    }
}

/// Specification of a variant for an enum of the form `Ident(Type)`
#[derive(Debug, Clone)]
pub struct FieldedEnumVariant {
    /// Identifier for this variant
    pub variant_name: String,
    /// The type of the single field
    pub type_name: String,
    /// If `Some`, used to apply `serde(rename)` to this variant
    pub rename_as: Option<String>,
}

impl FieldedEnumVariant {
    pub fn new<T: Display, U: Display>(variant: T, type_name: U) -> Self {
        Self {
            variant_name: variant.to_string(),
            type_name: type_name.to_string(),
            rename_as: None,
        }
    }

    pub fn rename_as<T: Display>(mut self, rename: T) -> Self {
        self.rename_as = Some(rename.to_string());
        self
    }
}
