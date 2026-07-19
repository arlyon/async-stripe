use serde::Serialize;

use crate::Object;

/// An id or object.
///
/// By default stripe will return an id for most fields, but if more detail is
/// necessary the `expand` parameter can be provided to ask for the id to be
/// loaded as an object instead.
///
/// For more details see <https://stripe.com/docs/api/expanding_objects>.
#[derive(Clone, Debug, Serialize)] // TODO: Implement deserialize by hand for better error messages with `serde` enabled
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[serde(untagged)]
pub enum Expandable<T: Object> {
    /// Just the object id.
    Id(T::Id),
    /// The entire object.
    Object(Box<T>),
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
    /// Returns true if we have the object.
    pub fn is_object(&self) -> bool {
        match self {
            Expandable::Id(_) => false,
            Expandable::Object(_) => true,
        }
    }

    /// If the `Expandable` is an object, return that, otherwise return None.
    pub fn as_object(&self) -> Option<&T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(obj),
        }
    }

    /// If the `Expandable` is an object, return that by taking ownership, otherwise return None.
    pub fn into_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(*obj),
        }
    }

    /// Take ownership of the object's id.
    pub fn into_id(self) -> T::Id {
        match self {
            Expandable::Id(id) => id,
            Expandable::Object(obj) => obj.into_id(),
        }
    }

    /// Extract the object's id.
    pub fn id(&self) -> &T::Id {
        match self {
            Expandable::Id(id) => id,
            Expandable::Object(obj) => obj.id(),
        }
    }
}

#[doc(hidden)]
mod miniserde {
    use stripe_miniserde::de::Visitor;
    use stripe_miniserde::{Deserialize, make_place};

    use crate::{Expandable, FromCursor, Object};
    make_place!(Place);

    impl<T> Deserialize for Expandable<T>
    where
        T: Object + Deserialize,
    {
        const WANTS_RAW: bool = true;

        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl<T> Visitor for Place<Expandable<T>>
    where
        T: Object + Deserialize,
    {
        fn wants_raw(&self) -> bool {
            true
        }

        fn raw(&mut self, bytes: &str) -> stripe_miniserde::Result<()> {
            let trimmed = bytes.trim_ascii_start();
            let first = trimmed.as_bytes().first().copied().ok_or(stripe_miniserde::Error)?;
            match first {
                b'"' => {
                    let s: String = stripe_miniserde::json::from_str(trimmed)?;
                    let id = T::Id::from_cursor(&s).ok_or(stripe_miniserde::Error)?;
                    self.out = Some(Expandable::Id(id));
                }
                b'{' => {
                    let parsed: T = stripe_miniserde::json::from_str(trimmed)?;
                    self.out = Some(Expandable::Object(Box::new(parsed)));
                }
                _ => return Err(stripe_miniserde::Error),
            }
            Ok(())
        }
    }
}
