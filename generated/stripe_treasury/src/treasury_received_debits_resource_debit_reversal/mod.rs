/// You can reverse some [ReceivedDebits](https://stripe.com/docs/api#received_debits) depending on their network and source flow.
///
/// Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedDebitsResourceDebitReversal {
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
    pub id: stripe_treasury::treasury_received_debits_resource_debit_reversal::TreasuryDebitReversalId,
    /// Other flows linked to a DebitReversal.
    pub linked_flows: Option<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryReceivedDebitsResourceDebitReversalNetwork,
    /// The ReceivedDebit being reversed.
    pub received_debit: String,
    /// Status of the DebitReversal.
    pub status: TreasuryReceivedDebitsResourceDebitReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedDebitsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransactionsResourceTransaction>>,
}
/// The rails used to reverse the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitsResourceDebitReversalNetwork {
    Ach,
    Card,
}

impl TreasuryReceivedDebitsResourceDebitReversalNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitsResourceDebitReversalNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceDebitReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceDebitReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitsResourceDebitReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitsResourceDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitsResourceDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitsResourceDebitReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitsResourceDebitReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitsResourceDebitReversalNetwork"))
    }
}
/// Status of the DebitReversal.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitsResourceDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
}

impl TreasuryReceivedDebitsResourceDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitsResourceDebitReversalStatus::*;
        match self {
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceDebitReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceDebitReversalStatus::*;
        match s {
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitsResourceDebitReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitsResourceDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitsResourceDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitsResourceDebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitsResourceDebitReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitsResourceDebitReversalStatus"))
    }
}
impl stripe_types::Object for TreasuryReceivedDebitsResourceDebitReversal {
    type Id = stripe_treasury::treasury_received_debits_resource_debit_reversal::TreasuryDebitReversalId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TreasuryDebitReversalId);
#[cfg(feature = "treasury_received_debits_resource_debit_reversal")]
mod requests;
#[cfg(feature = "treasury_received_debits_resource_debit_reversal")]
pub use requests::*;
