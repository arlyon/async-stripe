/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Capability {
    /// The account for which the capability enables functionality.
    pub account: crate::Expandable<crate::account::Account>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<crate::capability::future_requirements::FutureRequirements>,
    /// The identifier for the capability.
    pub id: crate::capability::CapabilityId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CapabilityObject,
    /// Whether the capability has been requested.
    pub requested: bool,
    /// Time at which the capability was requested.
    ///
    /// Measured in seconds since the Unix epoch.
    pub requested_at: Option<crate::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<crate::capability::requirements::Requirements>,
    /// The status of the capability.
    ///
    /// Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: CapabilityStatus,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Capability {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// The status of the capability.
///
/// Can be `active`, `inactive`, `pending`, or `unrequested`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for Capability {
    type Id = crate::capability::CapabilityId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(CapabilityId);
pub mod future_requirements;
pub mod requests;
pub use future_requirements::FutureRequirements;
pub mod requirements;
pub use requirements::Requirements;
