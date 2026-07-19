#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutPaymentMethodOptionsMandateOptionsSepaDebit")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder {
    reference_prefix: Option<Option<String>>,
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

    impl Deserialize for CheckoutPaymentMethodOptionsMandateOptionsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutPaymentMethodOptionsMandateOptionsSepaDebit>,
        builder: CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder,
    }

    impl Visitor for Place<CheckoutPaymentMethodOptionsMandateOptionsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutPaymentMethodOptionsMandateOptionsSepaDebitBuilder {
                    reference_prefix: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference_prefix" => Deserialize::begin(&mut self.builder.reference_prefix),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(reference_prefix),) = (self.builder.reference_prefix.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(CheckoutPaymentMethodOptionsMandateOptionsSepaDebit { reference_prefix });
            Ok(())
        }
    }
};
