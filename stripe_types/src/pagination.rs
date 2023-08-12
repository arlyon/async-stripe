use serde::{Deserialize, Serialize};

/// Implemented by types which represent stripe objects.
pub trait Object {
    /// The canonical id type for this object.
    type Id;
    /// The id of the object.
    fn id(&self) -> Option<&str>;
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

impl<T> Default for List<T> {
    fn default() -> Self {
        List { data: Vec::new(), has_more: false, total_count: None, url: String::new() }
    }
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
