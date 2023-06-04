use std::fmt::{Debug, Display, Formatter};

use crate::object_context::PrintableType;
use crate::rust_type::RustType;
use crate::types::{ComponentPath, RustIdent};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustObjectData {
    Struct(Vec<StructField>),
    Enum(RustEnum),
    FieldedEnum(Vec<FieldedEnumVariant>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RustObject {
    pub doc_comment: Option<String>,
    pub ident: RustIdent,
    pub data: RustObjectData,
}

impl RustObjectData {
    pub fn schema_deps(&self) -> Vec<&ComponentPath> {
        let mut deps = vec![];
        self.add_schema_deps(&mut deps);
        deps
    }

    pub fn is_copy(&self) -> bool {
        match self {
            RustObjectData::Struct(fields) => fields.iter().all(|f| f.rust_type.is_copy()),
            RustObjectData::Enum(_) => true,
            RustObjectData::FieldedEnum(variants) => variants.iter().all(|f| match &f.rust_type {
                None => true,
                Some(typ) => typ.is_copy(),
            }),
        }
    }

    pub fn has_borrow(&self) -> bool {
        match self {
            RustObjectData::Struct(fields) => fields.iter().any(|f| f.rust_type.has_borrow()),
            RustObjectData::Enum(_) => false,
            RustObjectData::FieldedEnum(variants) => variants.iter().any(|v| match &v.rust_type {
                None => false,
                Some(typ) => typ.has_borrow(),
            }),
        }
    }

    fn add_schema_deps<'a>(&'a self, deps: &mut Vec<&'a ComponentPath>) {
        match self {
            Self::Struct(fields) => {
                for field in fields {
                    if let Some(component_path) = field.rust_type.as_reference_path() {
                        deps.push(component_path);
                    }
                }
            }
            Self::FieldedEnum(variants) => {
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        if let Some(component_path) = typ.as_reference_path() {
                            deps.push(component_path);
                        }
                    }
                }
            }
            Self::Enum(_) => {}
        }
    }
}

/// Specification of a variant for an enum of the form `Ident(Type)`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldedEnumVariant {
    /// Identifier for this variant
    pub variant: RustIdent,
    /// The type of the single field. If `None`, no inner field
    pub rust_type: Option<RustType>,
}

impl FieldedEnumVariant {
    pub fn new(variant: RustIdent, rust_type: RustType) -> Self {
        Self { variant, rust_type: Some(rust_type) }
    }

    pub fn fieldless(variant: RustIdent) -> Self {
        Self { variant, rust_type: None }
    }
}

/// Specification of a variant for an enum of the form `Ident(Type)`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PrintableFieldedEnumVariant {
    /// Identifier for this variant
    pub variant: RustIdent,
    /// The type of the single field
    pub rust_type: Option<PrintableType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct RustEnum {
    pub variants: Vec<EnumVariant>,
}

impl RustEnum {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_variant(&mut self, variant: EnumVariant) {
        self.variants.push(variant);
    }
}

/// Specification of a single field-less variant for an enum
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EnumVariant {
    /// Raw string association of the enum, used for renaming and `as_str` implementations
    pub wire_name: String,
    /// Identifier for this variant
    pub variant_name: RustIdent,
}

#[derive(Debug, Clone, Default)]
pub struct RustObjectBuilder {
    pub doc_comment: Option<String>,
}

impl RustObjectBuilder {
    pub fn new() -> Self {
        Self { doc_comment: None }
    }

    pub fn maybe_doc<T: Display>(mut self, doc_comment: Option<T>) -> Self {
        if let Some(doc) = &doc_comment {
            self.doc_comment = Some(doc.to_string());
        }
        self
    }

    pub fn build(self, data: RustObjectData, ident: RustIdent) -> RustObject {
        RustObject { doc_comment: self.doc_comment, data, ident }
    }
}

/// Specification for a field in a struct
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct StructField {
    /// Used to document this enum if provided
    pub doc_comment: Option<String>,
    /// If provided, used to apply `serde(rename)`
    pub rename_as: Option<String>,
    /// Name of this field
    pub field_name: String,
    /// Type for this field
    pub rust_type: RustType,
    pub required: bool,
}

impl Debug for StructField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructField")
            .field("field_name", &self.field_name)
            .field("rust_type", &self.rust_type)
            .field("required", &self.required)
            .finish()
    }
}

/// Specification for a field in a struct
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PrintableStructField<'a> {
    /// Used to document this enum if provided
    pub doc_comment: Option<&'a str>,
    /// If provided, used to apply `serde(rename)`
    pub rename_as: Option<&'a str>,
    /// If provided, used to apply `serde(default)`.
    pub deser_default: Option<DeserDefault>,
    /// If provided, used to apply `serde(skip_serializing_if)`.
    pub skip_serializing: Option<&'static str>,
    /// Name of this field
    pub field_name: &'a str,
    /// Type for this field
    pub rust_type: PrintableType,
    pub required: bool,
}

impl<'a> PrintableStructField<'a> {
    pub fn from_field(field: &'a StructField, printable_type: PrintableType) -> Self {
        let mut deser_default = None;
        let mut skip_serializing = None;
        if !field.required {
            if let Some(default) = field.rust_type.as_deser_default() {
                deser_default = Some(default);
            }
            if let Some(skip) = field.rust_type.as_skip_serializing() {
                skip_serializing = Some(skip);
            }
        }
        Self {
            doc_comment: field.doc_comment.as_deref(),
            rename_as: field.rename_as.as_deref(),
            deser_default,
            skip_serializing,
            field_name: &field.field_name,
            rust_type: printable_type,
            required: field.required,
        }
    }
}

impl StructField {
    pub fn new<T: Display>(field_name: T, rust_type: RustType, required: bool) -> Self {
        Self {
            field_name: field_name.to_string(),
            rust_type,
            doc_comment: None,
            rename_as: None,
            required,
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
}

/// Specifications for a `serde(default = ...)` attribute
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub enum DeserDefault {
    /// Just `serde(default)`
    #[default]
    Default,
}

impl DeserDefault {
    /// The corresponding `serde` attribute
    pub fn to_serde_attr(self) -> &'static str {
        match self {
            Self::Default => "#[serde(default)]",
        }
    }
}