/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing List Items](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ValueListItem {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: crate::radar::value_list_item::RadarValueListItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ValueListItemObject,
    /// The value of the item.
    pub value: String,
    /// The identifier of the value list this item belongs to.
    pub value_list: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ValueListItem {
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
pub enum ValueListItemObject {
    #[serde(rename = "radar.value_list_item")]
    RadarValueListItem,
}

impl ValueListItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RadarValueListItem => "radar.value_list_item",
        }
    }
}

impl AsRef<str> for ValueListItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ValueListItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for ValueListItem {
    type Id = crate::radar::value_list_item::RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(RadarValueListItemId);
pub mod deleted;
pub mod requests;
pub use deleted::DeletedValueListItem;
