#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Consent {
    /// If `opt_in`, the customer consents to receiving promotional communications
    /// from the merchant about this Checkout Session.
    pub promotions: Option<ConsentPromotions>,
    /// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
    pub terms_of_service: Option<ConsentTermsOfService>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Consent {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// If `opt_in`, the customer consents to receiving promotional communications
/// from the merchant about this Checkout Session.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ConsentPromotions {
    OptIn,
    OptOut,
}

impl ConsentPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OptIn => "opt_in",
            Self::OptOut => "opt_out",
        }
    }
}

impl AsRef<str> for ConsentPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConsentPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ConsentTermsOfService {
    Accepted,
}

impl ConsentTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
        }
    }
}

impl AsRef<str> for ConsentTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConsentTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
