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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ApplePayIneligibleReason {
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
impl serde::Serialize for ApplePayIneligibleReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ApplePayIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ApplePayIneligibleReason"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApplePayIneligibleReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ApplePayIneligibleReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApplePayIneligibleReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
