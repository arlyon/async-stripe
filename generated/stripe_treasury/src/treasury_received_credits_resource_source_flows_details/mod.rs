#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<stripe_treasury::TreasuryReceivedCreditsResourceCreditReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<stripe_types::Payout>,
    /// The type of the source flow that originated the ReceivedCredit.
    #[serde(rename = "type")]
    pub type_: TreasuryReceivedCreditsResourceSourceFlowsDetailsType,
}
/// The type of the source flow that originated the ReceivedCredit.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceSourceFlowsDetailsType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            Payout => "payout",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceSourceFlowsDetailsType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "payout" => Ok(Payout),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceSourceFlowsDetailsType"))
    }
}
