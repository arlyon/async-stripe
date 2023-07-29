/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Capability {
    /// The account for which the capability enables functionality.
    pub account: stripe_types::Expandable<stripe_types::account::Account>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements:
        Option<stripe_connect::capability::future_requirements::FutureRequirements>,
    /// The identifier for the capability.
    pub id: stripe_connect::capability::CapabilityId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CapabilityObject,
    /// Whether the capability has been requested.
    pub requested: bool,
    /// Time at which the capability was requested.
    ///
    /// Measured in seconds since the Unix epoch.
    pub requested_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_connect::capability::requirements::Requirements>,
    /// The status of the capability.
    ///
    /// Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: CapabilityStatus,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilityObject {
    Capability,
}

impl CapabilityObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Capability => "capability",
        }
    }
}

impl std::str::FromStr for CapabilityObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "capability" => Ok(Self::Capability),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilityObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilityObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilityObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilityObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilityObject"))
    }
}
/// The status of the capability.
///
/// Can be `active`, `inactive`, `pending`, or `unrequested`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilityStatus {
    Active,
    Disabled,
    Inactive,
    Pending,
    Unrequested,
}

impl CapabilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
            Self::Unrequested => "unrequested",
        }
    }
}

impl std::str::FromStr for CapabilityStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),
            "unrequested" => Ok(Self::Unrequested),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilityStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilityStatus"))
    }
}
impl stripe_types::Object for Capability {
    type Id = stripe_connect::capability::CapabilityId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CapabilityId);
pub mod future_requirements;
pub use future_requirements::FutureRequirements;
pub mod requirements;
pub use requirements::Requirements;
