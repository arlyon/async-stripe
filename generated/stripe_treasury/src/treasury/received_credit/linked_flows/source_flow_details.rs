#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceFlowDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<stripe_treasury::treasury::credit_reversal::CreditReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<stripe_treasury::treasury::outbound_payment::OutboundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<stripe_core::payout::Payout>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SourceFlowDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "credit_reversal" => Ok(Self::CreditReversal),
            "other" => Ok(Self::Other),
            "outbound_payment" => Ok(Self::OutboundPayment),
            "payout" => Ok(Self::Payout),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SourceFlowDetailsType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceFlowDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<SourceFlowDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceFlowDetailsType::from_str(s)?);
        Ok(())
    }
}
