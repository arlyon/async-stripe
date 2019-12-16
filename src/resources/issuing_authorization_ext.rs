use serde_derive::{Deserialize, Serialize};

/// An enum representing the possible values of the `IssuingAuthorizationVerificationData` fields.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationCheck {
    Match,
    Mismatch,
    NotProvided,
}

/// An enum representing the possible values of the `IssuingAuthorization`'s `authorization_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationMethod {
    KeyedIn,
    Swipe,
    Chip,
    Contactless,
    Online,
}

/// An enum representing the possible values of the `IssuingAuthorizationRequest`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationReason {
    AuthenticationFailed,
    AuthorizationControls,
    CardActive,
    CardInactive,
    InsufficientFunds,
    AccountComplianceDisabled,
    AccountInactive,
    SuspectedFraud,
    WebhookApproved,
    WebhookDeclined,
    WebhookTimeout,
}

/// An enum representing the possible values of an `IssuingAuthorization`'s `wallet_provider` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl IssuingAuthorizationWalletProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingAuthorizationWalletProvider::ApplePay => "apple_pay",
            IssuingAuthorizationWalletProvider::GooglePay => "google_pay",
            IssuingAuthorizationWalletProvider::SamsungPay => "samsung_pay",
        }
    }
}

impl AsRef<str> for IssuingAuthorizationWalletProvider {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingAuthorizationWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
