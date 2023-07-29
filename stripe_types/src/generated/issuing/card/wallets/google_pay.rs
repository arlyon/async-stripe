#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GooglePay {
    /// Google Pay Eligibility.
    pub eligible: bool,
    /// Reason the card is ineligible for Google Pay.
    pub ineligible_reason: Option<GooglePayIneligibleReason>,
}
/// Reason the card is ineligible for Google Pay.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GooglePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

impl GooglePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingAgreement => "missing_agreement",
            Self::MissingCardholderContact => "missing_cardholder_contact",
            Self::UnsupportedRegion => "unsupported_region",
        }
    }
}

impl std::str::FromStr for GooglePayIneligibleReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "missing_agreement" => Ok(Self::MissingAgreement),
            "missing_cardholder_contact" => Ok(Self::MissingCardholderContact),
            "unsupported_region" => Ok(Self::UnsupportedRegion),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for GooglePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for GooglePayIneligibleReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GooglePayIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GooglePayIneligibleReason"))
    }
}
