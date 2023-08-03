#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedSubscriptionItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::subscription_item::SubscriptionItemId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedSubscriptionItemObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeletedSubscriptionItemObject {
    SubscriptionItem,
}

impl DeletedSubscriptionItemObject {
    pub fn as_str(self) -> &'static str {
        use DeletedSubscriptionItemObject::*;
        match self {
            SubscriptionItem => "subscription_item",
        }
    }
}

impl std::str::FromStr for DeletedSubscriptionItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedSubscriptionItemObject::*;
        match s {
            "subscription_item" => Ok(SubscriptionItem),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeletedSubscriptionItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DeletedSubscriptionItemObject"))
    }
}
impl stripe_types::Object for DeletedSubscriptionItem {
    type Id = stripe_types::subscription_item::SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
