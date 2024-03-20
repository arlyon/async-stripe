/// A Transaction represents a real transaction that affects a Financial Connections Account balance.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FinancialConnectionsTransaction {
    /// The ID of the Financial Connections Account this transaction belongs to.
    pub account: String,
    /// The amount of this transaction, in cents (or local equivalent).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The description of this transaction.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The status of the transaction.
    pub status: FinancialConnectionsTransactionStatus,
    pub status_transitions:
        stripe_misc::BankConnectionsResourceTransactionResourceStatusTransitions,
    /// Time at which the transaction was transacted. Measured in seconds since the Unix epoch.
    pub transacted_at: stripe_types::Timestamp,
    /// The token of the transaction refresh that last updated or created this transaction.
    pub transaction_refresh: String,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
}
/// The status of the transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsTransactionStatus {
    Pending,
    Posted,
    Void,
}
impl FinancialConnectionsTransactionStatus {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsTransactionStatus::*;
        match self {
            Pending => "pending",
            Posted => "posted",
            Void => "void",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsTransactionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsTransactionStatus::*;
        match s {
            "pending" => Ok(Pending),
            "posted" => Ok(Posted),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsTransactionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsTransactionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsTransactionStatus")
        })
    }
}
impl stripe_types::Object for FinancialConnectionsTransaction {
    type Id = stripe_misc::FinancialConnectionsTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FinancialConnectionsTransactionId);
