/// Value lists allow you to group values together which can then be referenced in rules.
///
/// Related guide: [Default Stripe lists](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ValueList {
    /// The name of the value list for use in rules.
    pub alias: String,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who created this value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_types::radar::value_list::RadarValueListId,
    /// The type of items in the value list.
    ///
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: ValueListItemType,
    /// List of items contained within this value list.
    pub list_items: stripe_types::List<stripe_types::radar::value_list_item::ValueListItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The name of the value list.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ValueListObject,
}
/// The type of items in the value list.
///
/// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    String,
}

impl ValueListItemType {
    pub fn as_str(self) -> &'static str {
        use ValueListItemType::*;
        match self {
            CardBin => "card_bin",
            CardFingerprint => "card_fingerprint",
            CaseSensitiveString => "case_sensitive_string",
            Country => "country",
            CustomerId => "customer_id",
            Email => "email",
            IpAddress => "ip_address",
            String => "string",
        }
    }
}

impl std::str::FromStr for ValueListItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ValueListItemType::*;
        match s {
            "card_bin" => Ok(CardBin),
            "card_fingerprint" => Ok(CardFingerprint),
            "case_sensitive_string" => Ok(CaseSensitiveString),
            "country" => Ok(Country),
            "customer_id" => Ok(CustomerId),
            "email" => Ok(Email),
            "ip_address" => Ok(IpAddress),
            "string" => Ok(String),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ValueListItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ValueListItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ValueListItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ValueListItemType"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValueListObject {
    RadarValueList,
}

impl ValueListObject {
    pub fn as_str(self) -> &'static str {
        use ValueListObject::*;
        match self {
            RadarValueList => "radar.value_list",
        }
    }
}

impl std::str::FromStr for ValueListObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ValueListObject::*;
        match s {
            "radar.value_list" => Ok(RadarValueList),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ValueListObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ValueListObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ValueListObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ValueListObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for ValueListObject"))
    }
}
impl stripe_types::Object for ValueList {
    type Id = stripe_types::radar::value_list::RadarValueListId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RadarValueListId);
pub mod deleted;
pub use deleted::DeletedValueList;
