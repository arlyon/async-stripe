#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    /// Whether this payment method may be offered at checkout.
    /// True if `display_preference` is `on` and the payment method's capability is active.
    pub available: bool,
    pub display_preference: stripe_payment::PaymentMethodConfigResourceDisplayPreference,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodConfigResourcePaymentMethodProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodConfigResourcePaymentMethodProperties").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodConfigResourcePaymentMethodPropertiesBuilder {
    available: Option<bool>,
    display_preference: Option<stripe_payment::PaymentMethodConfigResourceDisplayPreference>,
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

    impl Deserialize for PaymentMethodConfigResourcePaymentMethodProperties {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfigResourcePaymentMethodProperties>,
        builder: PaymentMethodConfigResourcePaymentMethodPropertiesBuilder,
    }

    impl Visitor for Place<PaymentMethodConfigResourcePaymentMethodProperties> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodConfigResourcePaymentMethodPropertiesBuilder {
                    available: Deserialize::default(),
                    display_preference: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.builder.available),
                "display_preference" => Deserialize::begin(&mut self.builder.display_preference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available), Some(display_preference)) =
                (self.builder.available, self.builder.display_preference.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodConfigResourcePaymentMethodProperties {
                available,
                display_preference,
            });
            Ok(())
        }
    }
};
