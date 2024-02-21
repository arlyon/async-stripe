// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{RadarValueListId};
use crate::params::{List, Metadata, Object, Timestamp};
use crate::resources::{RadarValueListItem};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "RadarListList".
///
/// For more details see <https://stripe.com/docs/api/radar/value_lists/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarValueList {
    /// Unique identifier for the object.
    pub id: RadarValueListId,

    /// The name of the value list for use in rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// The name or email address of the user who created this value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The type of items in the value list.
    ///
    /// One of `card_fingerprint`, `us_bank_account_fingerprint`, `sepa_debit_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<RadarValueListItemType>,

    /// List of items contained within this value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_items: Option<List<RadarValueListItem>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The name of the value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Object for RadarValueList {
    type Id = RadarValueListId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "radar.value_list"
    }
}

/// An enum representing the possible values of an `RadarValueList`'s `item_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RadarValueListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    SepaDebitFingerprint,
    String,
    UsBankAccountFingerprint,
}

impl RadarValueListItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            RadarValueListItemType::CardBin => "card_bin",
            RadarValueListItemType::CardFingerprint => "card_fingerprint",
            RadarValueListItemType::CaseSensitiveString => "case_sensitive_string",
            RadarValueListItemType::Country => "country",
            RadarValueListItemType::CustomerId => "customer_id",
            RadarValueListItemType::Email => "email",
            RadarValueListItemType::IpAddress => "ip_address",
            RadarValueListItemType::SepaDebitFingerprint => "sepa_debit_fingerprint",
            RadarValueListItemType::String => "string",
            RadarValueListItemType::UsBankAccountFingerprint => "us_bank_account_fingerprint",
        }
    }
}

impl AsRef<str> for RadarValueListItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RadarValueListItemType {
    fn default() -> Self {
        Self::CardBin
    }
}
