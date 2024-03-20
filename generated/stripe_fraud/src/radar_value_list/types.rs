/// Value lists allow you to group values together which can then be referenced in rules.
///
/// Related guide: [Default Stripe lists](https://stripe.com/docs/radar/lists#managing-list-items)
///
/// For more details see <<https://stripe.com/docs/api/radar/value_lists/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarValueList {
    /// The name of the value list for use in rules.
    pub alias: String,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who created this value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListId,
    /// The type of items in the value list.
    /// One of `card_fingerprint`, `us_bank_account_fingerprint`, `sepa_debit_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: stripe_fraud::RadarValueListItemType,
    /// List of items contained within this value list.
    pub list_items: stripe_types::List<stripe_fraud::RadarValueListItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The name of the value list.
    pub name: String,
}
impl stripe_types::Object for RadarValueList {
    type Id = stripe_fraud::RadarValueListId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(RadarValueListId);
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use RadarValueListItemType::*;
        match self {
            CardBin => "card_bin",
            CardFingerprint => "card_fingerprint",
            CaseSensitiveString => "case_sensitive_string",
            Country => "country",
            CustomerId => "customer_id",
            Email => "email",
            IpAddress => "ip_address",
            SepaDebitFingerprint => "sepa_debit_fingerprint",
            String => "string",
            UsBankAccountFingerprint => "us_bank_account_fingerprint",
        }
    }
}

impl std::str::FromStr for RadarValueListItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RadarValueListItemType::*;
        match s {
            "card_bin" => Ok(CardBin),
            "card_fingerprint" => Ok(CardFingerprint),
            "case_sensitive_string" => Ok(CaseSensitiveString),
            "country" => Ok(Country),
            "customer_id" => Ok(CustomerId),
            "email" => Ok(Email),
            "ip_address" => Ok(IpAddress),
            "sepa_debit_fingerprint" => Ok(SepaDebitFingerprint),
            "string" => Ok(String),
            "us_bank_account_fingerprint" => Ok(UsBankAccountFingerprint),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RadarValueListItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RadarValueListItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RadarValueListItemType"))
    }
}
