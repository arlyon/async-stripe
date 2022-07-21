// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "AccountServiceResourceTreasuryToggleSettings".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,

    /// Whether the Feature is operational.
    pub status: AccountServiceResourceTreasuryToggleSettingsStatus,

    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<AccountServiceResourceTreasuryTogglesSettingStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceTreasuryTogglesSettingStatusDetails {
    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode,

    /// Represents what the user should do, if anything, to activate the Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution>,

    /// The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction>,
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryToggleSettings`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl AccountServiceResourceTreasuryToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryToggleSettingsStatus::Active => "active",
            AccountServiceResourceTreasuryToggleSettingsStatus::Pending => "pending",
            AccountServiceResourceTreasuryToggleSettingsStatus::Restricted => "restricted",
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryToggleSettingsStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryTogglesSettingStatusDetails`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode {
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

impl AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::Activating => "activating",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::CapabilityNotRequested => "capability_not_requested",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::FinancialAccountClosed => "financial_account_closed",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::RejectedOther => "rejected_other",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::RejectedUnsupportedBusiness => "rejected_unsupported_business",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::RequirementsPastDue => "requirements_past_due",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::RequirementsPendingVerification => "requirements_pending_verification",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::RestrictedByPlatform => "restricted_by_platform",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode::RestrictedOther => "restricted_other",
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryTogglesSettingStatusDetailsCode {
    fn default() -> Self {
        Self::Activating
    }
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryTogglesSettingStatusDetails`'s `resolution` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

impl AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution::ContactStripe => "contact_stripe",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution::ProvideInformation => "provide_information",
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution::RemoveRestriction => "remove_restriction",
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceTreasuryTogglesSettingStatusDetailsResolution {
    fn default() -> Self {
        Self::ContactStripe
    }
}

/// An enum representing the possible values of an `AccountServiceResourceTreasuryTogglesSettingStatusDetails`'s `restriction` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

impl AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction::InboundFlows => {
                "inbound_flows"
            }
            AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction::OutboundFlows => {
                "outbound_flows"
            }
        }
    }
}

impl AsRef<str> for AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for AccountServiceResourceTreasuryTogglesSettingStatusDetailsRestriction
{
    fn default() -> Self {
        Self::InboundFlows
    }
}
