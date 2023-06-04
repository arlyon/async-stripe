/// You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow.
///
/// Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CreditReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: crate::treasury::credit_reversal::TreasuryCreditReversalId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// The rails used to reverse the funds.
    pub network: CreditReversalNetwork,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CreditReversalObject,
    /// The ReceivedCredit being reversed.
    pub received_credit: String,
    /// Status of the CreditReversal.
    pub status: CreditReversalStatus,
    pub status_transitions: crate::treasury::received_credit::status_transitions::StatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<crate::Expandable<crate::treasury::transaction::Transaction>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditReversal {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The rails used to reverse the funds.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CreditReversalNetwork {
    Ach,
    Stripe,
}

impl CreditReversalNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for CreditReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditReversalNetwork {
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
pub enum CreditReversalObject {
    #[serde(rename = "treasury.credit_reversal")]
    TreasuryCreditReversal,
}

impl CreditReversalObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryCreditReversal => "treasury.credit_reversal",
        }
    }
}

impl AsRef<str> for CreditReversalObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditReversalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Status of the CreditReversal.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}

impl CreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Posted => "posted",
            Self::Processing => "processing",
        }
    }
}

impl AsRef<str> for CreditReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for CreditReversal {
    type Id = crate::treasury::credit_reversal::TreasuryCreditReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TreasuryCreditReversalId);
pub mod requests;
