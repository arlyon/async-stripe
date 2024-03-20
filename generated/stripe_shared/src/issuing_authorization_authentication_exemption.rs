#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingAuthorizationAuthenticationExemption {
    /// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
    pub claimed_by: IssuingAuthorizationAuthenticationExemptionClaimedBy,
    /// The specific exemption claimed for this authorization.
    #[serde(rename = "type")]
    pub type_: IssuingAuthorizationAuthenticationExemptionType,
}
/// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
}
impl IssuingAuthorizationAuthenticationExemptionClaimedBy {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match self {
            Acquirer => "acquirer",
            Issuer => "issuer",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match s {
            "acquirer" => Ok(Acquirer),
            "issuer" => Ok(Issuer),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationAuthenticationExemptionClaimedBy",
            )
        })
    }
}
/// The specific exemption claimed for this authorization.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthenticationExemptionType {
    LowValueTransaction,
    TransactionRiskAnalysis,
    Unknown,
}
impl IssuingAuthorizationAuthenticationExemptionType {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match self {
            LowValueTransaction => "low_value_transaction",
            TransactionRiskAnalysis => "transaction_risk_analysis",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match s {
            "low_value_transaction" => Ok(LowValueTransaction),
            "transaction_risk_analysis" => Ok(TransactionRiskAnalysis),
            "unknown" => Ok(Unknown),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationAuthenticationExemptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationAuthenticationExemptionType",
            )
        })
    }
}
