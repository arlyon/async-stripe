#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    /// Token of the FinancialAccount.
    pub id: String,
    /// The rails used to send funds.
    pub network: OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}
#[doc(hidden)]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder {
    id: Option<String>,
    network: Option<OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork>,
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
                builder: OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder {
        type Out = OutboundPaymentsPaymentMethodDetailsFinancialAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "network" => Deserialize::begin(&mut self.network),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), network: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(id), Some(network)) = (self.id.take(), self.network) else {
                return None;
            };
            Some(Self::Out { id, network })
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

    impl ObjectDeser for OutboundPaymentsPaymentMethodDetailsFinancialAccount {
        type Builder = OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder;
    }

    impl FromValueOpt for OutboundPaymentsPaymentMethodDetailsFinancialAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                OutboundPaymentsPaymentMethodDetailsFinancialAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The rails used to send funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}
impl OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::*;
        match self {
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::*;
        match s {
            "stripe" => Ok(Stripe),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork",
            )
        })
    }
}
