// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingTokenId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::IssuingCard;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingNetworkToken".
///
/// For more details see <https://stripe.com/docs/api/issuing/tokens/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingToken {
    /// Unique identifier for the object.
    pub id: IssuingTokenId,

    /// Card associated with this token.
    pub card: Expandable<IssuingCard>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The hashed ID derived from the device ID from the card network associated with the token.
    pub device_fingerprint: Option<String>,

    /// The last four digits of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The token service provider / card network associated with the token.
    pub network: IssuingTokenNetwork,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<IssuingNetworkTokenNetworkData>,

    /// Time at which the token was last updated by the card network.
    ///
    /// Measured in seconds since the Unix epoch.
    pub network_updated_at: Timestamp,

    /// The usage state of the token.
    pub status: IssuingTokenStatus,

    /// The digital wallet for this token, if one was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<IssuingTokenWalletProvider>,
}

impl Object for IssuingToken {
    type Id = IssuingTokenId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.token"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingNetworkTokenNetworkData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<IssuingNetworkTokenDevice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<IssuingNetworkTokenMastercard>,

    /// The network that the token is associated with.
    ///
    /// An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
    #[serde(rename = "type")]
    pub type_: IssuingNetworkTokenNetworkDataType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<IssuingNetworkTokenVisa>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<IssuingNetworkTokenWalletProvider>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingNetworkTokenDevice {
    /// An obfuscated ID derived from the device ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,

    /// The IP address of the device at provisioning time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// The geographic latitude/longitude coordinates of the device at provisioning time.
    ///
    /// The format is [+-]decimal/[+-]decimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// The name of the device used for tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The phone number of the device used for tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    /// The type of device used for tokenization.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<IssuingNetworkTokenDeviceType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingNetworkTokenMastercard {
    /// A unique reference ID from MasterCard to represent the card account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_reference_id: Option<String>,

    /// The network-unique identifier for the token.
    pub token_reference_id: String,

    /// The ID of the entity requesting tokenization, specific to MasterCard.
    pub token_requestor_id: String,

    /// The name of the entity requesting tokenization, if known.
    ///
    /// This is directly provided from MasterCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_requestor_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingNetworkTokenVisa {
    /// A unique reference ID from Visa to represent the card account number.
    pub card_reference_id: String,

    /// The network-unique identifier for the token.
    pub token_reference_id: String,

    /// The ID of the entity requesting tokenization, specific to Visa.
    pub token_requestor_id: String,

    /// Degree of risk associated with the token between `01` and `99`, with higher number indicating higher risk.
    ///
    /// A `00` value indicates the token was not scored by Visa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_risk_score: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingNetworkTokenWalletProvider {
    /// The wallet provider-given account ID of the digital wallet the token belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// An evaluation on the trustworthiness of the wallet account between 1 and 5.
    ///
    /// A higher score indicates more trustworthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_trust_score: Option<i64>,

    /// The method used for tokenizing a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number_source: Option<IssuingNetworkTokenWalletProviderCardNumberSource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_address: Option<IssuingNetworkTokenAddress>,

    /// The name of the cardholder tokenizing the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,

    /// An evaluation on the trustworthiness of the device.
    ///
    /// A higher score indicates more trustworthy.
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingNetworkTokenAddress {
    /// The street address of the cardholder tokenizing the card.
    pub line1: String,

    /// The postal code of the cardholder tokenizing the card.
    pub postal_code: String,
}

/// An enum representing the possible values of an `IssuingNetworkTokenDevice`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenDeviceType {
    Other,
    Phone,
    Watch,
}

impl IssuingNetworkTokenDeviceType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingNetworkTokenDeviceType::Other => "other",
            IssuingNetworkTokenDeviceType::Phone => "phone",
            IssuingNetworkTokenDeviceType::Watch => "watch",
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenDeviceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingNetworkTokenDeviceType {
    fn default() -> Self {
        Self::Other
    }
}

/// An enum representing the possible values of an `IssuingNetworkTokenNetworkData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenNetworkDataType {
    Mastercard,
    Visa,
}

impl IssuingNetworkTokenNetworkDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingNetworkTokenNetworkDataType::Mastercard => "mastercard",
            IssuingNetworkTokenNetworkDataType::Visa => "visa",
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenNetworkDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenNetworkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingNetworkTokenNetworkDataType {
    fn default() -> Self {
        Self::Mastercard
    }
}

/// An enum representing the possible values of an `IssuingNetworkTokenWalletProvider`'s `card_number_source` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenWalletProviderCardNumberSource {
    App,
    Manual,
    OnFile,
    Other,
}

impl IssuingNetworkTokenWalletProviderCardNumberSource {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingNetworkTokenWalletProviderCardNumberSource::App => "app",
            IssuingNetworkTokenWalletProviderCardNumberSource::Manual => "manual",
            IssuingNetworkTokenWalletProviderCardNumberSource::OnFile => "on_file",
            IssuingNetworkTokenWalletProviderCardNumberSource::Other => "other",
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn default() -> Self {
        Self::App
    }
}

/// An enum representing the possible values of an `IssuingNetworkTokenWalletProvider`'s `reason_codes` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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
}

impl IssuingNetworkTokenWalletProviderReasonCodes {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingNetworkTokenWalletProviderReasonCodes::AccountCardTooNew => "account_card_too_new",
            IssuingNetworkTokenWalletProviderReasonCodes::AccountRecentlyChanged => "account_recently_changed",
            IssuingNetworkTokenWalletProviderReasonCodes::AccountTooNew => "account_too_new",
            IssuingNetworkTokenWalletProviderReasonCodes::AccountTooNewSinceLaunch => "account_too_new_since_launch",
            IssuingNetworkTokenWalletProviderReasonCodes::AdditionalDevice => "additional_device",
            IssuingNetworkTokenWalletProviderReasonCodes::DataExpired => "data_expired",
            IssuingNetworkTokenWalletProviderReasonCodes::DeferIdVDecision => "defer_id_v_decision",
            IssuingNetworkTokenWalletProviderReasonCodes::DeviceRecentlyLost => "device_recently_lost",
            IssuingNetworkTokenWalletProviderReasonCodes::GoodActivityHistory => "good_activity_history",
            IssuingNetworkTokenWalletProviderReasonCodes::HasSuspendedTokens => "has_suspended_tokens",
            IssuingNetworkTokenWalletProviderReasonCodes::HighRisk => "high_risk",
            IssuingNetworkTokenWalletProviderReasonCodes::InactiveAccount => "inactive_account",
            IssuingNetworkTokenWalletProviderReasonCodes::LongAccountTenure => "long_account_tenure",
            IssuingNetworkTokenWalletProviderReasonCodes::LowAccountScore => "low_account_score",
            IssuingNetworkTokenWalletProviderReasonCodes::LowDeviceScore => "low_device_score",
            IssuingNetworkTokenWalletProviderReasonCodes::LowPhoneNumberScore => "low_phone_number_score",
            IssuingNetworkTokenWalletProviderReasonCodes::NetworkServiceError => "network_service_error",
            IssuingNetworkTokenWalletProviderReasonCodes::OutsideHomeTerritory => "outside_home_territory",
            IssuingNetworkTokenWalletProviderReasonCodes::ProvisioningCardholderMismatch => "provisioning_cardholder_mismatch",
            IssuingNetworkTokenWalletProviderReasonCodes::ProvisioningDeviceAndCardholderMismatch => "provisioning_device_and_cardholder_mismatch",
            IssuingNetworkTokenWalletProviderReasonCodes::ProvisioningDeviceMismatch => "provisioning_device_mismatch",
            IssuingNetworkTokenWalletProviderReasonCodes::SameDeviceNoPriorAuthentication => "same_device_no_prior_authentication",
            IssuingNetworkTokenWalletProviderReasonCodes::SameDeviceSuccessfulPriorAuthentication => "same_device_successful_prior_authentication",
            IssuingNetworkTokenWalletProviderReasonCodes::SoftwareUpdate => "software_update",
            IssuingNetworkTokenWalletProviderReasonCodes::SuspiciousActivity => "suspicious_activity",
            IssuingNetworkTokenWalletProviderReasonCodes::TooManyDifferentCardholders => "too_many_different_cardholders",
            IssuingNetworkTokenWalletProviderReasonCodes::TooManyRecentAttempts => "too_many_recent_attempts",
            IssuingNetworkTokenWalletProviderReasonCodes::TooManyRecentTokens => "too_many_recent_tokens",
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenWalletProviderReasonCodes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenWalletProviderReasonCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingNetworkTokenWalletProviderReasonCodes {
    fn default() -> Self {
        Self::AccountCardTooNew
    }
}

/// An enum representing the possible values of an `IssuingNetworkTokenWalletProvider`'s `suggested_decision` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenWalletProviderSuggestedDecision {
    Approve,
    Decline,
    RequireAuth,
}

impl IssuingNetworkTokenWalletProviderSuggestedDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingNetworkTokenWalletProviderSuggestedDecision::Approve => "approve",
            IssuingNetworkTokenWalletProviderSuggestedDecision::Decline => "decline",
            IssuingNetworkTokenWalletProviderSuggestedDecision::RequireAuth => "require_auth",
        }
    }
}

impl AsRef<str> for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn default() -> Self {
        Self::Approve
    }
}

/// An enum representing the possible values of an `IssuingToken`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTokenNetwork {
    Mastercard,
    Visa,
}

impl IssuingTokenNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingTokenNetwork::Mastercard => "mastercard",
            IssuingTokenNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for IssuingTokenNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTokenNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingTokenNetwork {
    fn default() -> Self {
        Self::Mastercard
    }
}

/// An enum representing the possible values of an `IssuingToken`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
}

impl IssuingTokenStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingTokenStatus::Active => "active",
            IssuingTokenStatus::Deleted => "deleted",
            IssuingTokenStatus::Requested => "requested",
            IssuingTokenStatus::Suspended => "suspended",
        }
    }
}

impl AsRef<str> for IssuingTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingTokenStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `IssuingToken`'s `wallet_provider` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTokenWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl IssuingTokenWalletProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingTokenWalletProvider::ApplePay => "apple_pay",
            IssuingTokenWalletProvider::GooglePay => "google_pay",
            IssuingTokenWalletProvider::SamsungPay => "samsung_pay",
        }
    }
}

impl AsRef<str> for IssuingTokenWalletProvider {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingTokenWalletProvider {
    fn default() -> Self {
        Self::ApplePay
    }
}
