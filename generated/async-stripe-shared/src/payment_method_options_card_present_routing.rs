#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardPresentRouting {
    /// Requested routing priority
    pub requested_priority: Option<PaymentMethodOptionsCardPresentRoutingRequestedPriority>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCardPresentRoutingBuilder {
    requested_priority: Option<Option<PaymentMethodOptionsCardPresentRoutingRequestedPriority>>,
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

    impl Deserialize for PaymentMethodOptionsCardPresentRouting {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCardPresentRouting>,
        builder: PaymentMethodOptionsCardPresentRoutingBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCardPresentRouting> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCardPresentRoutingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCardPresentRoutingBuilder {
        type Out = PaymentMethodOptionsCardPresentRouting;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "requested_priority" => Deserialize::begin(&mut self.requested_priority),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { requested_priority: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(requested_priority),) = (self.requested_priority,) else {
                return None;
            };
            Some(Self::Out { requested_priority })
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

    impl ObjectDeser for PaymentMethodOptionsCardPresentRouting {
        type Builder = PaymentMethodOptionsCardPresentRoutingBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCardPresentRouting {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsCardPresentRoutingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "requested_priority" => b.requested_priority = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Requested routing priority
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    Domestic,
    International,
}
impl PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCardPresentRoutingRequestedPriority::*;
        match self {
            Domestic => "domestic",
            International => "international",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardPresentRoutingRequestedPriority::*;
        match s {
            "domestic" => Ok(Domestic),
            "international" => Ok(International),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentMethodOptionsCardPresentRoutingRequestedPriority>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsCardPresentRoutingRequestedPriority::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsCardPresentRoutingRequestedPriority);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsCardPresentRoutingRequestedPriority",
            )
        })
    }
}
