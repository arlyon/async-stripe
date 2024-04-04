#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxIDsOwner {
    /// The account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The Connect Application being referenced when `type` is `application`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The customer being referenced when `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Type of owner referenced.
    #[serde(rename = "type")]
    pub type_: TaxIDsOwnerType,
}
/// Type of owner referenced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxIDsOwnerType {
    Account,
    Application,
    Customer,
    Self_,
}
impl TaxIDsOwnerType {
    pub fn as_str(self) -> &'static str {
        use TaxIDsOwnerType::*;
        match self {
            Account => "account",
            Application => "application",
            Customer => "customer",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for TaxIDsOwnerType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxIDsOwnerType::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            "customer" => Ok(Customer),
            "self" => Ok(Self_),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxIDsOwnerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxIDsOwnerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxIDsOwnerType"))
    }
}
