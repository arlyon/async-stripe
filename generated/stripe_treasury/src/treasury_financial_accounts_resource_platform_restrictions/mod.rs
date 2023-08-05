/// Restrictions that a Connect Platform has placed on this FinancialAccount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {
    /// Restricts all inbound money movement.
    pub inbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>,
    /// Restricts all outbound money movement.
    pub outbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows>,
}
/// Restricts all inbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows"))
    }
}
/// Restricts all outbound money movement.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows::*;
        match self {
            Restricted => "restricted",
            Unrestricted => "unrestricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows::*;
        match s {
            "restricted" => Ok(Restricted),
            "unrestricted" => Ok(Unrestricted),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows"))
    }
}
