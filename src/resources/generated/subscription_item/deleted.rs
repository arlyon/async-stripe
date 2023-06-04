#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedSubscriptionItem {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: crate::subscription_item::SubscriptionItemId,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for DeletedSubscriptionItem {
    type Id = crate::subscription_item::SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
