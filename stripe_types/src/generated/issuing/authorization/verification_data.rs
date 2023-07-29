#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationData {
    /// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    pub address_line1_check: VerificationDataAddressLine1Check,
    /// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    pub address_postal_code_check: VerificationDataAddressPostalCodeCheck,
    /// Whether the cardholder provided a CVC and if it matched Stripe’s record.
    pub cvc_check: VerificationDataCvcCheck,
    /// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    pub expiry_check: VerificationDataExpiryCheck,
}
/// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationDataAddressLine1Check {
    Match,
    Mismatch,
    NotProvided,
}

impl VerificationDataAddressLine1Check {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Mismatch => "mismatch",
            Self::NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for VerificationDataAddressLine1Check {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "match" => Ok(Self::Match),
            "mismatch" => Ok(Self::Mismatch),
            "not_provided" => Ok(Self::NotProvided),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationDataAddressLine1Check {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationDataAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationDataAddressLine1Check {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationDataAddressLine1Check {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for VerificationDataAddressLine1Check")
        })
    }
}
/// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationDataAddressPostalCodeCheck {
    Match,
    Mismatch,
    NotProvided,
}

impl VerificationDataAddressPostalCodeCheck {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Mismatch => "mismatch",
            Self::NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for VerificationDataAddressPostalCodeCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "match" => Ok(Self::Match),
            "mismatch" => Ok(Self::Mismatch),
            "not_provided" => Ok(Self::NotProvided),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationDataAddressPostalCodeCheck {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationDataAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationDataAddressPostalCodeCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationDataAddressPostalCodeCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for VerificationDataAddressPostalCodeCheck")
        })
    }
}
/// Whether the cardholder provided a CVC and if it matched Stripe’s record.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationDataCvcCheck {
    Match,
    Mismatch,
    NotProvided,
}

impl VerificationDataCvcCheck {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Mismatch => "mismatch",
            Self::NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for VerificationDataCvcCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "match" => Ok(Self::Match),
            "mismatch" => Ok(Self::Mismatch),
            "not_provided" => Ok(Self::NotProvided),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationDataCvcCheck {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationDataCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationDataCvcCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationDataCvcCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationDataCvcCheck"))
    }
}
/// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VerificationDataExpiryCheck {
    Match,
    Mismatch,
    NotProvided,
}

impl VerificationDataExpiryCheck {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Mismatch => "mismatch",
            Self::NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for VerificationDataExpiryCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "match" => Ok(Self::Match),
            "mismatch" => Ok(Self::Mismatch),
            "not_provided" => Ok(Self::NotProvided),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for VerificationDataExpiryCheck {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerificationDataExpiryCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for VerificationDataExpiryCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerificationDataExpiryCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerificationDataExpiryCheck"))
    }
}
