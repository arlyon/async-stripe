/// Restrictions that a Connect Platform has placed on this FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PlatformRestriction {
    /// Restricts all inbound money movement.
    pub inbound_flows: Option<PlatformRestrictionInboundFlows>,
    /// Restricts all outbound money movement.
    pub outbound_flows: Option<PlatformRestrictionOutboundFlows>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlatformRestriction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Restricts all inbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PlatformRestrictionInboundFlows {
    Restricted,
    Unrestricted,
}

impl PlatformRestrictionInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for PlatformRestrictionInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlatformRestrictionInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PlatformRestrictionOutboundFlows {
    Restricted,
    Unrestricted,
}

impl PlatformRestrictionOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for PlatformRestrictionOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlatformRestrictionOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
