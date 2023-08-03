/// Additional details on the FinancialAccount Features information.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode,
    /// Represents what the user should do, if anything, to activate the Feature.
    pub resolution: Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution>,
    /// The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction>,
}
/// Represents the reason why the status is `pending` or `restricted`.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::*;
        match self {
            Activating => "activating",
            CapabilityNotRequested => "capability_not_requested",
            FinancialAccountClosed => "financial_account_closed",
            RejectedOther => "rejected_other",
            RejectedUnsupportedBusiness => "rejected_unsupported_business",
            RequirementsPastDue => "requirements_past_due",
            RequirementsPendingVerification => "requirements_pending_verification",
            RestrictedByPlatform => "restricted_by_platform",
            RestrictedOther => "restricted_other",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::*;
        match s {
            "activating" => Ok(Activating),
            "capability_not_requested" => Ok(CapabilityNotRequested),
            "financial_account_closed" => Ok(FinancialAccountClosed),
            "rejected_other" => Ok(RejectedOther),
            "rejected_unsupported_business" => Ok(RejectedUnsupportedBusiness),
            "requirements_past_due" => Ok(RequirementsPastDue),
            "requirements_pending_verification" => Ok(RequirementsPendingVerification),
            "restricted_by_platform" => Ok(RestrictedByPlatform),
            "restricted_other" => Ok(RestrictedOther),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode"))
    }
}
/// Represents what the user should do, if anything, to activate the Feature.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::*;
        match self {
            ContactStripe => "contact_stripe",
            ProvideInformation => "provide_information",
            RemoveRestriction => "remove_restriction",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::*;
        match s {
            "contact_stripe" => Ok(ContactStripe),
            "provide_information" => Ok(ProvideInformation),
            "remove_restriction" => Ok(RemoveRestriction),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution"))
    }
}
/// The `platform_restrictions` that are restricting this Feature.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::*;
        match self {
            InboundFlows => "inbound_flows",
            OutboundFlows => "outbound_flows",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::*;
        match s {
            "inbound_flows" => Ok(InboundFlows),
            "outbound_flows" => Ok(OutboundFlows),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction"))
    }
}
