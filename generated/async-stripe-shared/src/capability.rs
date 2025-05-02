/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities)
///
/// For more details see <<https://stripe.com/docs/api/capabilities/object>>.
#[derive(Clone, Debug)]
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: CapabilityBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CapabilityBuilder {
        type Out = Capability;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "future_requirements" => Deserialize::begin(&mut self.future_requirements),
                "id" => Deserialize::begin(&mut self.id),
                "requested" => Deserialize::begin(&mut self.requested),
                "requested_at" => Deserialize::begin(&mut self.requested_at),
                "requirements" => Deserialize::begin(&mut self.requirements),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                future_requirements: Deserialize::default(),
                id: Deserialize::default(),
                requested: Deserialize::default(),
                requested_at: Deserialize::default(),
                requirements: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account),
                Some(future_requirements),
                Some(id),
                Some(requested),
                Some(requested_at),
                Some(requirements),
                Some(status),
            ) = (
                self.account.take(),
                self.future_requirements.take(),
                self.id.take(),
                self.requested,
                self.requested_at,
                self.requirements.take(),
                self.status,
            )
            else {
                return None;
            };
            Some(Self::Out {
                account,
                future_requirements,
                id,
                requested,
                requested_at,
                requirements,
                status,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Capability {
        type Builder = CapabilityBuilder;
    }

    impl FromValueOpt for Capability {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CapabilityBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "future_requirements" => b.future_requirements = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "requested" => b.requested = FromValueOpt::from_value(v),
                    "requested_at" => b.requested_at = FromValueOpt::from_value(v),
                    "requirements" => b.requirements = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CapabilityStatus::*;
        match s {
            "active" => Ok(Active),
            "disabled" => Ok(Disabled),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            "unrequested" => Ok(Unrequested),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for CapabilityStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CapabilityStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CapabilityStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilityStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CapabilityStatus);
#[cfg(feature = "deserialize")]
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

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CapabilityId);
