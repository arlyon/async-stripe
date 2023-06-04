#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GooglePay {
    /// Google Pay Eligibility.
    pub eligible: bool,
    /// Reason the card is ineligible for Google Pay.
    pub ineligible_reason: Option<GooglePayIneligibleReason>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GooglePay {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Reason the card is ineligible for Google Pay.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
