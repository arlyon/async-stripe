#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TopupResourceUsBankAccount {
    /// The US bank transfer network used for this top-up. The default is `ach`.
    pub network: TopupResourceUsBankAccountNetwork,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TopupResourceUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TopupResourceUsBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TopupResourceUsBankAccountBuilder {
    network: Option<TopupResourceUsBankAccountNetwork>,
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

    impl Deserialize for TopupResourceUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TopupResourceUsBankAccount>,
        builder: TopupResourceUsBankAccountBuilder,
    }

    impl Visitor for Place<TopupResourceUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TopupResourceUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TopupResourceUsBankAccountBuilder {
        type Out = TopupResourceUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "network" => Deserialize::begin(&mut self.network),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { network: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(network),) = (self.network.take(),) else {
                return None;
            };
            Some(Self::Out { network })
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

    impl ObjectDeser for TopupResourceUsBankAccount {
        type Builder = TopupResourceUsBankAccountBuilder;
    }

    impl FromValueOpt for TopupResourceUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TopupResourceUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "network" => b.network = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The US bank transfer network used for this top-up. The default is `ach`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TopupResourceUsBankAccountNetwork {
    Ach,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TopupResourceUsBankAccountNetwork {
    pub fn as_str(&self) -> &str {
        use TopupResourceUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TopupResourceUsBankAccountNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TopupResourceUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TopupResourceUsBankAccountNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TopupResourceUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TopupResourceUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TopupResourceUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TopupResourceUsBankAccountNetwork)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TopupResourceUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TopupResourceUsBankAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TopupResourceUsBankAccountNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TopupResourceUsBankAccountNetwork::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TopupResourceUsBankAccountNetwork);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TopupResourceUsBankAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
