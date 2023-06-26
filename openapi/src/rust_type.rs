use std::fmt::{Debug, Display, Formatter};

use crate::rust_object::{DeserDefault, ObjectMetadata, RustObject};
use crate::types::{ComponentPath, RustIdent};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RefType {
    Component(ComponentPath),
    ObjectId(ComponentPath),
    Types(RustIdent),
    IntraFile(RustIdent),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustType {
    Object(RustObject, ObjectMetadata),
    Simple(SimpleType),
    Ref { typ: RefType, borrowed: bool, has_borrow: bool, is_copy: bool },
    Compound(CompoundType),
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
        Self::Compound(CompoundType::Option(Box::new(typ)))
    }

    pub fn boxed(typ: RustType) -> Self {
        Self::Compound(CompoundType::Box(Box::new(typ)))
    }

    pub fn list(typ: RustType) -> Self {
        Self::Compound(CompoundType::List(Box::new(typ)))
    }

    pub fn expandable(typ: RustType) -> Self {
        Self::Compound(CompoundType::Expandable(Box::new(typ)))
    }

    pub fn str_map(value: RustType, borrowed: bool) -> Self {
        Self::Compound(CompoundType::Map { key: MapKey::String, value: Box::new(value), borrowed })
    }

    pub fn currency_map(value: RustType, borrowed: bool) -> Self {
        Self::Compound(CompoundType::Map {
            key: MapKey::Currency,
            value: Box::new(value),
            borrowed,
        })
    }

    pub fn vec(typ: RustType) -> Self {
        Self::Compound(CompoundType::Vec(Box::new(typ)))
    }

    pub fn slice(typ: RustType) -> Self {
        Self::Compound(CompoundType::Slice(Box::new(typ)))
    }

    pub fn as_id_path(&self) -> Option<&ComponentPath> {
        match self {
            RustType::Ref { typ: RefType::ObjectId(path), .. } => Some(path),
            RustType::Compound(typ) => typ.value_typ().as_id_path(),
            _ => None,
        }
    }

    pub const fn is_option(&self) -> bool {
        matches!(self, Self::Compound(CompoundType::Option(_)))
    }

    /// Can this type derive `Copy`?
    pub fn is_copy(&self) -> bool {
        match self {
            RustType::Object(obj, ..) => obj.is_copy(),
            RustType::Simple(typ) => typ.is_copy(),
            RustType::Ref { is_copy, .. } => *is_copy,
            RustType::Compound(typ) => match typ {
                CompoundType::List(_) => false,
                CompoundType::Vec(_) => false,
                CompoundType::Slice(_) => true,
                CompoundType::Expandable(_) => false,
                CompoundType::Option(inner) => inner.is_copy(),
                CompoundType::Box(inner) => inner.is_copy(),
                CompoundType::Map { borrowed, .. } => *borrowed,
            },
        }
    }

    pub fn has_borrow(&self) -> bool {
        match self {
            RustType::Object(obj, ..) => obj.has_borrow(),
            RustType::Simple(typ) => typ.is_borrowed(),
            RustType::Ref { has_borrow, .. } => *has_borrow,
            RustType::Compound(typ) => match typ {
                CompoundType::Slice(_) => true,
                CompoundType::Map { borrowed: true, .. } => true,
                _ => typ.value_typ().has_borrow(),
            },
        }
    }

    pub fn component_path(path: ComponentPath) -> Self {
        Self::Ref {
            typ: RefType::Component(path),
            borrowed: false,
            has_borrow: false,
            is_copy: false,
        }
    }

    pub fn object_id(id_path: ComponentPath, borrowed: bool) -> Self {
        Self::Ref { typ: RefType::ObjectId(id_path), borrowed, is_copy: false, has_borrow: false }
    }

    pub fn into_nullable(self) -> Self {
        match self {
            Self::Compound(CompoundType::List(_)) | Self::Compound(CompoundType::Option(_)) => self,
            _ => Self::option(self),
        }
    }

    pub fn as_skip_serializing(&self) -> Option<&'static str> {
        match self {
            Self::Compound(CompoundType::Option(_)) => Some("Option::is_none"),
            _ => None,
        }
    }

    pub fn as_reference_path(&self) -> Option<&ComponentPath> {
        match self {
            Self::Compound(typ) => typ.value_typ().as_reference_path(),
            Self::Ref { typ: RefType::Component(path), .. } => Some(path),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<(&RustObject, &ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            Self::Simple(_) | Self::Ref { .. } => None,
            Self::Compound(typ) => typ.value_typ().as_object(),
        }
    }

    pub fn into_object(self) -> Option<(RustObject, ObjectMetadata)> {
        match self {
            Self::Object(obj, meta) => Some((obj, meta)),
            Self::Simple(_) | Self::Ref { .. } => None,
            Self::Compound(typ) => typ.into_value_typ().into_object(),
        }
    }

    pub fn as_rust_object(&self) -> Option<&RustObject> {
        self.as_object().map(|r| r.0)
    }

    pub fn as_deser_default(&self) -> Option<DeserDefault> {
        match self {
            Self::Simple(SimpleType::Bool)
            | Self::Compound(CompoundType::Vec(_))
            | Self::Compound(CompoundType::List(_)) => Some(DeserDefault::Default),
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CompoundType {
    List(Box<RustType>),
    Vec(Box<RustType>),
    Slice(Box<RustType>),
    Expandable(Box<RustType>),
    Option(Box<RustType>),
    Box(Box<RustType>),
    Map { key: MapKey, value: Box<RustType>, borrowed: bool },
}

impl CompoundType {
    pub fn value_typ(&self) -> &RustType {
        match self {
            CompoundType::List(typ) => typ,
            CompoundType::Vec(typ) => typ,
            CompoundType::Slice(typ) => typ,
            CompoundType::Expandable(typ) => typ,
            CompoundType::Option(typ) => typ,
            CompoundType::Box(typ) => typ,
            CompoundType::Map { value, .. } => value,
        }
    }

    pub fn into_value_typ(self) -> RustType {
        let res = match self {
            CompoundType::List(typ) => typ,
            CompoundType::Vec(typ) => typ,
            CompoundType::Slice(typ) => typ,
            CompoundType::Expandable(typ) => typ,
            CompoundType::Option(typ) => typ,
            CompoundType::Box(typ) => typ,
            CompoundType::Map { value, .. } => value,
        };
        *res
    }

    pub fn value_typ_mut(&mut self) -> &mut RustType {
        match self {
            CompoundType::List(typ) => typ,
            CompoundType::Vec(typ) => typ,
            CompoundType::Slice(typ) => typ,
            CompoundType::Expandable(typ) => typ,
            CompoundType::Option(typ) => typ,
            CompoundType::Box(typ) => typ,
            CompoundType::Map { value, .. } => value,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SimpleType {
    Bool,
    Float,
    Str,
    String,
    Int(IntType),
    Ext(ExtType),
}

impl SimpleType {
    /// Does this type implement `Copy`?
    pub const fn is_copy(self) -> bool {
        !matches!(self, Self::String)
    }

    pub const fn is_borrowed(self) -> bool {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ExtType {
    Currency,
    RangeQueryTs,
    Timestamp,
}

impl ExtType {
    pub const fn ident(self) -> &'static str {
        match self {
            Self::Currency => "Currency",
            Self::RangeQueryTs => "RangeQueryTs",
            Self::Timestamp => "Timestamp",
        }
    }
}
