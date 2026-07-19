#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct UsBankAccountNetworks {
    /// The preferred network.
    pub preferred: Option<String>,
    /// All supported networks.
    pub supported: Vec<UsBankAccountNetworksSupported>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UsBankAccountNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UsBankAccountNetworks").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct UsBankAccountNetworksBuilder {
    preferred: Option<Option<String>>,
    supported: Option<Vec<UsBankAccountNetworksSupported>>,
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
                builder: UsBankAccountNetworksBuilder {
                    preferred: Deserialize::default(),
                    supported: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "preferred" => Deserialize::begin(&mut self.builder.preferred),
                "supported" => Deserialize::begin(&mut self.builder.supported),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(preferred), Some(supported)) =
                (self.builder.preferred.take(), self.builder.supported.take())
            else {
                return Ok(());
            };
            *self.out = Some(UsBankAccountNetworks { preferred, supported });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UsBankAccountNetworksSupported)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for UsBankAccountNetworksSupported {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<UsBankAccountNetworksSupported> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(UsBankAccountNetworksSupported::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UsBankAccountNetworksSupported {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
