#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsPayto {
    pub mandate_options: Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsPayto>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsPayto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsPaytoBuilder {
    mandate_options:
        Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsPayto>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsPayto>,
        builder: SetupIntentPaymentMethodOptionsPaytoBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsPaytoBuilder {
                    mandate_options: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mandate_options" => Deserialize::begin(&mut self.builder.mandate_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(mandate_options),) = (self.builder.mandate_options.take(),) else {
                return Ok(());
            };
            *self.out = Some(SetupIntentPaymentMethodOptionsPayto { mandate_options });
            Ok(())
        }
    }
};
