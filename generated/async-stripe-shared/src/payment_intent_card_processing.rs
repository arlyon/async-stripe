#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentCardProcessing {
    pub customer_notification: Option<stripe_shared::PaymentIntentProcessingCustomerNotification>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentCardProcessing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentCardProcessing").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentCardProcessingBuilder {
    customer_notification: Option<stripe_shared::PaymentIntentProcessingCustomerNotification>,
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

    impl Deserialize for PaymentIntentCardProcessing {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentCardProcessing>,
        builder: PaymentIntentCardProcessingBuilder,
    }

    impl Visitor for Place<PaymentIntentCardProcessing> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentCardProcessingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentCardProcessingBuilder {
        type Out = PaymentIntentCardProcessing;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_notification" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.customer_notification)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { customer_notification: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { customer_notification } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { customer_notification })
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

    impl ObjectDeser for PaymentIntentCardProcessing {
        type Builder = PaymentIntentCardProcessingBuilder;
    }

    impl FromValueOpt for PaymentIntentCardProcessing {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
