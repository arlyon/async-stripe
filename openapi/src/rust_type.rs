use std::fmt::{Debug, Display, Formatter};

use crate::rust_object::{DeserDefault, ObjectMetadata, RustObject};
use crate::types::{ComponentPath, RustIdent};

/// A path to a type defined elsewhere.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PathToType {
    /// A top-level component.
    Component { path: ComponentPath },
    /// The id for a top-level component.
    ObjectId { path: ComponentPath, is_ref: bool },
    /// A type generated in `stripe_types`, so printed as `stripe_types::{ident}`
    Types { ident: RustIdent, is_ref: bool, has_ref: bool, is_copy: bool },
    /// A reference to a type in the same file (so not import needed).
    IntraFile { ident: RustIdent, is_ref: bool, has_ref: bool, is_copy: bool },
}

impl PathToType {
    pub fn is_copy(&self) -> bool {
        use PathToType::*;
        match self {
            Component { .. } | ObjectId { .. } => false,
            Types { is_copy, .. } => *is_copy,
            IntraFile { is_copy, .. } => *is_copy,
        }
    }

    pub fn has_reference(&self) -> bool {
        use PathToType::*;
        match self {
            Component { .. } | ObjectId { .. } => false,
            Types { has_ref, .. } => *has_ref,
            IntraFile { has_ref, .. } => *has_ref,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustType {
    Object(RustObject, ObjectMetadata),
    /// Either a Rust defined scalar type, or a type predefined in `stripe_types`.
    Simple(SimpleType),
    Path(PathToType),
    /// Type containing an inner type.
    Container(Container),
}

impl RustType {
    pub const fn bool() -> Self {
        Self::Simple(SimpleType::Bool)
    }

    pub const fn float() -> Self {
        Self::Simple(SimpleType::Float)
    }

    pub const fn string() -> Self {
        Self::Simple(SimpleType::String)
    }

    pub const fn int(typ: IntType) -> Self {
        Self::Simple(SimpleType::Int(typ))
    }

    pub const fn ext(ext: ExtType) -> Self {
        Self::Simple(SimpleType::Ext(ext))
    }

    pub fn option(typ: Self) -> Self {
        Self::Container(Container::Option(Box::new(typ)))
    }

    pub fn boxed(typ: Self) -> Self {
        Self::Container(Container::Box(Box::new(typ)))
    }

    pub fn list(typ: Self) -> Self {
        Self::Container(Container::List(Box::new(typ)))
    }

    /// Construct an `Expandable<{typ}>`.
    pub fn expandable(typ: Self) -> Self {
        Self::Container(Container::Expandable(Box::new(typ)))
    }

    /// Construct a `HashMap<String, {typ}>`.
    pub fn str_map(value: Self, is_ref: bool) -> Self {
        Self::Container(Container::Map { key: MapKey::String, value: Box::new(value), is_ref })
    }

    /// Construct a `HashMap<Currency, {typ}>`.
    pub fn currency_map(typ: Self, is_ref: bool) -> Self {
        Self::Container(Container::Map { key: MapKey::Currency, value: Box::new(typ), is_ref })
    }

    /// Construct a `Vec<{typ}>`.
    pub fn vec(typ: Self) -> Self {
        Self::Container(Container::Vec(Box::new(typ)))
    }

    /// Construct a `&[{typ}]`
    pub fn slice(typ: Self) -> Self {
        Self::Container(Container::Slice(Box::new(typ)))
    }

    pub fn as_id_path(&self) -> Option<&ComponentPath> {
        match self {
            Self::Path(PathToType::ObjectId { path, .. }) => Some(path),
            Self::Container(typ) => typ.value_typ().as_id_path(),
            _ => None,
        }
    }

    pub fn as_component_path(&self) -> Option<&ComponentPath> {
        match self {
            Self::Path(PathToType::Component { path }) => Some(path),
            Self::Container(typ) => typ.value_typ().as_component_path(),
            _ => None,
        }
    }

    pub fn implies_private_field(&self) -> bool {
        matches!(self, Self::Simple(SimpleType::Ext(ExtType::AlwaysTrue)))
    }

    /// Is this type `Option<>`?
    pub const fn is_option(&self) -> bool {
        matches!(self, Self::Container(Container::Option(_)))
    }

    /// Can this type derive `Copy`?
    pub fn is_copy(&self) -> bool {
        use Container::*;
        match self {
            Self::Object(obj, ..) => obj.is_copy(),
            Self::Simple(typ) => typ.is_copy(),
            Self::Path(typ) => typ.is_copy(),
            Self::Container(typ) => match typ {
                List(_) | Vec(_) | Expandable(_) => false,
                Slice(_) => true,
                Option(inner) | Box(inner) => inner.is_copy(),
                Map { is_ref, .. } => *is_ref,
            },
        }
    }

    /// Does this contain a reference? Primarily used for detecting types that may
    /// require lifetimes.
    pub fn has_reference(&self) -> bool {
        match self {
            Self::Object(obj, ..) => obj.has_reference(),
            Self::Simple(typ) => typ.is_reference(),
            Self::Path(typ) => typ.has_reference(),
            Self::Container(typ) => match typ {
                Container::Slice(_) => true,
                Container::Map { is_ref: true, .. } => true,
                _ => typ.value_typ().has_reference(),
            },
        }
    }

    pub fn component_path(path: ComponentPath) -> Self {
        Self::Path(PathToType::Component { path })
    }

    pub fn serde_json_value(is_ref: bool) -> Self {
        Self::Simple(SimpleType::Ext(ExtType::Value { is_ref }))
    }

    pub fn object_id(id_path: ComponentPath, is_ref: bool) -> Self {
        Self::Path(PathToType::ObjectId { path: id_path, is_ref })
    }

    pub fn into_nullable(self) -> Self {
        match self {
            Self::Container(Container::List(_)) | Self::Container(Container::Option(_)) => self,
            _ => Self::option(self),
        }
    }

    pub fn as_skip_serializing(&self) -> Option<&'static str> {
        match self {
            Self::Container(Container::Option(_)) => Some("Option::is_none"),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<(&RustObject, &ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            Self::Simple(_) | Self::Path { .. } => None,
            Self::Container(typ) => typ.value_typ().as_object(),
        }
    }

    pub fn as_object_mut(&mut self) -> Option<(&mut RustObject, &ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            Self::Simple(_) | Self::Path { .. } => None,
            Self::Container(typ) => typ.value_typ_mut().as_object_mut(),
        }
    }

    pub fn into_object(self) -> Option<(RustObject, ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            Self::Simple(_) | Self::Path { .. } => None,
            Self::Container(typ) => typ.into_value_typ().into_object(),
        }
    }

    pub fn as_rust_object_mut(&mut self) -> Option<&mut RustObject> {
        self.as_object_mut().map(|r| r.0)
    }

    pub fn as_rust_object(&self) -> Option<&RustObject> {
        self.as_object().map(|r| r.0)
    }

    pub fn as_deser_default(&self) -> Option<DeserDefault> {
        match self {
            Self::Simple(SimpleType::Bool)
            | Self::Container(Container::Vec(_))
            | Self::Container(Container::List(_)) => Some(DeserDefault::Default),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MapKey {
    String,
    Currency,
}

impl Display for MapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Self::String => "String",
            Self::Currency => "stripe_types::Currency",
        };
        f.write_str(as_str)
    }
}

/// Representation of a type containing an inner type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Container {
    /// List<{typ}>
    List(Box<RustType>),
    /// Vec<{typ}>
    Vec(Box<RustType>),
    /// &[{typ}]
    Slice(Box<RustType>),
    /// Expandable<{typ}>
    Expandable(Box<RustType>),
    /// Option<{typ}>
    Option(Box<RustType>),
    /// Box<{typ}>
    Box(Box<RustType>),
    /// HashMap<{key}, {typ}>
    Map {
        /// HashMap key type.
        key: MapKey,
        /// HashMap value type.
        value: Box<RustType>,
        /// Are we a reference?
        is_ref: bool,
    },
}

impl Container {
    /// Get a reference to the inner contained type.
    pub fn value_typ(&self) -> &RustType {
        use Container::*;
        match self {
            List(typ) => typ,
            Vec(typ) => typ,
            Slice(typ) => typ,
            Expandable(typ) => typ,
            Option(typ) => typ,
            Box(typ) => typ,
            Map { value, .. } => value,
        }
    }

    /// Extract the inner contained type.
    pub fn into_value_typ(self) -> RustType {
        use Container::*;
        let res = match self {
            List(typ) => typ,
            Vec(typ) => typ,
            Slice(typ) => typ,
            Expandable(typ) => typ,
            Option(typ) => typ,
            Box(typ) => typ,
            Map { value, .. } => value,
        };
        *res
    }

    /// Get a mutable reference to the inner contained type.
    pub fn value_typ_mut(&mut self) -> &mut RustType {
        use Container::*;
        match self {
            List(typ) => typ,
            Vec(typ) => typ,
            Slice(typ) => typ,
            Expandable(typ) => typ,
            Option(typ) => typ,
            Box(typ) => typ,
            Map { value, .. } => value,
        }
    }
}

/// Either a Rust defined scalar type, or a type predefined in `stripe_types`.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SimpleType {
    /// bool
    Bool,
    /// f64
    Float,
    /// &str
    Str,
    /// String
    String,
    /// One of primitive rust integer types.
    Int(IntType),
    /// Type defined in `stripe_types`.
    Ext(ExtType),
}

impl SimpleType {
    /// Does this type implement `Copy`?
    pub const fn is_copy(self) -> bool {
        !matches!(self, Self::String | Self::Ext(ExtType::Value { is_ref: false }))
    }

    /// Is this type a reference?
    pub const fn is_reference(self) -> bool {
        matches!(self, Self::Str | Self::Ext(ExtType::Value { is_ref: true }))
    }

    pub const fn ident(self) -> &'static str {
        use SimpleType::*;
        match self {
            Bool => "bool",
            Float => "f64",
            String => "String",
            Str => "str",
            Int(typ) => typ.as_str(),
            Ext(ext) => ext.ident(),
        }
    }
}

impl Debug for SimpleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.ident())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum IntType {
    U8,
    U32,
    U64,
    I64,
}

impl IntType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I64 => "i64",
        }
    }
}

impl Display for IntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Types defined outside of codegen.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ExtType {
    Currency,
    RangeQueryTs,
    Timestamp,
    AlwaysTrue,
    /// serde_json::Value
    Value {
        is_ref: bool,
    },
}

impl ExtType {
    pub const fn ident(self) -> &'static str {
        match self {
            Self::Currency => "stripe_types::Currency",
            Self::RangeQueryTs => "stripe_types::RangeQueryTs",
            Self::Timestamp => "stripe_types::Timestamp",
            Self::AlwaysTrue => "stripe_types::AlwaysTrue",
            Self::Value { .. } => "serde_json::Value",
        }
    }
}
