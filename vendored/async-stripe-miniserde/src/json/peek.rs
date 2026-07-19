//! Peek helpers for polymorphic dispatch over JSON objects.
//!
//! Used together with the [`Visitor::wants_raw`] / [`Visitor::raw`] machinery:
//! a `raw()` body receives the bytes of one JSON object, peeks the tag, and
//! re-parses the same bytes via the variant's [`Deserialize`] impl.
//!
//! [`Visitor::wants_raw`]: crate::de::Visitor::wants_raw
//! [`Visitor::raw`]: crate::de::Visitor::raw

use alloc::boxed::Box;
use alloc::string::String;

use crate::Result;
use crate::de::{Deserialize, Map, Visitor};
use crate::json::from_str_with_visitor;

/// Walk a JSON object and return the value of its `"object"` key as a `String`,
/// or `None` if the input isn't an object or the key is missing/non-string.
pub fn peek_object_tag(bytes: &str) -> Option<String> {
    let mut tag: Option<String> = None;
    let mut visitor = TagVisitor { tag: &mut tag };
    let _ = from_str_with_visitor(bytes, &mut visitor);
    tag
}

/// Walk a JSON object and return the value of its `"deleted"` key as a `bool`,
/// defaulting to `false` if the key is missing or not a boolean.
pub fn peek_deleted_flag(bytes: &str) -> bool {
    let mut deleted: Option<bool> = None;
    let mut visitor = DeletedFlagVisitor { deleted: &mut deleted };
    let _ = from_str_with_visitor(bytes, &mut visitor);
    deleted.unwrap_or(false)
}

struct TagVisitor<'a> {
    tag: &'a mut Option<String>,
}

impl Visitor for TagVisitor<'_> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(TagMap { tag: &mut *self.tag }))
    }
}

struct TagMap<'a> {
    tag: &'a mut Option<String>,
}

impl Map for TagMap<'_> {
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        if k == "object" && self.tag.is_none() {
            Ok(<String as Deserialize>::begin(self.tag))
        } else {
            Ok(<dyn Visitor>::ignore())
        }
    }
    fn finish(&mut self) -> Result<()> {
        Ok(())
    }
}

struct DeletedFlagVisitor<'a> {
    deleted: &'a mut Option<bool>,
}

impl Visitor for DeletedFlagVisitor<'_> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(DeletedFlagMap { deleted: &mut *self.deleted }))
    }
}

struct DeletedFlagMap<'a> {
    deleted: &'a mut Option<bool>,
}

impl Map for DeletedFlagMap<'_> {
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        if k == "deleted" && self.deleted.is_none() {
            Ok(<bool as Deserialize>::begin(self.deleted))
        } else {
            Ok(<dyn Visitor>::ignore())
        }
    }
    fn finish(&mut self) -> Result<()> {
        Ok(())
    }
}
