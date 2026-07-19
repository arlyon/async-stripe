#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SepaDebitGeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SepaDebitGeneratedFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SepaDebitGeneratedFrom").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SepaDebitGeneratedFromBuilder {
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    setup_attempt: Option<Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>>,
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

    impl Deserialize for SepaDebitGeneratedFrom {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SepaDebitGeneratedFrom>,
        builder: SepaDebitGeneratedFromBuilder,
    }

    impl Visitor for Place<SepaDebitGeneratedFrom> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SepaDebitGeneratedFromBuilder {
                    charge: Deserialize::default(),
                    setup_attempt: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.builder.charge),
                "setup_attempt" => Deserialize::begin(&mut self.builder.setup_attempt),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(charge), Some(setup_attempt)) =
                (self.builder.charge.take(), self.builder.setup_attempt.take())
            else {
                return Ok(());
            };
            *self.out = Some(SepaDebitGeneratedFrom { charge, setup_attempt });
            Ok(())
        }
    }
};
