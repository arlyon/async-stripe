// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "AccountServiceResourceToggleSettings".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,

    /// Whether the Feature is operational.
    pub status: AccountServiceResourceToggleSettingsStatus,

    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details: Vec<AccountServiceResourceStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountServiceResourceStatusDetails {
    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: AccountServiceResourceStatusDetailsCode,

    /// Represents what the user should do, if anything, to activate the Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<AccountServiceResourceStatusDetailsResolution>,

    /// The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<AccountServiceResourceStatusDetailsRestriction>,
}

/// An enum representing the possible values of an `AccountServiceResourceStatusDetails`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceStatusDetailsCode {
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

impl AccountServiceResourceStatusDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceStatusDetailsCode::Activating => "activating",
            AccountServiceResourceStatusDetailsCode::CapabilityNotRequested => {
                "capability_not_requested"
            }
            AccountServiceResourceStatusDetailsCode::FinancialAccountClosed => {
                "financial_account_closed"
            }
            AccountServiceResourceStatusDetailsCode::RejectedOther => "rejected_other",
            AccountServiceResourceStatusDetailsCode::RejectedUnsupportedBusiness => {
                "rejected_unsupported_business"
            }
            AccountServiceResourceStatusDetailsCode::RequirementsPastDue => "requirements_past_due",
            AccountServiceResourceStatusDetailsCode::RequirementsPendingVerification => {
                "requirements_pending_verification"
            }
            AccountServiceResourceStatusDetailsCode::RestrictedByPlatform => {
                "restricted_by_platform"
            }
            AccountServiceResourceStatusDetailsCode::RestrictedOther => "restricted_other",
        }
    }
}

impl AsRef<str> for AccountServiceResourceStatusDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceStatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceStatusDetailsCode {
    fn default() -> Self {
        Self::Activating
    }
}

/// An enum representing the possible values of an `AccountServiceResourceStatusDetails`'s `resolution` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

impl AccountServiceResourceStatusDetailsResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceStatusDetailsResolution::ContactStripe => "contact_stripe",
            AccountServiceResourceStatusDetailsResolution::ProvideInformation => {
                "provide_information"
            }
            AccountServiceResourceStatusDetailsResolution::RemoveRestriction => {
                "remove_restriction"
            }
        }
    }
}

impl AsRef<str> for AccountServiceResourceStatusDetailsResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceStatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceStatusDetailsResolution {
    fn default() -> Self {
        Self::ContactStripe
    }
}

/// An enum representing the possible values of an `AccountServiceResourceStatusDetails`'s `restriction` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

impl AccountServiceResourceStatusDetailsRestriction {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceStatusDetailsRestriction::InboundFlows => "inbound_flows",
            AccountServiceResourceStatusDetailsRestriction::OutboundFlows => "outbound_flows",
        }
    }
}

impl AsRef<str> for AccountServiceResourceStatusDetailsRestriction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceStatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceStatusDetailsRestriction {
    fn default() -> Self {
        Self::InboundFlows
    }
}

/// An enum representing the possible values of an `AccountServiceResourceToggleSettings`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountServiceResourceToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl AccountServiceResourceToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountServiceResourceToggleSettingsStatus::Active => "active",
            AccountServiceResourceToggleSettingsStatus::Pending => "pending",
            AccountServiceResourceToggleSettingsStatus::Restricted => "restricted",
        }
    }
}

impl AsRef<str> for AccountServiceResourceToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountServiceResourceToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountServiceResourceToggleSettingsStatus {
    fn default() -> Self {
        Self::Active
    }
}
