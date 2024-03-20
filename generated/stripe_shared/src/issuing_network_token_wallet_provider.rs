#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingNetworkTokenWalletProvider {
    /// The wallet provider-given account ID of the digital wallet the token belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// An evaluation on the trustworthiness of the wallet account between 1 and 5.
    /// A higher score indicates more trustworthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_trust_score: Option<i64>,
    /// The method used for tokenizing a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number_source: Option<IssuingNetworkTokenWalletProviderCardNumberSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_address: Option<stripe_shared::IssuingNetworkTokenAddress>,
    /// The name of the cardholder tokenizing the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    /// An evaluation on the trustworthiness of the device. A higher score indicates more trustworthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_trust_score: Option<i64>,
    /// The hashed email address of the cardholder's account with the wallet provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashed_account_email_address: Option<String>,
    /// The reasons for suggested tokenization given by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<IssuingNetworkTokenWalletProviderReasonCodes>>,
    /// The recommendation on responding to the tokenization request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_decision: Option<IssuingNetworkTokenWalletProviderSuggestedDecision>,
    /// The version of the standard for mapping reason codes followed by the wallet provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_decision_version: Option<String>,
}
/// The method used for tokenizing a card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenWalletProviderCardNumberSource {
    App,
    Manual,
    OnFile,
    Other,
}
impl IssuingNetworkTokenWalletProviderCardNumberSource {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenWalletProviderCardNumberSource::*;
        match self {
            App => "app",
            Manual => "manual",
            OnFile => "on_file",
            Other => "other",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenWalletProviderCardNumberSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenWalletProviderCardNumberSource::*;
        match s {
            "app" => Ok(App),
            "manual" => Ok(Manual),
            "on_file" => Ok(OnFile),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingNetworkTokenWalletProviderCardNumberSource",
            )
        })
    }
}
/// The reasons for suggested tokenization given by the card network.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingNetworkTokenWalletProviderReasonCodes {
    AccountCardTooNew,
    AccountRecentlyChanged,
    AccountTooNew,
    AccountTooNewSinceLaunch,
    AdditionalDevice,
    DataExpired,
    DeferIdVDecision,
    DeviceRecentlyLost,
    GoodActivityHistory,
    HasSuspendedTokens,
    HighRisk,
    InactiveAccount,
    LongAccountTenure,
    LowAccountScore,
    LowDeviceScore,
    LowPhoneNumberScore,
    NetworkServiceError,
    OutsideHomeTerritory,
    ProvisioningCardholderMismatch,
    ProvisioningDeviceAndCardholderMismatch,
    ProvisioningDeviceMismatch,
    SameDeviceNoPriorAuthentication,
    SameDeviceSuccessfulPriorAuthentication,
    SoftwareUpdate,
    SuspiciousActivity,
    TooManyDifferentCardholders,
    TooManyRecentAttempts,
    TooManyRecentTokens,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl IssuingNetworkTokenWalletProviderReasonCodes {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenWalletProviderReasonCodes::*;
        match self {
            AccountCardTooNew => "account_card_too_new",
            AccountRecentlyChanged => "account_recently_changed",
            AccountTooNew => "account_too_new",
            AccountTooNewSinceLaunch => "account_too_new_since_launch",
            AdditionalDevice => "additional_device",
            DataExpired => "data_expired",
            DeferIdVDecision => "defer_id_v_decision",
            DeviceRecentlyLost => "device_recently_lost",
            GoodActivityHistory => "good_activity_history",
            HasSuspendedTokens => "has_suspended_tokens",
            HighRisk => "high_risk",
            InactiveAccount => "inactive_account",
            LongAccountTenure => "long_account_tenure",
            LowAccountScore => "low_account_score",
            LowDeviceScore => "low_device_score",
            LowPhoneNumberScore => "low_phone_number_score",
            NetworkServiceError => "network_service_error",
            OutsideHomeTerritory => "outside_home_territory",
            ProvisioningCardholderMismatch => "provisioning_cardholder_mismatch",
            ProvisioningDeviceAndCardholderMismatch => {
                "provisioning_device_and_cardholder_mismatch"
            }
            ProvisioningDeviceMismatch => "provisioning_device_mismatch",
            SameDeviceNoPriorAuthentication => "same_device_no_prior_authentication",
            SameDeviceSuccessfulPriorAuthentication => {
                "same_device_successful_prior_authentication"
            }
            SoftwareUpdate => "software_update",
            SuspiciousActivity => "suspicious_activity",
            TooManyDifferentCardholders => "too_many_different_cardholders",
            TooManyRecentAttempts => "too_many_recent_attempts",
            TooManyRecentTokens => "too_many_recent_tokens",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenWalletProviderReasonCodes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenWalletProviderReasonCodes::*;
        match s {
            "account_card_too_new" => Ok(AccountCardTooNew),
            "account_recently_changed" => Ok(AccountRecentlyChanged),
            "account_too_new" => Ok(AccountTooNew),
            "account_too_new_since_launch" => Ok(AccountTooNewSinceLaunch),
            "additional_device" => Ok(AdditionalDevice),
            "data_expired" => Ok(DataExpired),
            "defer_id_v_decision" => Ok(DeferIdVDecision),
            "device_recently_lost" => Ok(DeviceRecentlyLost),
            "good_activity_history" => Ok(GoodActivityHistory),
            "has_suspended_tokens" => Ok(HasSuspendedTokens),
            "high_risk" => Ok(HighRisk),
            "inactive_account" => Ok(InactiveAccount),
            "long_account_tenure" => Ok(LongAccountTenure),
            "low_account_score" => Ok(LowAccountScore),
            "low_device_score" => Ok(LowDeviceScore),
            "low_phone_number_score" => Ok(LowPhoneNumberScore),
            "network_service_error" => Ok(NetworkServiceError),
            "outside_home_territory" => Ok(OutsideHomeTerritory),
            "provisioning_cardholder_mismatch" => Ok(ProvisioningCardholderMismatch),
            "provisioning_device_and_cardholder_mismatch" => {
                Ok(ProvisioningDeviceAndCardholderMismatch)
            }
            "provisioning_device_mismatch" => Ok(ProvisioningDeviceMismatch),
            "same_device_no_prior_authentication" => Ok(SameDeviceNoPriorAuthentication),
            "same_device_successful_prior_authentication" => {
                Ok(SameDeviceSuccessfulPriorAuthentication)
            }
            "software_update" => Ok(SoftwareUpdate),
            "suspicious_activity" => Ok(SuspiciousActivity),
            "too_many_different_cardholders" => Ok(TooManyDifferentCardholders),
            "too_many_recent_attempts" => Ok(TooManyRecentAttempts),
            "too_many_recent_tokens" => Ok(TooManyRecentTokens),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenWalletProviderReasonCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenWalletProviderReasonCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenWalletProviderReasonCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenWalletProviderReasonCodes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(IssuingNetworkTokenWalletProviderReasonCodes::Unknown))
    }
}
/// The recommendation on responding to the tokenization request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenWalletProviderSuggestedDecision {
    Approve,
    Decline,
    RequireAuth,
}
impl IssuingNetworkTokenWalletProviderSuggestedDecision {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenWalletProviderSuggestedDecision::*;
        match self {
            Approve => "approve",
            Decline => "decline",
            RequireAuth => "require_auth",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenWalletProviderSuggestedDecision {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenWalletProviderSuggestedDecision::*;
        match s {
            "approve" => Ok(Approve),
            "decline" => Ok(Decline),
            "require_auth" => Ok(RequireAuth),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingNetworkTokenWalletProviderSuggestedDecision",
            )
        })
    }
}
