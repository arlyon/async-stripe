// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{IssuingAuthorization, TreasuryCreditReversal, TreasuryDebitReversal, TreasuryInboundTransfer, TreasuryOutboundPayment, TreasuryOutboundTransfer, TreasuryReceivedCredit, TreasuryReceivedDebit};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryTransactionsResourceFlowDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryTransactionsResourceFlowDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<TreasuryCreditReversal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<TreasuryDebitReversal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<TreasuryInboundTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<IssuingAuthorization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<TreasuryOutboundPayment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<TreasuryOutboundTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<TreasuryReceivedCredit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<TreasuryReceivedDebit>,

    /// Type of the flow that created the Transaction.
    ///
    /// Set to the same value as `flow_type`.
    #[serde(rename = "type")]
    pub type_: TreasuryTransactionsResourceFlowDetailsType,
}

/// An enum representing the possible values of an `TreasuryTransactionsResourceFlowDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionsResourceFlowDetailsType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

impl TreasuryTransactionsResourceFlowDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryTransactionsResourceFlowDetailsType::CreditReversal => "credit_reversal",
            TreasuryTransactionsResourceFlowDetailsType::DebitReversal => "debit_reversal",
            TreasuryTransactionsResourceFlowDetailsType::InboundTransfer => "inbound_transfer",
            TreasuryTransactionsResourceFlowDetailsType::IssuingAuthorization => "issuing_authorization",
            TreasuryTransactionsResourceFlowDetailsType::Other => "other",
            TreasuryTransactionsResourceFlowDetailsType::OutboundPayment => "outbound_payment",
            TreasuryTransactionsResourceFlowDetailsType::OutboundTransfer => "outbound_transfer",
            TreasuryTransactionsResourceFlowDetailsType::ReceivedCredit => "received_credit",
            TreasuryTransactionsResourceFlowDetailsType::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TreasuryTransactionsResourceFlowDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryTransactionsResourceFlowDetailsType {
    fn default() -> Self {
        Self::CreditReversal
    }
}
