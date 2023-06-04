#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FlowDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<crate::treasury::credit_reversal::CreditReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<crate::treasury::debit_reversal::DebitReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<crate::treasury::inbound_transfer::InboundTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<crate::issuing::authorization::Authorization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<crate::treasury::outbound_payment::OutboundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<crate::treasury::outbound_transfer::OutboundTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<crate::treasury::received_credit::ReceivedCredit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<crate::treasury::received_debit::ReceivedDebit>,
    /// Type of the flow that created the Transaction.
    ///
    /// Set to the same value as `flow_type`.
    #[serde(rename = "type")]
    pub type_: FlowDetailsType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FlowDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of the flow that created the Transaction.
///
/// Set to the same value as `flow_type`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FlowDetailsType {
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

impl FlowDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::DebitReversal => "debit_reversal",
            Self::InboundTransfer => "inbound_transfer",
            Self::IssuingAuthorization => "issuing_authorization",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::OutboundTransfer => "outbound_transfer",
            Self::ReceivedCredit => "received_credit",
            Self::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for FlowDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
