/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities)
///
/// For more details see <<https://stripe.com/docs/api/capabilities/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Capability {
    /// The account for which the capability enables functionality.
    pub account: stripe_types::Expandable<stripe_shared::Account>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_shared::AccountCapabilityFutureRequirements>,
    /// The identifier for the capability.
    pub id: stripe_shared::CapabilityId,
    /// Whether the capability has been requested.
    pub requested: bool,
    /// Time at which the capability was requested. Measured in seconds since the Unix epoch.
    pub requested_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_shared::AccountCapabilityRequirements>,
    /// The status of the capability. Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: CapabilityStatus,
}
/// The status of the capability. Can be `active`, `inactive`, `pending`, or `unrequested`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CapabilityStatus {
    Active,
    Disabled,
    Inactive,
    Pending,
    Unrequested,
}
impl CapabilityStatus {
    pub fn as_str(self) -> &'static str {
        use CapabilityStatus::*;
        match self {
            Active => "active",
            Disabled => "disabled",
            Inactive => "inactive",
            Pending => "pending",
            Unrequested => "unrequested",
        }
    }
}

impl std::str::FromStr for CapabilityStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CapabilityStatus::*;
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
impl std::fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CapabilityStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilityStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilityStatus"))
    }
}
impl stripe_types::Object for Capability {
    type Id = stripe_shared::CapabilityId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CapabilityId);
