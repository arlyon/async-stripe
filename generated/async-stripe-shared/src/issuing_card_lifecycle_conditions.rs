#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardLifecycleConditions {
    /// The card is automatically cancelled when it makes this number of non-zero payment authorizations and transactions.
    /// The count includes penny authorizations, but doesn't include non-payment actions, such as authorization advice.
    pub payment_count: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardLifecycleConditions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardLifecycleConditions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardLifecycleConditionsBuilder {
    payment_count: Option<u64>,
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

    impl Deserialize for IssuingCardLifecycleConditions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardLifecycleConditions>,
        builder: IssuingCardLifecycleConditionsBuilder,
    }

    impl Visitor for Place<IssuingCardLifecycleConditions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardLifecycleConditionsBuilder {
                    payment_count: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_count" => Deserialize::begin(&mut self.builder.payment_count),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payment_count),) = (self.builder.payment_count,) else {
                return Ok(());
            };
            *self.out = Some(IssuingCardLifecycleConditions { payment_count });
            Ok(())
        }
    }
};
