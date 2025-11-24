#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalCustomerUpdate {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    pub allowed_updates: Vec<PortalCustomerUpdateAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[doc(hidden)]
pub struct PortalCustomerUpdateBuilder {
    allowed_updates: Option<Vec<PortalCustomerUpdateAllowedUpdates>>,
    enabled: Option<bool>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalCustomerUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalCustomerUpdate>,
        builder: PortalCustomerUpdateBuilder,
    }

    impl Visitor for Place<PortalCustomerUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalCustomerUpdateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalCustomerUpdateBuilder {
        type Out = PortalCustomerUpdate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allowed_updates" => Deserialize::begin(&mut self.allowed_updates),
                "enabled" => Deserialize::begin(&mut self.enabled),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { allowed_updates: Deserialize::default(), enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(allowed_updates), Some(enabled)) =
                (self.allowed_updates.take(), self.enabled)
            else {
                return None;
            };
            Some(Self::Out { allowed_updates, enabled })
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

    impl ObjectDeser for PortalCustomerUpdate {
        type Builder = PortalCustomerUpdateBuilder;
    }

    impl FromValueOpt for PortalCustomerUpdate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalCustomerUpdateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allowed_updates" => b.allowed_updates = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The types of customer updates that are supported. When empty, customers are not updateable.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalCustomerUpdateAllowedUpdates {
    pub fn as_str(&self) -> &str {
        use PortalCustomerUpdateAllowedUpdates::*;
        match self {
            Address => "address",
            Email => "email",
            Name => "name",
            Phone => "phone",
            Shipping => "shipping",
            TaxId => "tax_id",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalCustomerUpdateAllowedUpdates {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalCustomerUpdateAllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalCustomerUpdateAllowedUpdates"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalCustomerUpdateAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PortalCustomerUpdateAllowedUpdates {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PortalCustomerUpdateAllowedUpdates> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalCustomerUpdateAllowedUpdates::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PortalCustomerUpdateAllowedUpdates);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalCustomerUpdateAllowedUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
