#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    /// The FinancialAccount ID.
    pub id: String,
    /// The rails the ReceivedCredit was sent over. A FinancialAccount can only send funds over `stripe`.
    pub network: ReceivedPaymentMethodDetailsFinancialAccountNetwork,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReceivedPaymentMethodDetailsFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReceivedPaymentMethodDetailsFinancialAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReceivedPaymentMethodDetailsFinancialAccountBuilder {
    id: Option<String>,
    network: Option<ReceivedPaymentMethodDetailsFinancialAccountNetwork>,
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

    impl Deserialize for ReceivedPaymentMethodDetailsFinancialAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReceivedPaymentMethodDetailsFinancialAccount>,
        builder: ReceivedPaymentMethodDetailsFinancialAccountBuilder,
    }

    impl Visitor for Place<ReceivedPaymentMethodDetailsFinancialAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReceivedPaymentMethodDetailsFinancialAccountBuilder {
                    id: Deserialize::default(),
                    network: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                "network" => Deserialize::begin(&mut self.builder.network),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(id), Some(network)) = (self.builder.id.take(), self.builder.network.take())
            else {
                return Ok(());
            };
            *self.out = Some(ReceivedPaymentMethodDetailsFinancialAccount { id, network });
            Ok(())
        }
    }
};
/// The rails the ReceivedCredit was sent over. A FinancialAccount can only send funds over `stripe`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(&self) -> &str {
        use ReceivedPaymentMethodDetailsFinancialAccountNetwork::*;
        match self {
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReceivedPaymentMethodDetailsFinancialAccountNetwork::*;
        match s {
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ReceivedPaymentMethodDetailsFinancialAccountNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReceivedPaymentMethodDetailsFinancialAccountNetwork))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ReceivedPaymentMethodDetailsFinancialAccountNetwork> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ReceivedPaymentMethodDetailsFinancialAccountNetwork::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
