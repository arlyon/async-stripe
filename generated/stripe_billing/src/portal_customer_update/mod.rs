#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalCustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    pub allowed_updates: Vec<PortalCustomerUpdateAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
/// The types of customer updates that are supported.
///
/// When empty, customers are not updateable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
}

impl PortalCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use PortalCustomerUpdateAllowedUpdates::*;
        match self {
            Address => "address",
            Email => "email",
            Name => "name",
            Phone => "phone",
            Shipping => "shipping",
            TaxId => "tax_id",
        }
    }
}

impl std::str::FromStr for PortalCustomerUpdateAllowedUpdates {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalCustomerUpdateAllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PortalCustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalCustomerUpdateAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalCustomerUpdateAllowedUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PortalCustomerUpdateAllowedUpdates")
        })
    }
}
