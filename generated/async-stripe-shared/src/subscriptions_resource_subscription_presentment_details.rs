#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceSubscriptionPresentmentDetails {
    /// Currency used for customer payments.
    pub presentment_currency: stripe_types::Currency,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceSubscriptionPresentmentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceSubscriptionPresentmentDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceSubscriptionPresentmentDetailsBuilder {
    presentment_currency: Option<stripe_types::Currency>,
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

    impl Deserialize for SubscriptionsResourceSubscriptionPresentmentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceSubscriptionPresentmentDetails>,
        builder: SubscriptionsResourceSubscriptionPresentmentDetailsBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceSubscriptionPresentmentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceSubscriptionPresentmentDetailsBuilder {
                    presentment_currency: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "presentment_currency" => {
                    Deserialize::begin(&mut self.builder.presentment_currency)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(presentment_currency),) = (self.builder.presentment_currency.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(SubscriptionsResourceSubscriptionPresentmentDetails { presentment_currency });
            Ok(())
        }
    }
};
