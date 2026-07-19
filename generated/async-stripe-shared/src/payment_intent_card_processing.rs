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
    customer_notification:
        Option<Option<stripe_shared::PaymentIntentProcessingCustomerNotification>>,
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
                builder: PaymentIntentCardProcessingBuilder {
                    customer_notification: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_notification" => {
                    Deserialize::begin(&mut self.builder.customer_notification)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(customer_notification),) = (self.builder.customer_notification,) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentCardProcessing { customer_notification });
            Ok(())
        }
    }
};
