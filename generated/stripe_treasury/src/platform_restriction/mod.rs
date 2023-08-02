/// Restrictions that a Connect Platform has placed on this FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PlatformRestriction {
    /// Restricts all inbound money movement.
    pub inbound_flows: Option<PlatformRestrictionInboundFlows>,
    /// Restricts all outbound money movement.
    pub outbound_flows: Option<PlatformRestrictionOutboundFlows>,
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlatformRestrictionInboundFlows {
    Restricted,
    Unrestricted,
}

impl PlatformRestrictionInboundFlows {
    pub fn as_str(self) -> &'static str {
        use PlatformRestrictionInboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for PlatformRestrictionInboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlatformRestrictionInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
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
impl serde::Serialize for PlatformRestrictionInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlatformRestrictionInboundFlows {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PlatformRestrictionInboundFlows")
        })
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlatformRestrictionOutboundFlows {
    Restricted,
    Unrestricted,
}

impl PlatformRestrictionOutboundFlows {
    pub fn as_str(self) -> &'static str {
        use PlatformRestrictionOutboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for PlatformRestrictionOutboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlatformRestrictionOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
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
impl serde::Serialize for PlatformRestrictionOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlatformRestrictionOutboundFlows {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PlatformRestrictionOutboundFlows")
        })
    }
}
