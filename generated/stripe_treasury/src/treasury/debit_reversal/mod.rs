/// You can reverse some [ReceivedDebits](https://stripe.com/docs/api#received_debits) depending on their network and source flow.
///
/// Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DebitReversal {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The rails used to reverse the funds.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DebitReversalObject {
    #[serde(rename = "treasury.debit_reversal")]
    TreasuryDebitReversal,
}

impl DebitReversalObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryDebitReversal => "treasury.debit_reversal",
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
/// Status of the DebitReversal.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl stripe_types::Object for DebitReversal {
    type Id = stripe_treasury::treasury::debit_reversal::TreasuryDebitReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryDebitReversalId);
pub mod linked_flows;
pub mod requests;
pub use linked_flows::LinkedFlows;
