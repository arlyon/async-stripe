#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct UsBankAccountNetworks {
    /// The preferred network.
    pub preferred: Option<String>,
    /// All supported networks.
    pub supported: Vec<UsBankAccountNetworksSupported>,
}
#[doc(hidden)]
pub struct UsBankAccountNetworksBuilder {
    preferred: Option<Option<String>>,
    supported: Option<Vec<UsBankAccountNetworksSupported>>,
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

    impl Deserialize for UsBankAccountNetworks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<UsBankAccountNetworks>,
        builder: UsBankAccountNetworksBuilder,
    }

    impl Visitor for Place<UsBankAccountNetworks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: UsBankAccountNetworksBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for UsBankAccountNetworksBuilder {
        type Out = UsBankAccountNetworks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "preferred" => Deserialize::begin(&mut self.preferred),
                "supported" => Deserialize::begin(&mut self.supported),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { preferred: Deserialize::default(), supported: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(preferred), Some(supported)) = (self.preferred.take(), self.supported.take())
            else {
                return None;
            };
            Some(Self::Out { preferred, supported })
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

    impl ObjectDeser for UsBankAccountNetworks {
        type Builder = UsBankAccountNetworksBuilder;
    }

    impl FromValueOpt for UsBankAccountNetworks {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = UsBankAccountNetworksBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "preferred" => b.preferred = FromValueOpt::from_value(v),
                    "supported" => b.supported = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// All supported networks.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UsBankAccountNetworksSupported {
    Ach,
    UsDomesticWire,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UsBankAccountNetworksSupported {
    pub fn as_str(&self) -> &str {
        use UsBankAccountNetworksSupported::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UsBankAccountNetworksSupported {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UsBankAccountNetworksSupported::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UsBankAccountNetworksSupported"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for UsBankAccountNetworksSupported {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for UsBankAccountNetworksSupported {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<UsBankAccountNetworksSupported> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(UsBankAccountNetworksSupported::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(UsBankAccountNetworksSupported);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UsBankAccountNetworksSupported {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
