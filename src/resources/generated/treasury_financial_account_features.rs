// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object};
use crate::resources::{TreasuryFinancialAccountsResourceOutboundAchToggleSettings, TreasuryFinancialAccountsResourceToggleSettings};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryFinancialAccountsResourceFinancialAccountFeatures".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountFeatures {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<TreasuryFinancialAccountsResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<TreasuryFinancialAccountsResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<TreasuryFinancialAccountsResourceInboundTransfers>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<TreasuryFinancialAccountsResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<TreasuryFinancialAccountsResourceOutboundPayments>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<TreasuryFinancialAccountsResourceOutboundTransfers>,
}

impl Object for TreasuryFinancialAccountFeatures {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "treasury.financial_account_features"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<TreasuryFinancialAccountsResourceAbaToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettings {

    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,

    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceAbaToggleSettingsStatus,

    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryFinancialAccountsResourceInboundAchToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceInboundAchToggleSettings {

    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,

    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus,

    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {

    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode,

    /// Represents what the user should do, if anything, to activate the Feature.
    pub resolution: Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution>,

    /// The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction>,
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceAbaToggleSettings`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Active => "active",
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Pending => "pending",
            TreasuryFinancialAccountsResourceAbaToggleSettingsStatus::Restricted => "restricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceInboundAchToggleSettings`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus::Active => "active",
            TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus::Pending => "pending",
            TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus::Restricted => "restricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceTogglesSettingStatusDetails`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    Activating,
    CapabilityNotRequested,
    FinancialAccountClosed,
    RejectedOther,
    RejectedUnsupportedBusiness,
    RequirementsPastDue,
    RequirementsPendingVerification,
    RestrictedByPlatform,
    RestrictedOther,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::Activating => "activating",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::CapabilityNotRequested => "capability_not_requested",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::FinancialAccountClosed => "financial_account_closed",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RejectedOther => "rejected_other",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RejectedUnsupportedBusiness => "rejected_unsupported_business",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RequirementsPastDue => "requirements_past_due",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RequirementsPendingVerification => "requirements_pending_verification",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RestrictedByPlatform => "restricted_by_platform",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::RestrictedOther => "restricted_other",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn default() -> Self {
        Self::Activating
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceTogglesSettingStatusDetails`'s `resolution` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::ContactStripe => "contact_stripe",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::ProvideInformation => "provide_information",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::RemoveRestriction => "remove_restriction",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn default() -> Self {
        Self::ContactStripe
    }
}

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceTogglesSettingStatusDetails`'s `restriction` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::InboundFlows => "inbound_flows",
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::OutboundFlows => "outbound_flows",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn default() -> Self {
        Self::InboundFlows
    }
}
