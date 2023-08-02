/// Additional details on the FinancialAccount Features information.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StatusDetails {
    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: StatusDetailsCode,
    /// Represents what the user should do, if anything, to activate the Feature.
    pub resolution: Option<StatusDetailsResolution>,
    /// The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<StatusDetailsRestriction>,
}
/// Represents the reason why the status is `pending` or `restricted`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        use StatusDetailsCode::*;
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

impl std::str::FromStr for StatusDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatusDetailsCode::*;
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
impl serde::Serialize for StatusDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StatusDetailsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for StatusDetailsCode"))
    }
}
/// Represents what the user should do, if anything, to activate the Feature.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

impl StatusDetailsResolution {
    pub fn as_str(self) -> &'static str {
        use StatusDetailsResolution::*;
        match self {
            ContactStripe => "contact_stripe",
            ProvideInformation => "provide_information",
            RemoveRestriction => "remove_restriction",
        }
    }
}

impl std::str::FromStr for StatusDetailsResolution {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatusDetailsResolution::*;
        match s {
            "contact_stripe" => Ok(ContactStripe),
            "provide_information" => Ok(ProvideInformation),
            "remove_restriction" => Ok(RemoveRestriction),
            _ => Err(()),
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
impl serde::Serialize for StatusDetailsResolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StatusDetailsResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for StatusDetailsResolution"))
    }
}
/// The `platform_restrictions` that are restricting this Feature.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

impl StatusDetailsRestriction {
    pub fn as_str(self) -> &'static str {
        use StatusDetailsRestriction::*;
        match self {
            InboundFlows => "inbound_flows",
            OutboundFlows => "outbound_flows",
        }
    }
}

impl std::str::FromStr for StatusDetailsRestriction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatusDetailsRestriction::*;
        match s {
            "inbound_flows" => Ok(InboundFlows),
            "outbound_flows" => Ok(OutboundFlows),
            _ => Err(()),
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
impl serde::Serialize for StatusDetailsRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StatusDetailsRestriction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for StatusDetailsRestriction"))
    }
}
