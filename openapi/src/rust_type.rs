use std::fmt::{Debug, Display, Formatter};

use crate::rust_object::{DeserDefault, ObjectMetadata, RustObject};
use crate::types::{ComponentPath, RustIdent};

/// A path to a type defined elsewhere.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PathToType {
    /// A top-level component.
    Component(ComponentPath),
    /// The id for a top-level component.
    ObjectId(ComponentPath),
    /// A type generated in `stripe_types`, so printed as `stripe_types::{ident}`
    Types(RustIdent),
    /// A reference to a type in the same file (so not import needed).
    IntraFile(RustIdent),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustType {
    Object(RustObject, ObjectMetadata),
    /// Either a Rust defined scalar type, or a type predefined in `stripe_types`.
    Simple(SimpleType),
    Path {
        path: PathToType,
        is_ref: bool,
        /// Does the underlying type pointed to have references (e.g. when printing do
        /// we need a lifetime?)
        has_reference: bool,
        /// Does the underlying type pointed to implement `Copy`?
        is_copy: bool,
    },
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

    pub fn option(typ: RustType) -> Self {
        Self::Container(Container::Option(Box::new(typ)))
    }

    pub fn boxed(typ: RustType) -> Self {
        Self::Container(Container::Box(Box::new(typ)))
    }

    pub fn list(typ: RustType) -> Self {
        Self::Container(Container::List(Box::new(typ)))
    }

    /// Construct an `Expandable<{typ}>`.
    pub fn expandable(typ: RustType) -> Self {
        Self::Container(Container::Expandable(Box::new(typ)))
    }

    /// Construct a `HashMap<String, {typ}>`.
    pub fn str_map(value: RustType, is_ref: bool) -> Self {
        Self::Container(Container::Map { key: MapKey::String, value: Box::new(value), is_ref })
    }

    /// Construct a `HashMap<Currency, {typ}>`.
    pub fn currency_map(typ: RustType, is_ref: bool) -> Self {
        Self::Container(Container::Map { key: MapKey::Currency, value: Box::new(typ), is_ref })
    }

    /// Construct a `Vec<{typ}>`.
    pub fn vec(typ: RustType) -> Self {
        Self::Container(Container::Vec(Box::new(typ)))
    }

    /// Construct a `&[{typ}]`
    pub fn slice(typ: RustType) -> Self {
        Self::Container(Container::Slice(Box::new(typ)))
    }

    pub fn as_id_path(&self) -> Option<&ComponentPath> {
        match self {
            RustType::Path { path: PathToType::ObjectId(path), .. } => Some(path),
            RustType::Container(typ) => typ.value_typ().as_id_path(),
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
            RustType::Object(obj, ..) => obj.is_copy(),
            RustType::Simple(typ) => typ.is_copy(),
            RustType::Path { is_copy, .. } => *is_copy,
            RustType::Container(typ) => match typ {
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
            RustType::Object(obj, ..) => obj.has_reference(),
            RustType::Simple(typ) => typ.is_reference(),
            RustType::Path { has_reference, .. } => *has_reference,
            RustType::Container(typ) => match typ {
                Container::Slice(_) => true,
                Container::Map { is_ref: true, .. } => true,
                _ => typ.value_typ().has_reference(),
            },
        }
    }

    pub fn component_path(path: ComponentPath) -> Self {
        Self::Path {
            path: PathToType::Component(path),
            has_reference: false,
            is_ref: false,
            is_copy: false,
        }
    }

    pub fn object_id(id_path: ComponentPath, is_ref: bool) -> Self {
        Self::Path {
            path: PathToType::ObjectId(id_path),
            is_ref,
            is_copy: false,
            has_reference: false,
        }
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

    pub fn into_object(self) -> Option<(RustObject, ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            Self::Simple(_) | Self::Path { .. } => None,
            Self::Container(typ) => typ.into_value_typ().into_object(),
        }
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
            MapKey::String => "String",
            MapKey::Currency => "stripe_types::Currency",
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
        !matches!(self, Self::String)
    }

    /// Is this type a reference?
    pub const fn is_reference(self) -> bool {
        matches!(self, SimpleType::Str)
    }

    pub const fn ident(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Float => "f64",
            Self::String => "String",
            Self::Str => "str",
            Self::Int(typ) => typ.as_str(),
            Self::Ext(ext) => ext.ident(),
        }
    }

    pub const fn import_from(self) -> Option<&'static str> {
        if matches!(self, Self::Ext(_)) {
            Some("stripe_types")
        } else {
            None
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

/// Types defined outside of codegen, in the `stripe_types` crate.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ExtType {
    Currency,
    RangeQueryTs,
    Timestamp,
    AlwaysTrue,
}

impl ExtType {
    pub const fn ident(self) -> &'static str {
        match self {
            Self::Currency => "Currency",
            Self::RangeQueryTs => "RangeQueryTs",
            Self::Timestamp => "Timestamp",
            Self::AlwaysTrue => "AlwaysTrue",
        }
    }
}
