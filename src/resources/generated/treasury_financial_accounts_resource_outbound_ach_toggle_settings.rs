// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryFinancialAccountsResourceOutboundAchToggleSettings".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryFinancialAccountsResourceOutboundAchToggleSettings {

    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,

    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus,

    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
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

/// An enum representing the possible values of an `TreasuryFinancialAccountsResourceOutboundAchToggleSettings`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus::Active => "active",
            TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus::Pending => "pending",
            TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus::Restricted => "restricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
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
