use std::fmt::{Display, Formatter, Write};

use crate::components::PathInfo;
use crate::rust_type::{MapKey, SimpleType};
use crate::types::RustIdent;
use crate::STRIPE_TYPES;

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
    SearchList(Box<PrintableType>),
    Vec(Box<PrintableType>),
    Slice(Box<PrintableType>),
    Expandable(Box<PrintableType>),
    Option(Box<PrintableType>),
    Box(Box<PrintableType>),
    Map { key: MapKey, value: Box<PrintableType>, is_ref: bool },
}

impl Display for PrintableWithLifetime<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Some(lifetime) = self.lifetime else {
            if self.impl_into {
                return write!(f, "impl Into<{}>", self.typ);
            } else {
                return write!(f, "{}", self.typ);
            }
        };

        use PrintableContainer::*;
        use PrintableType::*;
        match &self.typ {
            QualifiedPath { path, has_ref, is_ref, ident } => {
                let full_path = PathWithIdent::new(path.as_ref(), ident);
                if *is_ref {
                    write!(f, "&{lifetime} ")?;
                } else if self.impl_into {
                    write!(f, "impl Into<")?;
                }
                if *has_ref {
                    write!(f, "{full_path}<{lifetime}>")?;
                } else {
                    write!(f, "{full_path}")?;
                }
                if !*is_ref && self.impl_into {
                    write!(f, ">")?;
                }
                Ok(())
            }
            Simple(typ) => {
                let is_ref = typ.is_reference();
                if is_ref {
                    write!(f, "&{lifetime} ")?;
                } else if self.impl_into {
                    write!(f, "impl Into<")?;
                }
                f.write_str(typ.ident())?;
                if !is_ref && self.impl_into {
                    write!(f, ">")?;
                }
                Ok(())
            }
            Container(typ) => match typ {
                List(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    if self.impl_into {
                        write!(f, "impl Into<")?;
                    }
                    write!(f, "{STRIPE_TYPES}::List<{inner}>")?;
                    if self.impl_into {
                        write!(f, ">")?;
                    }
                    Ok(())
                }
                SearchList(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    if self.impl_into {
                        write!(f, "impl Into<")?;
                    }
                    write!(f, "{STRIPE_TYPES}::SearchList<{inner}>")?;
                    if self.impl_into {
                        write!(f, ">")?;
                    }
                    Ok(())
                }
                Vec(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    if self.impl_into {
                        write!(f, "impl Into<")?;
                    }
                    write!(f, "Vec<{inner}>")?;
                    if self.impl_into {
                        write!(f, ">")?;
                    }
                    Ok(())
                }
                Slice(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "&{lifetime} [{inner}]")
                }
                Expandable(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    write!(f, "{STRIPE_TYPES}::Expandable<{inner}>")
                }
                Option(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    if self.impl_into {
                        write!(f, "impl Into<")?;
                    }
                    write!(f, "Option<{inner}>")?;
                    if self.impl_into {
                        write!(f, ">")?;
                    }
                    Ok(())
                }
                Box(inner) => {
                    let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                    if self.impl_into {
                        write!(f, "impl Into<")?;
                    }
                    write!(f, "Box<{inner}>")?;
                    if self.impl_into {
                        write!(f, ">")?;
                    }
                    Ok(())
                }
                Map { key, value, is_ref } => {
                    let value = PrintableWithLifetime::new(value, Some(lifetime));
                    if *is_ref {
                        write!(f, "&{lifetime} ")?;
                    } else if self.impl_into {
                        write!(f, "impl Into<")?;
                    }
                    write!(f, "std::collections::HashMap<{key}, {value}>")?;
                    if !*is_ref && self.impl_into {
                        write!(f, ">")?;
                    }
                    Ok(())
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
                let full_path = PathWithIdent::new(path.as_ref(), ident);
                if *is_ref {
                    f.write_char('&')?;
                }
                write!(f, "{full_path}")
            }
            Simple(typ) => {
                if typ.is_reference() {
                    f.write_char('&')?;
                }
                f.write_str(typ.ident())
            }
            Container(typ) => match typ {
                List(inner) => {
                    write!(f, "{STRIPE_TYPES}::List<{inner}>")
                }
                SearchList(inner) => {
                    write!(f, "{STRIPE_TYPES}::SearchList<{inner}>")
                }
                Vec(inner) => {
                    write!(f, "Vec<{inner}>")
                }
                Slice(inner) => {
                    write!(f, "&[{inner}]")
                }
                Expandable(inner) => {
                    write!(f, "{STRIPE_TYPES}::Expandable<{inner}>")
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

/// For now, we only support a single lifetime, so always creates `'a`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Lifetime;

impl Lifetime {
    pub fn as_str(self) -> &'static str {
        "'a"
    }

    pub fn as_param(self) -> &'static str {
        "<'a>"
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
    impl_into: bool,
}

impl<'a> PrintableWithLifetime<'a> {
    pub fn new(typ: &'a PrintableType, lifetime: Option<Lifetime>) -> Self {
        Self { typ, lifetime, impl_into: false }
    }
    pub fn impl_into(mut self) -> Self {
        self.impl_into = true;
        self
    }
}

#[derive(Debug, Copy, Clone)]
struct PathWithIdent<'a> {
    path: Option<&'a PathInfo>,
    ident: &'a RustIdent,
}

impl<'a> PathWithIdent<'a> {
    fn new(path: Option<&'a PathInfo>, ident: &'a RustIdent) -> Self {
        Self { path, ident }
    }
}

impl Display for PathWithIdent<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = self.path {
            write!(f, "{}::", path.krate.lib_name())?;
            if let Some(path) = &path.path {
                write!(f, "{path}::")?;
            }
        }

        write!(f, "{}", self.ident)
    }
}
