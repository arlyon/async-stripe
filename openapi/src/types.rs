use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

use heck::CamelCase;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RustIdent(String);

impl RustIdent {
    pub fn from_valid(val: String) -> Self {
        Self(val)
    }

    pub fn create(val: &str) -> Self {
        if val.contains('.') {
            Self(val.replace('.', "_").to_camel_case())
        } else {
            Self(val.to_camel_case())
        }
    }

    pub fn joined<T: Display, U: Display>(piece1: T, piece2: U) -> Self {
        Self::create(&format!("{piece1}_{piece2}"))
    }
}

impl Deref for RustIdent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for RustIdent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for RustIdent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ComponentPath(String);

impl ComponentPath {
    pub fn new(path: String) -> Self {
        Self(path)
    }

    pub fn from_reference(reference: &str) -> Self {
        let root_obj = reference.trim_start_matches("#/components/schemas/");
        Self::new(root_obj.into())
    }

    pub fn as_not_deleted(&self) -> Self {
        Self::new(self.0.trim_start_matches("deleted_").to_string())
    }

    pub fn is_deleted(&self) -> bool {
        self.starts_with("deleted_")
    }
}

// This is a bit silly...just done because `petgraph` prints graph labels using `Debug` so this
// is a hack to have the labels print more concisely while still using our `ComponentPath` newtype
impl Debug for ComponentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for ComponentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ComponentPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ComponentPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<str> for ComponentPath {
    fn borrow(&self) -> &str {
        &self.0
    }
}
