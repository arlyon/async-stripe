#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    CreditReversal,
    Other,
    OutboundPayment,
    OutboundTransfer,
    Payout,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    pub fn as_str(&self) -> &str {
        use ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundTransfer => "outbound_transfer",
            Payout => "payout",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_transfer" => Ok(OutboundTransfer),
            "payout" => Ok(Payout),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ListTreasuryReceivedCreditLinkedFlowsSourceFlowType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ListTreasuryReceivedCreditLinkedFlowsSourceFlowType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ListTreasuryReceivedCreditLinkedFlowsSourceFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ListTreasuryReceivedCreditLinkedFlowsSourceFlowType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
