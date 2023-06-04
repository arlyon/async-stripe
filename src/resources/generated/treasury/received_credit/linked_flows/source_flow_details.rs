#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceFlowDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<crate::treasury::credit_reversal::CreditReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<crate::treasury::outbound_payment::OutboundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<crate::payout::Payout>,
    /// The type of the source flow that originated the ReceivedCredit.
    #[serde(rename = "type")]
    pub type_: SourceFlowDetailsType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceFlowDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of the source flow that originated the ReceivedCredit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SourceFlowDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl SourceFlowDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::Payout => "payout",
        }
    }
}

impl AsRef<str> for SourceFlowDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
