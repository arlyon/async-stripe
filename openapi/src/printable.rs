use std::fmt::{Display, Formatter, Write};

use crate::components::PathInfo;
use crate::rust_object::{DeserDefault, StructField, Visibility};
use crate::rust_type::{MapKey, SimpleType};
use crate::types::RustIdent;

/// A `RustType` we can implement `Display` for. This requires converting the portions of `RustType`
/// we do not know how to print into other variants. For example, `RustType::Object` must be converted
/// to actual qualified paths and `RustType::Path` must be given information about crate +
/// path to import from.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PrintableType {
    QualifiedPath {
        path: Option<PathInfo>,
        ident: RustIdent,
        /// Does this contain references, e.g. do we need a lifetime when generating this type?
        has_ref: bool,
        /// Is this a reference?
        is_ref: bool,
    },
    Simple(SimpleType),
    Container(PrintableContainer),
}

/// A direct analogue of `Container`, but with `RustType` replaced by `PrintableType`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PrintableContainer {
    List(Box<PrintableType>),
    Vec(Box<PrintableType>),
    Slice(Box<PrintableType>),
    Expandable(Box<PrintableType>),
    Option(Box<PrintableType>),
    Box(Box<PrintableType>),
    Map { key: MapKey, value: Box<PrintableType>, is_ref: bool },
}

/// A direct analogue of `EnumVariant`, but with `RustType` replaced by `PrintableType`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PrintableEnumVariant {
    pub variant: RustIdent,
    pub rust_type: Option<PrintableType>,
}

/// A (mostly) direct analogue of `StructField`, but with `RustType` replaced by `PrintableType`.
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
    pub vis: Visibility,
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
            vis: field.vis,
        }
    }
}

impl<'a> Display for PrintableWithLifetime<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Some(lifetime) = self.lifetime else {
            return write!(f, "{}", self.typ);
        };

        use PrintableContainer::*;
        use PrintableType::*;
        match &self.typ {
            QualifiedPath { path, has_ref, is_ref, ident } => {
                let full_path = PathWithIdent::new(path, ident);
                if *is_ref {
                    write!(f, "&{lifetime} ")?;
                }
                if *has_ref {
                    write!(f, "{full_path}<{lifetime}>")
                } else {
                    write!(f, "{full_path}")
                }
            }
            Simple(typ) => {
                if typ.is_reference() {
                    write!(f, "&{lifetime} ")?;
                }
                if let Some(import) = typ.import_from() {
                    write!(f, "{import}::")?;
                }
                f.write_str(typ.ident())
            }
            Container(typ) => match typ {
                List(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "stripe_types::List<{inner}>")
                }
                Vec(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "Vec<{inner}>")
                }
                Slice(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "&{lifetime} [{inner}]")
                }
                Expandable(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "stripe_types::Expandable<{inner}>")
                }
                Option(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "Option<{inner}>")
                }
                Box(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "Box<{inner}>")
                }
                Map { key, value, is_ref } => {
                    let value = PrintableWithLifetime::new(value, Some(lifetime));
                    if *is_ref {
                        write!(f, "&{lifetime} ")?;
                    }
                    write!(f, "std::collections::HashMap<{key}, {value}>")
                }
            },
        }
    }
}

impl Display for PrintableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PrintableContainer::*;
        use PrintableType::*;
        match self {
            QualifiedPath { path, is_ref, ident, .. } => {
                let full_path = PathWithIdent::new(path, ident);
                if *is_ref {
                    f.write_char('&')?;
                }
                write!(f, "{full_path}")
            }
            Simple(typ) => {
                if typ.is_reference() {
                    f.write_char('&')?;
                }
                if let Some(import) = typ.import_from() {
                    write!(f, "{import}::")?;
                }
                f.write_str(typ.ident())
            }
            Container(typ) => match typ {
                List(inner) => {
                    write!(f, "stripe_types::List<{inner}>")
                }
                Vec(inner) => {
                    write!(f, "Vec<{inner}>")
                }
                Slice(inner) => {
                    write!(f, "&[{inner}]")
                }
                Expandable(inner) => {
                    write!(f, "stripe_types::Expandable<{inner}>")
                }
                Option(inner) => {
                    write!(f, "Option<{inner}>")
                }
                Box(inner) => {
                    write!(f, "Box<{inner}>")
                }
                Map { key, value, is_ref } => {
                    if *is_ref {
                        f.write_char('&')?;
                    }
                    write!(f, "std::collections::HashMap<{key}, {value}>")
                }
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Lifetime(&'static str);

impl Lifetime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }
}

impl Default for Lifetime {
    fn default() -> Self {
        Self("'a")
    }
}

impl Display for Lifetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Wrapper to implement `Display` on a combination of a lifetime and a type.
#[derive(Copy, Clone)]
pub struct PrintableWithLifetime<'a> {
    lifetime: Option<Lifetime>,
    typ: &'a PrintableType,
}

impl<'a> PrintableWithLifetime<'a> {
    pub fn new(typ: &'a PrintableType, lifetime: Option<Lifetime>) -> Self {
        Self { typ, lifetime }
    }
}

#[derive(Debug, Copy, Clone)]
struct PathWithIdent<'a> {
    path: &'a Option<PathInfo>,
    ident: &'a RustIdent,
}

impl<'a> PathWithIdent<'a> {
    fn new(path: &'a Option<PathInfo>, ident: &'a RustIdent) -> Self {
        Self { path, ident }
    }
}

impl<'a> Display for PathWithIdent<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = self.path {
            if let Some(krate) = path.krate {
                write!(f, "{}::", krate.name())?;
            }
            if let Some(path) = &path.path {
                write!(f, "{path}::")?;
            }
        }
        write!(f, "{}", self.ident)
    }
}
