/// You can reverse some [ReceivedDebits](https://stripe.com/docs/api#received_debits) depending on their network and source flow.
///
/// Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DebitReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury::debit_reversal::TreasuryDebitReversalId,
    /// Other flows linked to a DebitReversal.
    pub linked_flows: Option<stripe_treasury::treasury::debit_reversal::linked_flows::LinkedFlows>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: DebitReversalNetwork,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DebitReversalObject,
    /// The ReceivedDebit being reversed.
    pub received_debit: String,
    /// Status of the DebitReversal.
    pub status: DebitReversalStatus,
    pub status_transitions:
        stripe_treasury::treasury::received_debit::status_transitions::StatusTransitions,
    /// The Transaction associated with this object.
    pub transaction:
        Option<stripe_types::Expandable<stripe_treasury::treasury::transaction::Transaction>>,
}
/// The rails used to reverse the funds.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DebitReversalNetwork {
    Ach,
    Card,
}

impl DebitReversalNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Card => "card",
        }
    }
}

impl std::str::FromStr for DebitReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach" => Ok(Self::Ach),
            "card" => Ok(Self::Card),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DebitReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DebitReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DebitReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DebitReversalNetwork"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DebitReversalObject {
    TreasuryDebitReversal,
}

impl DebitReversalObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryDebitReversal => "treasury.debit_reversal",
        }
    }
}

impl std::str::FromStr for DebitReversalObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "treasury.debit_reversal" => Ok(Self::TreasuryDebitReversal),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DebitReversalObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DebitReversalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DebitReversalObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DebitReversalObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DebitReversalObject"))
    }
}
/// Status of the DebitReversal.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
}

impl DebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Processing => "processing",
            Self::Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for DebitReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "failed" => Ok(Self::Failed),
            "processing" => Ok(Self::Processing),
            "succeeded" => Ok(Self::Succeeded),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DebitReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DebitReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DebitReversalStatus"))
    }
}
impl stripe_types::Object for DebitReversal {
    type Id = stripe_treasury::treasury::debit_reversal::TreasuryDebitReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryDebitReversalId);
pub mod linked_flows;
pub use linked_flows::LinkedFlows;
