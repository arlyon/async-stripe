// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TreasuryReceivedCreditId};
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{Currency, Payout, TreasuryCreditReversal, TreasuryOutboundPayment, TreasuryOutboundTransfer, TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails, TreasuryTransaction};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryReceivedCreditsResourceReceivedCredit".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedCredit {
    /// Unique identifier for the object.
    pub id: TreasuryReceivedCreditId,

    /// Amount (in cents) transferred.
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,

    /// Reason for the failure.
    ///
    /// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
    pub failure_code: Option<TreasuryReceivedCreditFailureCode>,

    /// The FinancialAccount that received the funds.
    pub financial_account: Option<String>,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,

    pub initiating_payment_method_details: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,

    pub linked_flows: TreasuryReceivedCreditsResourceLinkedFlows,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The rails used to send the funds.
    pub network: TreasuryReceivedCreditNetwork,

    /// Details describing when a ReceivedCredit may be reversed.
    pub reversal_details: Option<TreasuryReceivedCreditsResourceReversalDetails>,

    /// Status of the ReceivedCredit.
    ///
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
    pub status: TreasuryReceivedCreditStatus,

    /// The Transaction associated with this object.
    pub transaction: Option<Expandable<TreasuryTransaction>>,
}

impl Object for TreasuryReceivedCredit {
    type Id = TreasuryReceivedCreditId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "treasury.received_credit"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {

    /// The CreditReversal created as a result of this ReceivedCredit being reversed.
    pub credit_reversal: Option<String>,

    /// Set if the ReceivedCredit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    pub issuing_authorization: Option<String>,

    /// Set if the ReceivedCredit is also viewable as an [Issuing transaction](https://stripe.com/docs/api#issuing_transactions) object.
    pub issuing_transaction: Option<String>,

    /// ID of the source flow.
    ///
    /// Set if `network` is `stripe` and the source flow is visible to the user.
    /// Examples of source flows include OutboundPayments, payouts, or CreditReversals.
    pub source_flow: Option<String>,

    /// The expandable object of the source flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_details: Option<TreasuryReceivedCreditsResourceSourceFlowsDetails>,

    /// The type of flow that originated the ReceivedCredit (for example, `outbound_payment`).
    pub source_flow_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedCreditsResourceReversalDetails {

    /// Time before which a ReceivedCredit can be reversed.
    pub deadline: Option<Timestamp>,

    /// Set if a ReceivedCredit cannot be reversed.
    pub restricted_reason: Option<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<TreasuryCreditReversal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<TreasuryOutboundPayment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<TreasuryOutboundTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<Payout>,

    /// The type of the source flow that originated the ReceivedCredit.
    #[serde(rename = "type")]
    pub type_: TreasuryReceivedCreditsResourceSourceFlowsDetailsType,
}

/// An enum representing the possible values of an `TreasuryReceivedCredit`'s `failure_code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    InternationalTransaction,
    Other,
}

impl TreasuryReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedCreditFailureCode::AccountClosed => "account_closed",
            TreasuryReceivedCreditFailureCode::AccountFrozen => "account_frozen",
            TreasuryReceivedCreditFailureCode::InternationalTransaction => "international_transaction",
            TreasuryReceivedCreditFailureCode::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedCreditFailureCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}

/// An enum representing the possible values of an `TreasuryReceivedCredit`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}

impl TreasuryReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedCreditNetwork::Ach => "ach",
            TreasuryReceivedCreditNetwork::Card => "card",
            TreasuryReceivedCreditNetwork::Stripe => "stripe",
            TreasuryReceivedCreditNetwork::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedCreditNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `TreasuryReceivedCredit`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditStatus {
    Failed,
    Succeeded,
}

impl TreasuryReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedCreditStatus::Failed => "failed",
            TreasuryReceivedCreditStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedCreditStatus {
    fn default() -> Self {
        Self::Failed
    }
}

/// An enum representing the possible values of an `TreasuryReceivedCreditsResourceReversalDetails`'s `restricted_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

impl TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::AlreadyReversed => "already_reversed",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::DeadlinePassed => "deadline_passed",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::NetworkRestricted => "network_restricted",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::Other => "other",
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::SourceFlowRestricted => "source_flow_restricted",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn default() -> Self {
        Self::AlreadyReversed
    }
}

/// An enum representing the possible values of an `TreasuryReceivedCreditsResourceSourceFlowsDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    OutboundTransfer,
    Payout,
}

impl TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::CreditReversal => "credit_reversal",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Other => "other",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::OutboundPayment => "outbound_payment",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::OutboundTransfer => "outbound_transfer",
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::Payout => "payout",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn default() -> Self {
        Self::CreditReversal
    }
}
