/// Value lists allow you to group values together which can then be referenced in rules.
///
/// Related guide: [Default Stripe lists](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarListList {
    /// The name of the value list for use in rules.
    pub alias: String,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who created this value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_list_list::RadarValueListId,
    /// The type of items in the value list.
    ///
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: RadarListListItemType,
    /// List of items contained within this value list.
    pub list_items: stripe_types::List<stripe_fraud::RadarListListItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The name of the value list.
    pub name: String,
}
/// The type of items in the value list.
///
/// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RadarListListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    String,
}

impl RadarListListItemType {
    pub fn as_str(self) -> &'static str {
        use RadarListListItemType::*;
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

impl std::str::FromStr for RadarListListItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RadarListListItemType::*;
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

impl AsRef<str> for RadarListListItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RadarListListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RadarListListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RadarListListItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RadarListListItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RadarListListItemType"))
    }
}
impl stripe_types::Object for RadarListList {
    type Id = stripe_fraud::radar_list_list::RadarValueListId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RadarValueListId);
#[cfg(feature = "radar_list_list")]
mod requests;
#[cfg(feature = "radar_list_list")]
pub use requests::*;
