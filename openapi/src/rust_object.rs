use std::fmt::{Debug, Display, Formatter};

use indexmap::IndexMap;

use crate::components::Components;
use crate::rust_type::RustType;
use crate::types::{ComponentPath, RustIdent};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustObject {
    /// A struct definition
    Struct(Vec<StructField>),
    /// An enum definition.
    Enum(Vec<EnumVariant>),
    /// A definition of an enum, containing only fieldless variants, e.g. a C-like enum.
    FieldlessEnum(Vec<FieldlessVariant>),
}

/// Metadata about a `RustObject` - information related to documentation / naming, but
/// unrelated to the underlying type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ObjectMetadata {
    /// The identifier for the object.
    pub ident: RustIdent,
    /// A doc comment for this object.
    pub doc: Option<String>,
    /// The `title` field from the OpenAPI schema, used for deduplication purposes.
    pub title: Option<String>,
    /// The name of the field in the OpenAPI schema
    pub field_name: Option<String>,
}

impl ObjectMetadata {
    pub fn new(ident: RustIdent) -> Self {
        Self { ident, doc: None, title: None, field_name: None }
    }

    /// Attach a doc comment.
    pub fn doc(mut self, doc: String) -> Self {
        self.doc = Some(doc);
        self
    }
}

impl RustObject {
    /// Can this derive `Copy`?
    pub fn is_copy(&self) -> bool {
        match self {
            Self::Struct(fields) => fields.iter().all(|f| f.rust_type.is_copy()),
            Self::FieldlessEnum(_) => true,
            Self::Enum(variants) => variants.iter().all(|f| match &f.rust_type {
                None => true,
                Some(typ) => typ.is_copy(),
            }),
        }
    }

    pub fn get_struct_field(&self, field_name: &str) -> Option<&RustType> {
        match self {
            Self::Struct(fields) => {
                let field = fields.iter().find(|f| f.field_name == field_name);
                field.map(|f| &f.rust_type)
            }
            _ => None,
        }
    }

    /// Does this contain a reference type?
    pub fn has_reference(&self) -> bool {
        match self {
            Self::Struct(fields) => fields.iter().any(|f| f.rust_type.has_reference()),
            Self::FieldlessEnum(_) => false,
            Self::Enum(variants) => variants.iter().any(|v| match &v.rust_type {
                None => false,
                Some(typ) => typ.has_reference(),
            }),
        }
    }

    pub fn typs_mut(&mut self) -> Vec<&mut RustType> {
        let mut res = vec![];
        match self {
            Self::Struct(fields) => {
                for field in fields {
                    res.push(&mut field.rust_type)
                }
            }
            Self::FieldlessEnum(_) => {}
            Self::Enum(variants) => {
                for variant in variants {
                    if let Some(typ) = &mut variant.rust_type {
                        res.push(typ);
                    }
                }
            }
        }
        res
    }
}

/// Specification of an enum variant that may or may not have fields.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EnumVariant {
    /// Identifier for this variant
    pub variant: RustIdent,
    /// The type of the single field. If `None`, no inner field
    pub rust_type: Option<RustType>,
}

impl EnumVariant {
    /// Make an enum variant with the given name and type.
    pub fn new(variant: RustIdent, rust_type: RustType) -> Self {
        Self { variant, rust_type: Some(rust_type) }
    }

    /// Make a fieldless variant with the given name.
    pub fn fieldless(variant: RustIdent) -> Self {
        Self { variant, rust_type: None }
    }
}

/// Specification of a single field-less variant for an enum
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldlessVariant {
    /// Raw string association of the enum, used for renaming and `as_str` implementations
    pub wire_name: String,
    /// Identifier for this variant
    pub variant_name: RustIdent,
}

impl FieldlessVariant {
    pub fn new(wire: String, variant: RustIdent) -> Self {
        Self { wire_name: wire, variant_name: variant }
    }
}

/// Visibility of a struct field
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Visibility {
    /// `pub`
    Public,
    /// Private
    Private,
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
    /// Whether the field is required, affecting de/ser behavior.
    pub required: bool,
    /// Visibility of this field
    pub vis: Visibility,
}

// Manually implemented to avoid printing the `doc_comment`.
impl Debug for StructField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructField")
            .field("field_name", &self.field_name)
            .field("rust_type", &self.rust_type)
            .field("required", &self.required)
            .field("vis", &self.vis)
            .field("rename_as", &self.rename_as)
            .finish()
    }
}

impl StructField {
    pub fn new<T: Display>(field_name: T, rust_type: RustType, required: bool) -> Self {
        Self {
            field_name: field_name.to_string(),
            doc_comment: None,
            rename_as: None,
            required,
            vis: if rust_type.implies_private_field() {
                Visibility::Private
            } else {
                Visibility::Public
            },
            rust_type,
        }
    }

    /// Rename the field when de/serializing.
    pub fn rename_as<T: Display>(mut self, rename: T) -> Self {
        self.rename_as = Some(rename.to_string());
        self
    }

    /// Attach a doc comment to the field
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

pub struct ObjectRef {
    pub path: ComponentPath,
    pub feature_gate: Option<String>,
}

pub fn as_enum_of_objects<'a>(
    components: &'a Components,
    variants: &'a [EnumVariant],
) -> Option<IndexMap<&'a str, ObjectRef>> {
    let mut object_map = IndexMap::new();
    for variant in variants {
        let path = variant.rust_type.as_ref().and_then(|t| t.as_component_path())?;
        let obj = components.get(path);
        let name = obj.data.object_name.as_deref()?;
        object_map.insert(name, ObjectRef { path: path.clone(), feature_gate: None });
    }
    Some(object_map)
}
