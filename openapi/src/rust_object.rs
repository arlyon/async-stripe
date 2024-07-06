use std::fmt::{Debug, Display, Formatter};

use indexmap::IndexMap;

use crate::components::Components;
use crate::rust_type::RustType;
use crate::types::{ComponentPath, RustIdent};
use crate::visitor::{Visit, VisitMut};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustObject {
    /// A struct definition
    Struct(Struct),
    /// An enum definition.
    Enum(Vec<EnumVariant>),
    /// A definition of an enum, containing only fieldless variants, e.g. a C-like enum.
    FieldlessEnum(Vec<FieldlessVariant>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Struct {
    pub fields: Vec<StructField>,
    /// A marker type which will never be included in the generated code. Signals
    /// that we must serialize a field "object": "{name}" when serializing this struct
    pub object_field: Option<String>,
    pub vis: Visibility,
}

impl Struct {
    pub fn new(fields: Vec<StructField>) -> Self {
        Self { fields, object_field: None, vis: Visibility::Public }
    }

    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }
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
    pub parent: Option<RustIdent>,
}

impl ObjectMetadata {
    pub fn new(ident: RustIdent) -> Self {
        Self { ident, doc: None, title: None, field_name: None, parent: None }
    }

    /// Attach a doc comment.
    pub fn doc(mut self, doc: String) -> Self {
        self.doc = Some(doc);
        self
    }
}

impl RustObject {
    /// Can this derive `Copy`?
    pub fn is_copy(&self, components: &Components) -> bool {
        match self {
            Self::Struct(struct_) => struct_.fields.iter().all(|f| f.rust_type.is_copy(components)),
            Self::FieldlessEnum(_) => true,
            Self::Enum(variants) => variants.iter().all(|f| match &f.rust_type {
                None => true,
                Some(typ) => typ.is_copy(components),
            }),
        }
    }

    pub fn visit<'a, T: Visit<'a>>(&'a self, visitor: &mut T, usage: ObjectUsage) {
        match self {
            Self::Struct(struct_) => {
                for field in &struct_.fields {
                    visitor.visit_typ(&field.rust_type, usage);
                }
            }
            Self::Enum(variants) => {
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        visitor.visit_typ(typ, usage);
                    }
                }
            }
            Self::FieldlessEnum(_) => {}
        }
    }

    pub fn visit_mut<T: VisitMut>(&mut self, visitor: &mut T, usage: ObjectUsage) {
        match self {
            Self::Struct(struct_) => {
                for field in &mut struct_.fields {
                    visitor.visit_typ_mut(&mut field.rust_type, usage);
                }
            }
            Self::Enum(variants) => {
                for variant in variants {
                    if let Some(typ) = &mut variant.rust_type {
                        visitor.visit_typ_mut(typ, usage);
                    }
                }
            }
            Self::FieldlessEnum(_) => {}
        }
    }

    pub fn get_struct_field(&self, field_name: &str) -> Option<&RustType> {
        match self {
            Self::Struct(struct_) => {
                let field = struct_.fields.iter().find(|f| f.field_name == field_name);
                field.map(|f| &f.rust_type)
            }
            _ => None,
        }
    }

    /// Does this contain a reference type?
    pub fn has_reference(&self, components: &Components) -> bool {
        match self {
            Self::Struct(struct_) => {
                struct_.fields.iter().any(|f| f.rust_type.has_reference(components))
            }
            Self::FieldlessEnum(_) => false,
            Self::Enum(variants) => variants.iter().any(|v| match &v.rust_type {
                None => false,
                Some(typ) => typ.has_reference(components),
            }),
        }
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

impl Visibility {
    pub fn is_public(self) -> bool {
        matches!(self, Visibility::Public)
    }

    pub fn is_private(self) -> bool {
        matches!(self, Visibility::Private)
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => f.write_str("pub"),
            Visibility::Private => Ok(()),
        }
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
    pub fn new<T: Display>(field_name: T, rust_type: RustType) -> Self {
        Self {
            field_name: field_name.to_string(),
            doc_comment: None,
            rename_as: None,
            required: false,
            vis: Visibility::Public,
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

    /// Expected name of the field when (de)serializing
    pub fn wire_name(&self) -> &str {
        self.rename_as.as_ref().unwrap_or(&self.field_name)
    }

    pub fn vis(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

fn as_object_union(
    components: &Components,
    variants: &[EnumVariant],
) -> Option<IndexMap<String, ObjectRef>> {
    let mut objects = IndexMap::new();
    for variant in variants {
        let path = variant.rust_type.as_ref()?.as_component_path()?;
        let obj = components.get(path);

        let name = obj.data.object_name.as_deref()?;
        let obj_ref = ObjectRef { path: path.clone(), ident: obj.ident().clone() };

        // Union of objects cannot have duplicate object names since then we cannot discriminate
        // by the object key
        if objects.insert(name.to_string(), obj_ref).is_some() {
            return None;
        }
    }
    Some(objects)
}

fn as_maybe_deleted(components: &Components, variants: &[EnumVariant]) -> Option<MaybeDeleted> {
    if variants.len() != 2 {
        return None;
    }

    let mut base = None;
    let mut deleted = None;

    for variant in variants {
        let path = variant.rust_type.as_ref()?.as_component_path()?;
        let obj = components.get(path);
        let item = PathAndIdent { path: path.clone(), ident: obj.ident().clone() };
        if path.starts_with("deleted_") {
            deleted = Some(item);
        } else {
            base = Some(item);
        }
    }

    Some(MaybeDeleted { base: base?, deleted: deleted? })
}

pub fn as_enum_of_objects(
    components: &Components,
    variants: &[EnumVariant],
) -> Option<EnumOfObjects> {
    if let Some(obj_union) = as_object_union(components, variants) {
        Some(EnumOfObjects::ObjectUnion(obj_union))
    } else {
        as_maybe_deleted(components, variants).map(EnumOfObjects::MaybeDeleted)
    }
}

#[derive(Debug)]
pub struct ObjectRef {
    pub path: ComponentPath,
    pub ident: RustIdent,
}

#[derive(Debug)]
pub enum EnumOfObjects {
    MaybeDeleted(MaybeDeleted),
    ObjectUnion(IndexMap<String, ObjectRef>),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ObjectUsage {
    pub kind: ObjectKind,
    pub used_as_request_param: bool,
}

impl ObjectUsage {
    fn new(kind: ObjectKind, used_as_request_param: bool) -> Self {
        Self { kind, used_as_request_param }
    }

    pub fn request_param() -> Self {
        Self::new(ObjectKind::RequestParam, true)
    }

    pub fn type_def() -> Self {
        Self::new(ObjectKind::Type, false)
    }

    pub fn return_type() -> Self {
        Self::new(ObjectKind::RequestReturned, false)
    }

    pub fn should_impl_serialize(self) -> bool {
        self.used_as_request_param
    }

    pub fn should_impl_deserialize(self) -> bool {
        matches!(self.kind, ObjectKind::RequestReturned | ObjectKind::Type)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ObjectKind {
    /// Only used as a request parameter
    RequestParam,
    /// Only returned from a request
    RequestReturned,
    /// A core type potentially referenced anywhere
    Type,
}

#[derive(Debug)]
pub struct PathAndIdent {
    pub path: ComponentPath,
    pub ident: RustIdent,
}

#[derive(Debug)]
pub struct MaybeDeleted {
    pub base: PathAndIdent,
    pub deleted: PathAndIdent,
}
