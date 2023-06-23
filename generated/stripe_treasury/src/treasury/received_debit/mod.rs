/// ReceivedDebits represent funds pulled from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
///
/// These are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReceivedDebit {
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
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
pub description: String,
    /// Reason for the failure.
    ///
    /// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
pub failure_code: Option<ReceivedDebitFailureCode>,
    /// The FinancialAccount that funds were pulled from.
pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: stripe_treasury::treasury::received_debit::TreasuryReceivedDebitId,
#[serde(skip_serializing_if = "Option::is_none")]
pub initiating_payment_method_details: Option<stripe_treasury::treasury::received_credit::initiating_payment_method_details::InitiatingPaymentMethodDetails>,
pub linked_flows: stripe_treasury::treasury::received_debit::linked_flows::LinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// The network used for the ReceivedDebit.
pub network: ReceivedDebitNetwork,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: ReceivedDebitObject,
    /// Details describing when a ReceivedDebit might be reversed.
pub reversal_details: Option<stripe_treasury::treasury::received_debit::reversal_details::ReversalDetails>,
    /// Status of the ReceivedDebit.
    ///
    /// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
    /// The failure reason can be found under the `failure_code`.
pub status: ReceivedDebitStatus,
    /// The Transaction associated with this object.
pub transaction: Option<stripe_types::Expandable<stripe_treasury::treasury::transaction::Transaction>>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReceivedDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Reason for the failure.
///
/// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    Other,
}

impl ReceivedDebitFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::InsufficientFunds => "insufficient_funds",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for ReceivedDebitFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The network used for the ReceivedDebit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
}

impl ReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Card => "card",
            Self::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for ReceivedDebitNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedDebitNetwork {
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
pub enum ReceivedDebitObject {
    #[serde(rename = "treasury.received_debit")]
    TreasuryReceivedDebit,
}

impl ReceivedDebitObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryReceivedDebit => "treasury.received_debit",
        }
    }
}

impl AsRef<str> for ReceivedDebitObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedDebitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Status of the ReceivedDebit.
///
/// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
/// The failure reason can be found under the `failure_code`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ReceivedDebitStatus {
    Failed,
    Succeeded,
}

impl ReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for ReceivedDebitStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for ReceivedDebit {
    type Id = stripe_treasury::treasury::received_debit::TreasuryReceivedDebitId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryReceivedDebitId);
pub mod linked_flows;
pub mod requests;
pub use linked_flows::LinkedFlows;
pub mod reversal_details;
pub use reversal_details::ReversalDetails;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
