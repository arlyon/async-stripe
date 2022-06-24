// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::TreasuryReceivedCreditId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{
    Currency, Payout, TreasuryCreditReversal, TreasuryOutboundPayment,
    TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    TreasuryTransaction,
};

/// The resource representing a Stripe "ReceivedCreditsResourceTreasuryReceivedCredit".
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<TreasuryReceivedCreditFailureCode>,

    /// The FinancialAccount that received the funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,

    pub initiating_payment_method_details:
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,

    pub linked_flows: ReceivedCreditsResourceTreasuryLinkedFlows,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The rails used to send the funds.
    pub network: TreasuryReceivedCreditNetwork,

    /// Details describing when a ReceivedCredit may be reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_details: Option<ReceivedCreditsResourceReversalDetails>,

    /// Status of the ReceivedCredit.
    ///
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
    pub status: TreasuryReceivedCreditStatus,

    /// The Transaction associated with this object.
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct ReceivedCreditsResourceReversalDetails {
    /// Time before which a ReceivedCredit can be reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Timestamp>,

    /// Set if a ReceivedCredit cannot be reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_reason: Option<ReceivedCreditsResourceReversalDetailsRestrictedReason>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReceivedCreditsResourceTreasuryLinkedFlows {
    /// The CreditReversal created as a result of this ReceivedCredit being reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<String>,

    /// Set if the ReceivedCredit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,

    /// Set if the ReceivedCredit is also viewable as an [Issuing transaction](https://stripe.com/docs/api#issuing_transactions) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,

    /// ID of the source flow.
    ///
    /// Set if `network` is `stripe` and the source flow is visible to the merchant.
    /// Examples of source flows include OutboundPayments, payouts, or CreditReversals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow: Option<String>,

    /// The expandable object of the source flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_details: Option<ReceivedCreditsResourceTreasurySourceFlowsDetails>,

    /// The type of flow that originated the ReceivedCredit (for example, `outbound_payment`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReceivedCreditsResourceTreasurySourceFlowsDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<TreasuryCreditReversal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<TreasuryOutboundPayment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<Payout>,

    /// The type of the source flow that originated the ReceivedCredit.
    #[serde(rename = "type")]
    pub type_: ReceivedCreditsResourceTreasurySourceFlowsDetailsType,
}

/// An enum representing the possible values of an `ReceivedCreditsResourceReversalDetails`'s `restricted_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReceivedCreditsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

impl ReceivedCreditsResourceReversalDetailsRestrictedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ReceivedCreditsResourceReversalDetailsRestrictedReason::AlreadyReversed => {
                "already_reversed"
            }
            ReceivedCreditsResourceReversalDetailsRestrictedReason::DeadlinePassed => {
                "deadline_passed"
            }
            ReceivedCreditsResourceReversalDetailsRestrictedReason::NetworkRestricted => {
                "network_restricted"
            }
            ReceivedCreditsResourceReversalDetailsRestrictedReason::Other => "other",
            ReceivedCreditsResourceReversalDetailsRestrictedReason::SourceFlowRestricted => {
                "source_flow_restricted"
            }
        }
    }
}

impl AsRef<str> for ReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn default() -> Self {
        Self::AlreadyReversed
    }
}

/// An enum representing the possible values of an `ReceivedCreditsResourceTreasurySourceFlowsDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReceivedCreditsResourceTreasurySourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl ReceivedCreditsResourceTreasurySourceFlowsDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            ReceivedCreditsResourceTreasurySourceFlowsDetailsType::CreditReversal => {
                "credit_reversal"
            }
            ReceivedCreditsResourceTreasurySourceFlowsDetailsType::Other => "other",
            ReceivedCreditsResourceTreasurySourceFlowsDetailsType::OutboundPayment => {
                "outbound_payment"
            }
            ReceivedCreditsResourceTreasurySourceFlowsDetailsType::Payout => "payout",
        }
    }
}

impl AsRef<str> for ReceivedCreditsResourceTreasurySourceFlowsDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedCreditsResourceTreasurySourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ReceivedCreditsResourceTreasurySourceFlowsDetailsType {
    fn default() -> Self {
        Self::CreditReversal
    }
}

/// An enum representing the possible values of an `TreasuryReceivedCredit`'s `failure_code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}

impl TreasuryReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryReceivedCreditFailureCode::AccountClosed => "account_closed",
            TreasuryReceivedCreditFailureCode::AccountFrozen => "account_frozen",
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
