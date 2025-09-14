use std::fmt::Debug;
use std::str::FromStr;

use serde::{Serialize, Serializer};

#[doc(hidden)]
pub trait FromCursor {
    fn from_cursor(val: &str) -> Option<Self>
    where
        Self: Sized;
}

impl FromCursor for smol_str::SmolStr {
    fn from_cursor(val: &str) -> Option<Self> {
        Self::from_str(val).ok()
    }
}

impl<T: FromCursor> FromCursor for Option<T> {
    fn from_cursor(val: &str) -> Option<Self> {
        Some(T::from_cursor(val))
    }
}

#[doc(hidden)]
pub trait AsCursorOpt {
    fn as_cursor_opt(&self) -> Option<&str>;
}

#[doc(hidden)]
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

/// Implemented by types which represent stripe objects.
pub trait Object {
    /// The canonical id type for this object.
    type Id: AsCursorOpt + FromCursor;

    /// The id of the object.
    fn id(&self) -> &Self::Id;

    /// The owned id of the object.
    fn into_id(self) -> Self::Id;
}

/// A single page of a cursor-paginated list of an object.
///
/// For more details, see <https://stripe.com/docs/api/pagination>
#[derive(Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct List<T> {
    /// A page of data.
    pub data: Vec<T>,
    /// If true, making another request with our new url will yield more data.
    pub has_more: bool,
    /// The base endpoint we're targeting.
    pub url: String,
}

// Manually implementing this because we need to add the "object": "list" key. The workarounds
// mentioned in https://github.com/serde-rs/serde/issues/760 require adding a 1-enum variant, which has the downside
// that either this field is public (so consumers now have to add this meaningless field) or it is private (and
// we have a breaking change where `List` cannot be constructed with struct literal syntax)
impl<T: Serialize> Serialize for List<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut ser = serializer.serialize_struct("List", 5)?;
        ser.serialize_field("data", &self.data)?;
        ser.serialize_field("has_more", &self.has_more)?;
        ser.serialize_field("url", &self.url)?;
        ser.serialize_field("object", "list")?;
        ser.end()
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List { data: self.data.clone(), has_more: self.has_more, url: self.url.clone() }
    }
}

/// A single page of a cursor-paginated list of a search object.
///
/// For more details, see <https://stripe.com/docs/api/pagination/search>
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SearchList<T> {
    /// The base endpoint we're targeting.
    pub url: String,
    /// If true, making another request with our new url will yield more data.
    pub has_more: bool,
    /// A page of data.
    pub data: Vec<T>,
    /// The next url to query if we want to continue paginating.
    pub next_page: Option<String>,
    /// The total number of results.
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

#[doc(hidden)]
mod impl_deserialize {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Error};

    use crate::miniserde_helpers::FromValueOpt;
    use crate::{List, SearchList};
    make_place!(Place);

    impl<T: Deserialize> Deserialize for List<T> {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl<T: Deserialize> Visitor for Place<List<T>> {
        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(Box::new(ListBuilder {
                out: &mut self.out,
                data: Deserialize::default(),
                has_more: Deserialize::default(),
                url: Deserialize::default(),
            }))
        }
    }

    struct ListBuilder<'a, T> {
        out: &'a mut Option<List<T>>,
        data: Option<Vec<T>>,
        has_more: Option<bool>,
        url: Option<String>,
    }

    impl<T: Deserialize> Map for ListBuilder<'_, T> {
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "url" => Ok(Deserialize::begin(&mut self.url)),
                "data" => Ok(Deserialize::begin(&mut self.data)),
                "has_more" => Ok(Deserialize::begin(&mut self.has_more)),
                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn finish(&mut self) -> miniserde::Result<()> {
            let url = self.url.take().ok_or(Error)?;
            let data = self.data.take().ok_or(Error)?;
            let has_more = self.has_more.ok_or(Error)?;
            *self.out = Some(List { data, has_more, url });
            Ok(())
        }
    }

    impl<T: FromValueOpt> FromValueOpt for List<T> {
        fn from_value(v: Value) -> Option<Self> {
            let mut data: Option<Vec<T>> = None;
            let mut has_more: Option<bool> = None;
            let mut url: Option<String> = None;
            let Value::Object(obj) = v else {
                return None;
            };
            for (k, v) in obj {
                match k.as_str() {
                    "has_more" => has_more = Some(bool::from_value(v)?),
                    "data" => data = Some(FromValueOpt::from_value(v)?),
                    "url" => url = Some(FromValueOpt::from_value(v)?),
                    _ => {}
                }
            }
            Some(Self { data: data?, has_more: has_more?, url: url? })
        }
    }

    impl<T: Deserialize> Deserialize for SearchList<T> {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct SearchListBuilder<'a, T> {
        out: &'a mut Option<SearchList<T>>,
        data: Option<Vec<T>>,
        has_more: Option<bool>,
        total_count: Option<Option<u64>>,
        url: Option<String>,
        next_page: Option<Option<String>>,
    }

    impl<T: Deserialize> Visitor for Place<SearchList<T>> {
        fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
            Ok(Box::new(SearchListBuilder {
                out: &mut self.out,
                data: Deserialize::default(),
                has_more: Deserialize::default(),
                total_count: Deserialize::default(),
                url: Deserialize::default(),
                next_page: Deserialize::default(),
            }))
        }
    }

    impl<T: Deserialize> Map for SearchListBuilder<'_, T> {
        fn key(&mut self, k: &str) -> miniserde::Result<&mut dyn Visitor> {
            match k {
                "url" => Ok(Deserialize::begin(&mut self.url)),
                "data" => Ok(Deserialize::begin(&mut self.data)),
                "has_more" => Ok(Deserialize::begin(&mut self.has_more)),
                "total_count" => Ok(Deserialize::begin(&mut self.total_count)),
                "next_page" => Ok(Deserialize::begin(&mut self.next_page)),
                _ => Ok(<dyn Visitor>::ignore()),
            }
        }

        fn finish(&mut self) -> miniserde::Result<()> {
            let url = self.url.take().ok_or(Error)?;
            let data = self.data.take().ok_or(Error)?;
            let has_more = self.has_more.take().ok_or(Error)?;
            let total_count = self.total_count.take().ok_or(Error)?;
            let next_page = self.next_page.take().ok_or(Error)?;
            *self.out = Some(SearchList { data, has_more, total_count, url, next_page });
            Ok(())
        }
    }
}
