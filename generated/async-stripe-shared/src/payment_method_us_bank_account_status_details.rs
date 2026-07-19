#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodUsBankAccountStatusDetails {
    pub blocked: Option<stripe_shared::PaymentMethodUsBankAccountBlocked>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodUsBankAccountStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodUsBankAccountStatusDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodUsBankAccountStatusDetailsBuilder {
    blocked: Option<Option<stripe_shared::PaymentMethodUsBankAccountBlocked>>,
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

    impl Deserialize for PaymentMethodUsBankAccountStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodUsBankAccountStatusDetails>,
        builder: PaymentMethodUsBankAccountStatusDetailsBuilder,
    }

    impl Visitor for Place<PaymentMethodUsBankAccountStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodUsBankAccountStatusDetailsBuilder {
                    blocked: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "blocked" => Deserialize::begin(&mut self.builder.blocked),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(blocked),) = (self.builder.blocked.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodUsBankAccountStatusDetails { blocked });
            Ok(())
        }
    }
};
