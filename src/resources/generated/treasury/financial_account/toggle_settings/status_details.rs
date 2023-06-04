/// Additional details on the FinancialAccount Features information.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StatusDetails {
    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: StatusDetailsCode,
    /// Represents what the user should do, if anything, to activate the Feature.
    pub resolution: Option<StatusDetailsResolution>,
    /// The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<StatusDetailsRestriction>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StatusDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Represents the reason why the status is `pending` or `restricted`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum StatusDetailsCode {
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

impl StatusDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Activating => "activating",
            Self::CapabilityNotRequested => "capability_not_requested",
            Self::FinancialAccountClosed => "financial_account_closed",
            Self::RejectedOther => "rejected_other",
            Self::RejectedUnsupportedBusiness => "rejected_unsupported_business",
            Self::RequirementsPastDue => "requirements_past_due",
            Self::RequirementsPendingVerification => "requirements_pending_verification",
            Self::RestrictedByPlatform => "restricted_by_platform",
            Self::RestrictedOther => "restricted_other",
        }
    }
}

impl AsRef<str> for StatusDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Represents what the user should do, if anything, to activate the Feature.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum StatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

impl StatusDetailsResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContactStripe => "contact_stripe",
            Self::ProvideInformation => "provide_information",
            Self::RemoveRestriction => "remove_restriction",
        }
    }
}

impl AsRef<str> for StatusDetailsResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The `platform_restrictions` that are restricting this Feature.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum StatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

impl StatusDetailsRestriction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InboundFlows => "inbound_flows",
            Self::OutboundFlows => "outbound_flows",
        }
    }
}

impl AsRef<str> for StatusDetailsRestriction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
