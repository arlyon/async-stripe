#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SourceFlowDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<stripe_treasury::treasury::credit_reversal::CreditReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<stripe_treasury::treasury::outbound_payment::OutboundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<stripe_types::payout::Payout>,
    /// The type of the source flow that originated the ReceivedCredit.
    #[serde(rename = "type")]
    pub type_: SourceFlowDetailsType,
}
/// The type of the source flow that originated the ReceivedCredit.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SourceFlowDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl SourceFlowDetailsType {
    pub fn as_str(self) -> &'static str {
        use SourceFlowDetailsType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            Payout => "payout",
        }
    }
}

impl std::str::FromStr for SourceFlowDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceFlowDetailsType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "payout" => Ok(Payout),
            _ => Err(()),
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
impl serde::Serialize for SourceFlowDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SourceFlowDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SourceFlowDetailsType"))
    }
}
