#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ApplePay {
    /// Apple Pay Eligibility.
    pub eligible: bool,
    /// Reason the card is ineligible for Apple Pay.
    pub ineligible_reason: Option<ApplePayIneligibleReason>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApplePay {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Reason the card is ineligible for Apple Pay.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ApplePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

impl ApplePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingAgreement => "missing_agreement",
            Self::MissingCardholderContact => "missing_cardholder_contact",
            Self::UnsupportedRegion => "unsupported_region",
        }
    }
}

impl AsRef<str> for ApplePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApplePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
