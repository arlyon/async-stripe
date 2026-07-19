/// Data used for generating a Mandate.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourceMandateData {
    pub customer_acceptance:
        stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptance,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokensResourceMandateData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmationTokensResourceMandateData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConfirmationTokensResourceMandateDataBuilder {
    customer_acceptance:
        Option<stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptance>,
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

    impl Deserialize for ConfirmationTokensResourceMandateData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokensResourceMandateData>,
        builder: ConfirmationTokensResourceMandateDataBuilder,
    }

    impl Visitor for Place<ConfirmationTokensResourceMandateData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokensResourceMandateDataBuilder {
                    customer_acceptance: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_acceptance" => Deserialize::begin(&mut self.builder.customer_acceptance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(customer_acceptance),) = (self.builder.customer_acceptance.take(),) else {
                return Ok(());
            };
            *self.out = Some(ConfirmationTokensResourceMandateData { customer_acceptance });
            Ok(())
        }
    }
};
