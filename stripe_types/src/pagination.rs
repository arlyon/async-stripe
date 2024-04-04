use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Implemented by types which represent stripe objects.
pub trait Object {
    /// The canonical id type for this object.
    type Id: AsCursorOpt;
    /// The id of the object.
    fn id(&self) -> &Self::Id;
}

pub trait AsCursorOpt {
    fn as_cursor_opt(&self) -> Option<&str>;
}

pub trait AsCursor {
    fn as_cursor(&self) -> &str;
}

impl AsCursor for smol_str::SmolStr {
    fn as_cursor(&self) -> &str {
        self.as_str()
    }
}

impl<T: AsCursor> AsCursorOpt for T {
    fn as_cursor_opt(&self) -> Option<&str> {
        Some(self.as_cursor())
    }
}

impl<T: AsCursor> AsCursorOpt for Option<T> {
    fn as_cursor_opt(&self) -> Option<&str> {
        self.as_ref().map(|id| id.as_cursor())
    }
}

/// A trait allowing `List<T>` and `SearchList<T>` to be treated the same. Not part of the
/// public API.
///
/// NB: this trait is designed specifically for `List` and `SearchList` and may not be sensible
/// in other cases. One gotcha is that `into_parts` and `from_parts` do not necessarily
/// round-trip, e.g. `SearchList<T>` will lose the `next_page` field since that
/// is not part of the shared list impl. We account for this by ensuring to call `update_params`
/// before breaking the `SearchList` into pieces.
#[doc(hidden)]
pub trait PaginableList: DeserializeOwned {
    /// Underlying single element type, e.g. `Account`
    type Data;

    /// Break into the shared parts list pagination requires
    fn into_parts(self) -> ListParts<Self::Data>;

    /// Reconstruct from the shared parts list pagination requires
    fn from_parts(parts: ListParts<Self::Data>) -> Self;

    /// Update the current parameter set, with `self` as the most
    /// recently fetched page.
    ///
    /// NB: this should also set `has_more` to `false` explicitly if we don't have a new cursor.
    /// (This seems redundant with `has_more` but is used to provide extra protection
    /// against any possibility where `has_more` is `true`, but the cursor is back to `None`,
    /// potentially leading to an infinite pagination loop).
    fn update_params(&mut self, params: &mut Value);
}

/// Specific list parts relied on by the client for pagination.
#[doc(hidden)]
#[derive(Debug)]
pub struct ListParts<T> {
    pub total_count: Option<u64>,
    pub url: String,
    pub data: Vec<T>,
    pub has_more: bool,
}

impl<T: Object + DeserializeOwned> PaginableList for List<T> {
    type Data = T;

    fn into_parts(self) -> ListParts<Self::Data> {
        ListParts {
            total_count: self.total_count,
            url: self.url,
            data: self.data,
            has_more: self.has_more,
        }
    }

    fn from_parts(parts: ListParts<Self::Data>) -> Self {
        Self {
            data: parts.data,
            has_more: parts.has_more,
            total_count: parts.total_count,
            url: parts.url,
        }
    }

    fn update_params(&mut self, params: &mut Value) {
        if let Some(new_cursor) = self.data.last().and_then(|l| l.id().as_cursor_opt()) {
            params["starting_after"] = Value::String(new_cursor.into());
        } else {
            self.has_more = false;
        }
    }
}

/// A single page of a cursor-paginated list of an object.
///
/// For more details, see <https://stripe.com/docs/api/pagination>
#[derive(Debug, Serialize, Deserialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            data: self.data.clone(),
            has_more: self.has_more,
            total_count: self.total_count,
            url: self.url.clone(),
        }
    }
}

/// A single page of a cursor-paginated list of a search object.
///
/// For more details, see <https://stripe.com/docs/api/pagination/search>
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchList<T> {
    pub url: String,
    pub has_more: bool,
    pub data: Vec<T>,
    pub next_page: Option<String>,
    pub total_count: Option<u64>,
}

impl<T: Clone> Clone for SearchList<T> {
    fn clone(&self) -> Self {
        SearchList {
            data: self.data.clone(),
            has_more: self.has_more,
            total_count: self.total_count,
            url: self.url.clone(),
            next_page: self.next_page.clone(),
        }
    }
}

impl<T: DeserializeOwned> PaginableList for SearchList<T> {
    type Data = T;

    /// NB: here we lose `next_page`, so we should be sure to `update_params`
    /// before calling this.
    fn into_parts(self) -> ListParts<Self::Data> {
        ListParts {
            total_count: self.total_count,
            url: self.url,
            data: self.data,
            has_more: self.has_more,
        }
    }

    fn from_parts(parts: ListParts<Self::Data>) -> Self {
        Self {
            url: parts.url,
            has_more: parts.has_more,
            data: parts.data,
            next_page: None,
            total_count: parts.total_count,
        }
    }

    fn update_params(&mut self, params: &mut Value) {
        if let Some(next_page) = self.next_page.take() {
            params["page"] = Value::String(next_page);
        } else {
            self.has_more = false;
        }
    }
}
