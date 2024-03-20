#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingAuthorizationThreeDSecure {
    /// The outcome of the 3D Secure authentication request.
    pub result: IssuingAuthorizationThreeDSecureResult,
}
/// The outcome of the 3D Secure authentication request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationThreeDSecureResult {
    AttemptAcknowledged,
    Authenticated,
    Failed,
    Required,
}
impl IssuingAuthorizationThreeDSecureResult {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationThreeDSecureResult::*;
        match self {
            AttemptAcknowledged => "attempt_acknowledged",
            Authenticated => "authenticated",
            Failed => "failed",
            Required => "required",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationThreeDSecureResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationThreeDSecureResult::*;
        match s {
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "failed" => Ok(Failed),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationThreeDSecureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationThreeDSecureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingAuthorizationThreeDSecureResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationThreeDSecureResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationThreeDSecureResult")
        })
    }
}
