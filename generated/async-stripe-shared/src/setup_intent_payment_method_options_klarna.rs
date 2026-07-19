#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsKlarna {
    /// The currency of the setup intent. Three letter ISO currency code.
    pub currency: Option<stripe_types::Currency>,
    /// Preferred locale of the Klarna checkout page that the customer is redirected to.
    pub preferred_locale: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsKlarna").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsKlarnaBuilder {
    currency: Option<Option<stripe_types::Currency>>,
    preferred_locale: Option<Option<String>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsKlarna>,
        builder: SetupIntentPaymentMethodOptionsKlarnaBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsKlarnaBuilder {
                    currency: Deserialize::default(),
                    preferred_locale: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "preferred_locale" => Deserialize::begin(&mut self.builder.preferred_locale),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(currency), Some(preferred_locale)) =
                (self.builder.currency.take(), self.builder.preferred_locale.take())
            else {
                return Ok(());
            };
            *self.out = Some(SetupIntentPaymentMethodOptionsKlarna { currency, preferred_locale });
            Ok(())
        }
    }
};
