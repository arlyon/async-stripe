#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Consent {
    /// If `opt_in`, the customer consents to receiving promotional communications
    /// from the merchant about this Checkout Session.
    pub promotions: Option<ConsentPromotions>,
    /// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
    pub terms_of_service: Option<ConsentTermsOfService>,
}
/// If `opt_in`, the customer consents to receiving promotional communications
/// from the merchant about this Checkout Session.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConsentPromotions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "opt_in" => Ok(Self::OptIn),
            "opt_out" => Ok(Self::OptOut),

            _ => Err(()),
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
impl serde::Serialize for ConsentPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConsentPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ConsentPromotions"))
    }
}
/// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ConsentTermsOfService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accepted" => Ok(Self::Accepted),

            _ => Err(()),
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
impl serde::Serialize for ConsentTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConsentTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ConsentTermsOfService"))
    }
}
