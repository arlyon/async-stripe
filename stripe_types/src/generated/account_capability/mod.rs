/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountCapability {
    /// The account for which the capability enables functionality.
    pub account: stripe_types::Expandable<stripe_types::Account>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types::AccountCapabilityFutureRequirements>,
    /// The identifier for the capability.
    pub id: stripe_types::account_capability::CapabilityId,
    /// Whether the capability has been requested.
    pub requested: bool,
    /// Time at which the capability was requested.
    ///
    /// Measured in seconds since the Unix epoch.
    pub requested_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types::AccountCapabilityRequirements>,
    /// The status of the capability.
    ///
    /// Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: AccountCapabilityStatus,
}
/// The status of the capability.
///
/// Can be `active`, `inactive`, `pending`, or `unrequested`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilityStatus {
    Active,
    Disabled,
    Inactive,
    Pending,
    Unrequested,
}

impl AccountCapabilityStatus {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilityStatus::*;
        match self {
            Active => "active",
            Disabled => "disabled",
            Inactive => "inactive",
            Pending => "pending",
            Unrequested => "unrequested",
        }
    }
}

impl std::str::FromStr for AccountCapabilityStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilityStatus::*;
        match s {
            "active" => Ok(Active),
            "disabled" => Ok(Disabled),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            "unrequested" => Ok(Unrequested),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilityStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilityStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilityStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountCapabilityStatus"))
    }
}
impl stripe_types::Object for AccountCapability {
    type Id = stripe_types::account_capability::CapabilityId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(CapabilityId);
