#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedSubscriptionItem {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_core::subscription_item::SubscriptionItemId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedSubscriptionItemObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedSubscriptionItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedSubscriptionItemObject {
    SubscriptionItem,
}

impl DeletedSubscriptionItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SubscriptionItem => "subscription_item",
        }
    }
}

impl std::str::FromStr for DeletedSubscriptionItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "subscription_item" => Ok(Self::SubscriptionItem),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedSubscriptionItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedSubscriptionItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedSubscriptionItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedSubscriptionItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for DeletedSubscriptionItemObject")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedSubscriptionItemObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedSubscriptionItemObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedSubscriptionItemObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedSubscriptionItem {
    type Id = stripe_core::subscription_item::SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
