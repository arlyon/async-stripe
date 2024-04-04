#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingAuthorizationVerificationData {
    /// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    pub address_line1_check: IssuingAuthorizationVerificationDataAddressLine1Check,
    /// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    pub address_postal_code_check: IssuingAuthorizationVerificationDataAddressPostalCodeCheck,
    /// The exemption applied to this authorization.
    pub authentication_exemption:
        Option<stripe_shared::IssuingAuthorizationAuthenticationExemption>,
    /// Whether the cardholder provided a CVC and if it matched Stripe’s record.
    pub cvc_check: IssuingAuthorizationVerificationDataCvcCheck,
    /// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    pub expiry_check: IssuingAuthorizationVerificationDataExpiryCheck,
    /// The postal code submitted as part of the authorization used for postal code verification.
    pub postal_code: Option<String>,
    /// 3D Secure details.
    pub three_d_secure: Option<stripe_shared::IssuingAuthorizationThreeDSecure>,
}
/// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataAddressLine1Check {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataAddressLine1Check {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataAddressLine1Check::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataAddressLine1Check {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataAddressLine1Check::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataAddressLine1Check {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationVerificationDataAddressLine1Check",
            )
        })
    }
}
/// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataAddressPostalCodeCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataAddressPostalCodeCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationVerificationDataAddressPostalCodeCheck",
            )
        })
    }
}
/// Whether the cardholder provided a CVC and if it matched Stripe’s record.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataCvcCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataCvcCheck {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataCvcCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataCvcCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataCvcCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataCvcCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataCvcCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationVerificationDataCvcCheck",
            )
        })
    }
}
/// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationVerificationDataExpiryCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl IssuingAuthorizationVerificationDataExpiryCheck {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationVerificationDataExpiryCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationVerificationDataExpiryCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationVerificationDataExpiryCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationVerificationDataExpiryCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationVerificationDataExpiryCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationVerificationDataExpiryCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationVerificationDataExpiryCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationVerificationDataExpiryCheck",
            )
        })
    }
}
