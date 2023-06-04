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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Indicates the outcome of 3D Secure authentication.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Additional information about why 3D Secure succeeded or failed based
/// on the `result`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// The version of 3D Secure that was used.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureVersion {
    #[serde(rename = "1.0.2")]
    V1_0_2,
    #[serde(rename = "2.1.0")]
    V2_1_0,
    #[serde(rename = "2.2.0")]
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
