/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing List Items](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ValueListItem {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_types::radar::value_list_item::RadarValueListItemId,
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueListItemObject {
    RadarValueListItem,
}

impl ValueListItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RadarValueListItem => "radar.value_list_item",
        }
    }
}

impl std::str::FromStr for ValueListItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "radar.value_list_item" => Ok(Self::RadarValueListItem),

            _ => Err(()),
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
impl serde::Serialize for ValueListItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ValueListItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ValueListItemObject"))
    }
}
impl stripe_types::Object for ValueListItem {
    type Id = stripe_types::radar::value_list_item::RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RadarValueListItemId);
pub mod deleted;
pub use deleted::DeletedValueListItem;
