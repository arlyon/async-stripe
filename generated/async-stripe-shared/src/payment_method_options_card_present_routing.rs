#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardPresentRouting {
    /// Requested routing priority
    pub requested_priority: Option<stripe_shared::RequestedPriority>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsCardPresentRouting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsCardPresentRouting").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCardPresentRoutingBuilder {
    requested_priority: Option<Option<stripe_shared::RequestedPriority>>,
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
            Self { requested_priority: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(requested_priority),) = (self.requested_priority.take(),) else {
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
pub type PaymentMethodOptionsCardPresentRoutingRequestedPriority = stripe_shared::RequestedPriority;
