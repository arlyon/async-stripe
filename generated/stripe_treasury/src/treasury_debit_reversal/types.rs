/// You can reverse some [ReceivedDebits](https://stripe.com/docs/api#received_debits) depending on their network and source flow.
/// Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryDebitReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryDebitReversalId,
    /// Other flows linked to a DebitReversal.
    pub linked_flows:
        Option<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryDebitReversalNetwork,
    /// The ReceivedDebit being reversed.
    pub received_debit: String,
    /// Status of the DebitReversal
    pub status: TreasuryDebitReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedDebitsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
/// The rails used to reverse the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryDebitReversalNetwork {
    Ach,
    Card,
}
impl TreasuryDebitReversalNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryDebitReversalNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
        }
    }
}

impl std::str::FromStr for TreasuryDebitReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryDebitReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryDebitReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryDebitReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryDebitReversalNetwork"))
    }
}
/// Status of the DebitReversal
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
}
impl TreasuryDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryDebitReversalStatus::*;
        match self {
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryDebitReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryDebitReversalStatus::*;
        match s {
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryDebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryDebitReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryDebitReversalStatus"))
    }
}
impl stripe_types::Object for TreasuryDebitReversal {
    type Id = stripe_treasury::TreasuryDebitReversalId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryDebitReversalId);
