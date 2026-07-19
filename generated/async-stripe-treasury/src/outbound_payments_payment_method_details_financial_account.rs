#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    /// Token of the FinancialAccount.
    pub id: String,
    /// The rails used to send funds.
    pub network: OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OutboundPaymentsPaymentMethodDetailsFinancialAccount")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder {
    id: Option<String>,
    network: Option<OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork>,
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

    impl Deserialize for OutboundPaymentsPaymentMethodDetailsFinancialAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
        builder: OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder,
    }

    impl Visitor for Place<OutboundPaymentsPaymentMethodDetailsFinancialAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder {
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
            *self.out = Some(OutboundPaymentsPaymentMethodDetailsFinancialAccount { id, network });
            Ok(())
        }
    }
};
/// The rails used to send funds.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(&self) -> &str {
        use OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::*;
        match self {
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::*;
        match s {
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
