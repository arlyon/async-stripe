use std::fmt::Display;

use indoc::formatdoc;

use crate::templates::derives::{write_derives_line, Derives};
use crate::util::{write_doc_comment, write_serde_rename};

/// Specification for a field in a struct
#[derive(Debug, Clone)]
pub struct StructField {
    /// Used to document this enum if provided
    pub doc_comment: Option<String>,
    /// If provided, used to apply `serde(rename)`
    pub rename_as: Option<String>,
    /// If provided, used to apply `serde(default)`.
    // TODO: could be clearer, for now, Some(None) applies to `serde(default)`, Some(Some(...))
    // to `serde(default = ...)`
    pub serde_default: Option<Option<&'static str>>,
    /// If provided, used to apply `serde(skip_serializing_if)`.
    pub skip_serializing: Option<&'static str>,
    /// Name of this field
    pub field_name: String,
    /// Type for this field
    pub rust_type: String,
    /// Whether this field should be required in the constructor
    pub required: bool,
}

impl StructField {
    pub fn new<T: Display, U: Display>(field_name: T, rust_type: U) -> Self {
        Self {
            field_name: field_name.to_string(),
            rust_type: rust_type.to_string(),
            serde_default: None,
            doc_comment: None,
            rename_as: None,
            skip_serializing: None,
            required: false,
        }
    }

    pub fn rename_as<T: Display>(mut self, rename: T) -> Self {
        self.rename_as = Some(rename.to_string());
        self
    }

    pub fn doc<T: Display>(mut self, doc_comment: T) -> Self {
        self.doc_comment = Some(doc_comment.to_string());
        self
    }

    pub fn skip_serializing(mut self, skip_serializing: &'static str) -> Self {
        self.skip_serializing = Some(skip_serializing);
        self
    }

    pub fn set_skip_serializing(mut self, skip_serializing: Option<&'static str>) -> Self {
        self.skip_serializing = skip_serializing;
        self
    }

    pub fn serde_default(mut self) -> Self {
        self.serde_default = Some(None);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn apply_specific_serde_default(mut self, val: &'static str) -> Self {
        self.serde_default = Some(Some(val));
        self
    }

    pub fn gen_def(&self) -> String {
        let mut lines = vec![];
        if let Some(doc_comment) = &self.doc_comment {
            // For simplicity, strip newline from doc comment since we join with newlines later
            lines.push(write_doc_comment(doc_comment, 1).trim_end().to_string());
        }
        if let Some(renamer) = &self.rename_as {
            lines.push(write_serde_rename(renamer));
        }
        if let Some(skip_serializing) = self.skip_serializing {
            lines.push(format!(r#"#[serde(skip_serializing_if = "{skip_serializing}")]"#));
        }
        match self.serde_default {
            None => {}
            Some(Some(default)) => {
                lines.push(format!(r#"#[serde(default = "{default}")]"#));
            }
            Some(None) => lines.push("#[serde(default)]".into()),
        }
        lines.push(format!("pub {}: {},", self.field_name, self.rust_type));
        lines.join("\n")
    }
}

#[derive(Clone)]
pub struct StructTemplateData {
    pub doc_comment: Option<String>,
    pub derives: Vec<Derives>,
    pub struct_name: String,
    pub fields: Vec<StructField>,
    pub lifetime: Option<&'static str>,
}

impl StructTemplateData {
    pub fn new<T: Display>(struct_name: T) -> Self {
        Self {
            doc_comment: None,
            derives: vec![],
            struct_name: struct_name.to_string(),
            fields: vec![],
            lifetime: None,
        }
    }

    pub fn doc<T: Display>(mut self, comment: T) -> Self {
        self.doc_comment = Some(comment.to_string());
        self
    }

    pub fn add_field(&mut self, field: StructField) {
        self.fields.push(field);
    }

    pub fn derives(mut self, derives: &[Derives]) -> Self {
        self.derives = derives.to_vec();
        self
    }

    pub fn lifetime(mut self, lifetime: &'static str) -> Self {
        self.lifetime = Some(lifetime);
        self
    }

    fn lifetime_def(&self) -> String {
        self.lifetime.map(|l| format!("<{l}>")).unwrap_or_default()
    }
}

impl StructTemplateData {
    pub fn gen_definition(&self) -> String {
        let doc_comment = if let Some(doc) = &self.doc_comment {
            // TODO: remove replace hack to minimize diff
            write_doc_comment(doc, 0).replace(">.", ">")
        } else {
            String::new()
        };
        let derives = write_derives_line(&self.derives);
        let fields = self.fields.iter().map(StructField::gen_def).collect::<Vec<_>>().join("\n\n");
        let lifetime = self.lifetime_def();
        let struct_name = &self.struct_name;
        formatdoc!(
            r"
        {doc_comment}{derives}
        pub struct {struct_name}{lifetime} {{
        {fields}
        }}
    "
        )
    }

    pub fn gen_constructor(&self) -> String {
        // Generate parameter arguments
        let new_args = self
            .fields
            .iter()
            .filter(|p| p.required)
            .map(|p| format!("{}: {}", p.field_name, p.rust_type))
            .collect::<Vec<_>>()
            .join(",");

        let fields = self
            .fields
            .iter()
            .map(|p| {
                let line_end = if p.required { "" } else { ": Default::default()" };
                format!("{}{line_end}", p.field_name)
            })
            .collect::<Vec<_>>()
            .join(",\n");

        let struct_name = &self.struct_name;
        let lifetime = self.lifetime_def();
        formatdoc!(
            r"
            impl{lifetime} {struct_name}{lifetime} {{
                pub fn new({new_args}) -> Self {{
                    {struct_name} {{
                        {fields}
                    }}
                }}
            }}
        "
        )
    }
}
