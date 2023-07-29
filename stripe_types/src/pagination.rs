use serde::{Deserialize, Serialize};

/// Implemented by types which represent stripe objects.
pub trait Object {
    /// The canonical id type for this object.
    type Id;
    /// The id of the object.
    fn id(&self) -> Self::Id;
}

/// The `Expand` struct is used to serialize `expand` arguments in retrieve and list apis.
#[doc(hidden)]
#[derive(Serialize)]
pub struct Expand<'a> {
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl Expand<'_> {
    pub(crate) fn is_empty(expand: &[&str]) -> bool {
        expand.is_empty()
    }
}

/// An id or object.
///
/// By default stripe will return an id for most fields, but if more detail is
/// necessary the `expand` parameter can be provided to ask for the id to be
/// loaded as an object instead:
///
/// ```rust,ignore
/// Charge::retrieve(&client, &charge_id, &["invoice.customer"])
/// ```
///
/// For more details see <https://stripe.com/docs/api/expanding_objects>.
#[derive(Clone, Debug, Serialize, Deserialize)] // TODO: Implement deserialize by hand for better error messages
#[serde(untagged)]
pub enum Expandable<T: Object> {
    Id(T::Id),
    Object(Box<T>),
}

impl<T> Expandable<T>
where
    T: Object,
    T::Id: Clone + Default,
{
    pub fn id(&self) -> T::Id {
        match self {
            Expandable::Id(id) => id.clone(),
            Expandable::Object(obj) => obj.id(),
        }
    }
}

impl<T: Object> Default for Expandable<T>
where
    T::Id: Default,
{
    fn default() -> Self {
        Expandable::Id(Default::default())
    }
}

impl<T: Object> Expandable<T> {
    pub fn is_object(&self) -> bool {
        match self {
            Expandable::Id(_) => false,
            Expandable::Object(_) => true,
        }
    }

    pub fn as_object(&self) -> Option<&T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(obj),
        }
    }

    pub fn into_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(*obj),
        }
    }
}

/// Implemented by types which support cursor-based pagination,
/// typically with an id, allowing them to be fetched using a `List`
/// returned by the corresponding "list" api request.
pub trait Paginate {
    type Cursor: AsCursor;
    fn cursor(&self) -> Self::Cursor;
}

pub trait AsCursor: AsRef<str> {}

impl<'a> AsCursor for &'a str {}
impl AsCursor for String {}

impl<T> Paginate for T
where
    T: Object,
    T::Id: AsCursor,
{
    type Cursor = T::Id;
    fn cursor(&self) -> Self::Cursor {
        self.id()
    }
}

pub trait Paginable {
    type O: Object + Send;
    fn set_last(&mut self, item: &Self::O);
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
