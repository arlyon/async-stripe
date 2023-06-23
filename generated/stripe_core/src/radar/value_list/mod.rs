/// Value lists allow you to group values together which can then be referenced in rules.
///
/// Related guide: [Default Stripe Lists](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
    pub id: stripe_core::radar::value_list::RadarValueListId,
    /// The type of items in the value list.
    ///
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: ValueListItemType,
    /// List of items contained within this value list.
    pub list_items: stripe_types::List<stripe_core::radar::value_list_item::ValueListItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: stripe_types::Metadata,
    /// The name of the value list.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ValueListObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ValueList {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of items in the value list.
///
/// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
        match self {
            Self::CardBin => "card_bin",
            Self::CardFingerprint => "card_fingerprint",
            Self::CaseSensitiveString => "case_sensitive_string",
            Self::Country => "country",
            Self::CustomerId => "customer_id",
            Self::Email => "email",
            Self::IpAddress => "ip_address",
            Self::String => "string",
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ValueListObject {
    #[serde(rename = "radar.value_list")]
    RadarValueList,
}

impl ValueListObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RadarValueList => "radar.value_list",
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
impl stripe_types::Object for ValueList {
    type Id = stripe_core::radar::value_list::RadarValueListId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RadarValueListId);
pub mod deleted;
pub mod requests;
pub use deleted::DeletedValueList;
