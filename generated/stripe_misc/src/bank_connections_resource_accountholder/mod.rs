#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceAccountholder {
    /// The ID of the Stripe account this account belongs to.
    ///
    /// Should only be present if `account_holder.type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types::Expandable<stripe_types::Account>>,
    /// ID of the Stripe customer this account belongs to.
    ///
    /// Present if and only if `account_holder.type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<stripe_types::Expandable<stripe_types::Customer>>,
    /// Type of account holder that this account belongs to.
    #[serde(rename = "type")]
    pub type_: BankConnectionsResourceAccountholderType,
}
/// Type of account holder that this account belongs to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceAccountholderType {
    Account,
    Customer,
}

impl BankConnectionsResourceAccountholderType {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceAccountholderType::*;
        match self {
            Account => "account",
            Customer => "customer",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceAccountholderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountholderType::*;
        match s {
            "account" => Ok(Account),
            "customer" => Ok(Customer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceAccountholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceAccountholderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceAccountholderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BankConnectionsResourceAccountholderType")
        })
    }
}
