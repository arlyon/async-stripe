#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ThreeDSecure {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureAuthenticationFlow>,
    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureResult>,
    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureResultReason>,
    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureVersion>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecure {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// For authenticated transactions: how the customer was authenticated by
/// the issuing bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ThreeDSecureAuthenticationFlow {
    Challenge,
    Frictionless,
}

impl ThreeDSecureAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Challenge => "challenge",
            Self::Frictionless => "frictionless",
        }
    }
}

impl std::str::FromStr for ThreeDSecureAuthenticationFlow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "challenge" => Ok(Self::Challenge),
            "frictionless" => Ok(Self::Frictionless),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ThreeDSecureAuthenticationFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ThreeDSecureAuthenticationFlow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureAuthenticationFlow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ThreeDSecureAuthenticationFlow")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureAuthenticationFlow {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ThreeDSecureAuthenticationFlow> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureAuthenticationFlow::from_str(s)?);
        Ok(())
    }
}
/// Indicates the outcome of 3D Secure authentication.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ThreeDSecureResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}

impl ThreeDSecureResult {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AttemptAcknowledged => "attempt_acknowledged",
            Self::Authenticated => "authenticated",
            Self::Exempted => "exempted",
            Self::Failed => "failed",
            Self::NotSupported => "not_supported",
            Self::ProcessingError => "processing_error",
        }
    }
}

impl std::str::FromStr for ThreeDSecureResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attempt_acknowledged" => Ok(Self::AttemptAcknowledged),
            "authenticated" => Ok(Self::Authenticated),
            "exempted" => Ok(Self::Exempted),
            "failed" => Ok(Self::Failed),
            "not_supported" => Ok(Self::NotSupported),
            "processing_error" => Ok(Self::ProcessingError),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ThreeDSecureResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ThreeDSecureResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureResult"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ThreeDSecureResult> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureResult::from_str(s)?);
        Ok(())
    }
}
/// Additional information about why 3D Secure succeeded or failed based
/// on the `result`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ThreeDSecureResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}

impl ThreeDSecureResultReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abandoned => "abandoned",
            Self::Bypassed => "bypassed",
            Self::Canceled => "canceled",
            Self::CardNotEnrolled => "card_not_enrolled",
            Self::NetworkNotSupported => "network_not_supported",
            Self::ProtocolError => "protocol_error",
            Self::Rejected => "rejected",
        }
    }
}

impl std::str::FromStr for ThreeDSecureResultReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abandoned" => Ok(Self::Abandoned),
            "bypassed" => Ok(Self::Bypassed),
            "canceled" => Ok(Self::Canceled),
            "card_not_enrolled" => Ok(Self::CardNotEnrolled),
            "network_not_supported" => Ok(Self::NetworkNotSupported),
            "protocol_error" => Ok(Self::ProtocolError),
            "rejected" => Ok(Self::Rejected),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ThreeDSecureResultReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ThreeDSecureResultReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureResultReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureResultReason"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureResultReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ThreeDSecureResultReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureResultReason::from_str(s)?);
        Ok(())
    }
}
/// The version of 3D Secure that was used.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ThreeDSecureVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
}

impl ThreeDSecureVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V1_0_2 => "1.0.2",
            Self::V2_1_0 => "2.1.0",
            Self::V2_2_0 => "2.2.0",
        }
    }
}

impl std::str::FromStr for ThreeDSecureVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0.2" => Ok(Self::V1_0_2),
            "2.1.0" => Ok(Self::V2_1_0),
            "2.2.0" => Ok(Self::V2_2_0),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ThreeDSecureVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ThreeDSecureVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureVersion"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ThreeDSecureVersion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureVersion::from_str(s)?);
        Ok(())
    }
}
