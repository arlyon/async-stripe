/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://docs.stripe.com/connect/account-capabilities)
///
/// For more details see <<https://stripe.com/docs/api/capabilities/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Capability {
    /// The account for which the capability enables functionality.
    pub account: stripe_types::Expandable<stripe_shared::Account>,
    pub future_requirements: Option<stripe_shared::AccountCapabilityFutureRequirements>,
    /// The identifier for the capability.
    pub id: stripe_shared::CapabilityId,
    /// Whether the capability has been requested.
    pub requested: bool,
    /// Time at which the capability was requested. Measured in seconds since the Unix epoch.
    pub requested_at: Option<stripe_types::Timestamp>,
    pub requirements: Option<stripe_shared::AccountCapabilityRequirements>,
    /// The status of the capability.
    pub status: CapabilityStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Capability").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CapabilityBuilder {
    account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    future_requirements: Option<Option<stripe_shared::AccountCapabilityFutureRequirements>>,
    id: Option<stripe_shared::CapabilityId>,
    requested: Option<bool>,
    requested_at: Option<Option<stripe_types::Timestamp>>,
    requirements: Option<Option<stripe_shared::AccountCapabilityRequirements>>,
    status: Option<CapabilityStatus>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for Capability {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Capability>,
        builder: CapabilityBuilder,
    }

    impl Visitor for Place<Capability> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CapabilityBuilder {
                    account: Deserialize::default(),
                    future_requirements: Deserialize::default(),
                    id: Deserialize::default(),
                    requested: Deserialize::default(),
                    requested_at: Deserialize::default(),
                    requirements: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.builder.account),
                "future_requirements" => Deserialize::begin(&mut self.builder.future_requirements),
                "id" => Deserialize::begin(&mut self.builder.id),
                "requested" => Deserialize::begin(&mut self.builder.requested),
                "requested_at" => Deserialize::begin(&mut self.builder.requested_at),
                "requirements" => Deserialize::begin(&mut self.builder.requirements),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account),
                Some(future_requirements),
                Some(id),
                Some(requested),
                Some(requested_at),
                Some(requirements),
                Some(status),
            ) = (
                self.builder.account.take(),
                self.builder.future_requirements.take(),
                self.builder.id.take(),
                self.builder.requested,
                self.builder.requested_at,
                self.builder.requirements.take(),
                self.builder.status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(Capability {
                account,
                future_requirements,
                id,
                requested,
                requested_at,
                requirements,
                status,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Capability {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Capability", 8)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("future_requirements", &self.future_requirements)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("requested", &self.requested)?;
        s.serialize_field("requested_at", &self.requested_at)?;
        s.serialize_field("requirements", &self.requirements)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "capability")?;
        s.end()
    }
}
/// The status of the capability.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CapabilityStatus {
    Active,
    Inactive,
    Pending,
    Unrequested,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CapabilityStatus {
    pub fn as_str(&self) -> &str {
        use CapabilityStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
            Unrequested => "unrequested",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CapabilityStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CapabilityStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            "unrequested" => Ok(Unrequested),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CapabilityStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CapabilityStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CapabilityStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CapabilityStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CapabilityStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilityStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CapabilityStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for Capability {
    type Id = stripe_shared::CapabilityId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CapabilityId);
