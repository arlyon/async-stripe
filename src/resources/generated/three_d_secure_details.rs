// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "three_d_secure_details".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreeDSecureDetails {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<ThreeDSecureDetailsAuthenticationFlow>,

    /// Indicates the outcome of 3D Secure authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ThreeDSecureDetailsResult>,

    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_reason: Option<ThreeDSecureDetailsResultReason>,

    /// The version of 3D Secure that was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ThreeDSecureDetailsVersion>,
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `authentication_flow` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
}

impl ThreeDSecureDetailsAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsAuthenticationFlow::Challenge => "challenge",
            ThreeDSecureDetailsAuthenticationFlow::Frictionless => "frictionless",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsAuthenticationFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `result` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    Failed,
    NotSupported,
    ProcessingError,
}

impl ThreeDSecureDetailsResult {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsResult::AttemptAcknowledged => "attempt_acknowledged",
            ThreeDSecureDetailsResult::Authenticated => "authenticated",
            ThreeDSecureDetailsResult::Failed => "failed",
            ThreeDSecureDetailsResult::NotSupported => "not_supported",
            ThreeDSecureDetailsResult::ProcessingError => "processing_error",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `result_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}

impl ThreeDSecureDetailsResultReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsResultReason::Abandoned => "abandoned",
            ThreeDSecureDetailsResultReason::Bypassed => "bypassed",
            ThreeDSecureDetailsResultReason::Canceled => "canceled",
            ThreeDSecureDetailsResultReason::CardNotEnrolled => "card_not_enrolled",
            ThreeDSecureDetailsResultReason::NetworkNotSupported => "network_not_supported",
            ThreeDSecureDetailsResultReason::ProtocolError => "protocol_error",
            ThreeDSecureDetailsResultReason::Rejected => "rejected",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsResultReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `version` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsVersion {
    #[serde(rename = "1.0.2")]
    V1_0_2,
    #[serde(rename = "2.1.0")]
    V2_1_0,
    #[serde(rename = "2.2.0")]
    V2_2_0,
}

impl ThreeDSecureDetailsVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsVersion::V1_0_2 => "1.0.2",
            ThreeDSecureDetailsVersion::V2_1_0 => "2.1.0",
            ThreeDSecureDetailsVersion::V2_2_0 => "2.2.0",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
