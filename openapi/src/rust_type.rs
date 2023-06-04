use std::fmt::{Debug, Display, Formatter};

use crate::rust_object::{DeserDefault, RustObject, RustObjectData};
use crate::types::ComponentPath;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RustType {
    Object(RustObject),
    Simple(SimpleType),
    Ref(ComponentPath),
    ObjectId { path: ComponentPath, borrowed: bool },
    Compound(CompoundTypeKind, Box<RustType>),
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
        Self::Compound(CompoundTypeKind::Option, Box::new(typ))
    }

    pub fn boxed(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::Box, Box::new(typ))
    }

    pub fn list(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::List, Box::new(typ))
    }

    pub fn vec(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::Vec, Box::new(typ))
    }

    pub fn slice(typ: RustType) -> Self {
        Self::Compound(CompoundTypeKind::Slice, Box::new(typ))
    }

    pub fn as_id_path(&self) -> Option<&ComponentPath> {
        match self {
            RustType::Object(_) => None,
            RustType::Simple(_) => None,
            RustType::Ref(_) => None,
            RustType::ObjectId { path, .. } => Some(path),
            RustType::Compound(_, inner) => inner.as_id_path(),
        }
    }

    pub const fn is_option(&self) -> bool {
        matches!(self, Self::Compound(CompoundTypeKind::Option, _))
    }

    /// Can this type derive `Copy`?
    pub fn is_copy(&self) -> bool {
        match self {
            RustType::Object(obj) => obj.data.is_copy(),
            RustType::Simple(typ) => typ.is_copy(),
            RustType::Ref(_) => false,
            RustType::ObjectId { borrowed, .. } => *borrowed,
            RustType::Compound(kind, typ) => match kind {
                CompoundTypeKind::List
                | CompoundTypeKind::Expandable
                | CompoundTypeKind::Vec
                | CompoundTypeKind::Box => false,
                CompoundTypeKind::Slice => true,
                _ => typ.is_copy(),
            },
        }
    }

    pub fn has_borrow(&self) -> bool {
        match self {
            RustType::Object(obj) => obj.data.has_borrow(),
            RustType::Simple(typ) => typ.is_borrowed(),
            RustType::Ref(_) => false,
            RustType::ObjectId { borrowed, .. } => *borrowed,
            RustType::Compound(kind, inner) => {
                if kind.has_borrow() {
                    return true;
                }
                inner.has_borrow()
            }
        }
    }

    pub fn from_ref(reference: &str) -> Self {
        let path = ComponentPath::from_reference(reference);
        Self::Ref(path)
    }

    /// Should we box this type?
    pub fn should_box(&self) -> bool {
        match self {
            // Without a box, we end up with an infinitely sized type
            Self::Ref(path) => path.as_ref() == "api_errors",
            _ => false,
        }
    }

    pub fn into_nullable(self) -> Self {
        match self {
            Self::Compound(CompoundTypeKind::List | CompoundTypeKind::Option, _) => self,
            _ => Self::option(self),
        }
    }

    pub fn as_skip_serializing(&self) -> Option<&'static str> {
        match self {
            Self::Compound(CompoundTypeKind::Option, _) => Some("Option::is_none"),
            _ => None,
        }
    }

    pub fn as_reference_path(&self) -> Option<&ComponentPath> {
        match self {
            Self::Compound(_, typ) => typ.as_reference_path(),
            Self::Ref(path) => Some(path),
            _ => None,
        }
    }

    pub fn as_rust_obj(&self) -> Option<&RustObject> {
        match self {
            Self::Object(obj) => Some(obj),
            Self::Simple(_) | Self::Ref(_) | Self::ObjectId { .. } => None,
            Self::Compound(_, typ) => typ.as_rust_obj(),
        }
    }

    pub fn into_rust_obj(self) -> Option<RustObject> {
        match self {
            Self::Object(obj) => Some(obj),
            Self::Simple(_) | Self::Ref(_) | Self::ObjectId { .. } => None,
            Self::Compound(_, typ) => typ.into_rust_obj(),
        }
    }

    pub fn as_rust_obj_data(&self) -> Option<&RustObjectData> {
        self.as_rust_obj().map(|r| &r.data)
    }

    pub fn as_deser_default(&self) -> Option<DeserDefault> {
        match self {
            Self::Simple(SimpleType::Bool)
            | Self::Compound(CompoundTypeKind::Vec | CompoundTypeKind::List, _) => {
                Some(DeserDefault::Default)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CompoundTypeKind {
    List,
    Vec,
    Slice,
    Expandable,
    Option,
    Box,
}

impl CompoundTypeKind {
    pub const fn has_borrow(self) -> bool {
        matches!(self, Self::Slice)
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
        match self {
            Self::Bool | SimpleType::Float | SimpleType::Int(_) | SimpleType::Str => true,
            Self::String => false,
            Self::Ext(typ) => typ.is_copy(),
        }
    }

    pub const fn is_borrowed(self) -> bool {
        match self {
            SimpleType::Str => true,
            SimpleType::Ext(typ) => typ.is_borrowed(),
            _ => false,
        }
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
        if let Self::Ext(ext) = self {
            Some(ext.import_from())
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
    Metadata { borrowed: bool },
    Timestamp,
}

impl ExtType {
    pub const fn import_from(self) -> &'static str {
        match self {
            Self::RangeQueryTs | Self::Metadata { .. } | Self::Timestamp | Self::Currency => {
                "crate"
            }
        }
    }

    pub const fn ident(self) -> &'static str {
        match self {
            Self::Currency => "Currency",
            Self::RangeQueryTs => "RangeQueryTs",
            Self::Timestamp => "Timestamp",
            Self::Metadata { .. } => "Metadata",
        }
    }

    /// Does the corresponding type implement `Copy`?
    pub const fn is_copy(self) -> bool {
        matches!(
            self,
            Self::Currency
                | Self::RangeQueryTs
                | Self::Timestamp
                | Self::Metadata { borrowed: true }
        )
    }

    pub const fn is_borrowed(self) -> bool {
        matches!(self, Self::Metadata { borrowed: true })
    }
}
