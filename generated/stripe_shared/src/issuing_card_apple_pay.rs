#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardApplePay {
    /// Apple Pay Eligibility
    pub eligible: bool,
    /// Reason the card is ineligible for Apple Pay
    pub ineligible_reason: Option<IssuingCardApplePayIneligibleReason>,
}
/// Reason the card is ineligible for Apple Pay
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardApplePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}
impl IssuingCardApplePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardApplePayIneligibleReason::*;
        match self {
            MissingAgreement => "missing_agreement",
            MissingCardholderContact => "missing_cardholder_contact",
            UnsupportedRegion => "unsupported_region",
        }
    }
}

impl std::str::FromStr for IssuingCardApplePayIneligibleReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardApplePayIneligibleReason::*;
        match s {
            "missing_agreement" => Ok(MissingAgreement),
            "missing_cardholder_contact" => Ok(MissingCardholderContact),
            "unsupported_region" => Ok(UnsupportedRegion),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingCardApplePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardApplePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardApplePayIneligibleReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardApplePayIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardApplePayIneligibleReason")
        })
    }
}
