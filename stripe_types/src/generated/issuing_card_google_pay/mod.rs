#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardGooglePay {
    /// Google Pay Eligibility.
    pub eligible: bool,
    /// Reason the card is ineligible for Google Pay.
    pub ineligible_reason: Option<IssuingCardGooglePayIneligibleReason>,
}
/// Reason the card is ineligible for Google Pay.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardGooglePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

impl IssuingCardGooglePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardGooglePayIneligibleReason::*;
        match self {
            MissingAgreement => "missing_agreement",
            MissingCardholderContact => "missing_cardholder_contact",
            UnsupportedRegion => "unsupported_region",
        }
    }
}

impl std::str::FromStr for IssuingCardGooglePayIneligibleReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardGooglePayIneligibleReason::*;
        match s {
            "missing_agreement" => Ok(MissingAgreement),
            "missing_cardholder_contact" => Ok(MissingCardholderContact),
            "unsupported_region" => Ok(UnsupportedRegion),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardGooglePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardGooglePayIneligibleReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardGooglePayIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardGooglePayIneligibleReason")
        })
    }
}
