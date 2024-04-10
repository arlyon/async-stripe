#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SepaDebitGeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>,
}
#[doc(hidden)]
pub struct SepaDebitGeneratedFromBuilder {
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    setup_attempt: Option<Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SepaDebitGeneratedFromBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SepaDebitGeneratedFromBuilder {
        type Out = SepaDebitGeneratedFrom;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.charge),
                "setup_attempt" => Deserialize::begin(&mut self.setup_attempt),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { charge: Deserialize::default(), setup_attempt: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                charge: self.charge.take()?,
                setup_attempt: self.setup_attempt.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SepaDebitGeneratedFrom {
        type Builder = SepaDebitGeneratedFromBuilder;
    }

    impl FromValueOpt for SepaDebitGeneratedFrom {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SepaDebitGeneratedFromBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "charge" => b.charge = Some(FromValueOpt::from_value(v)?),
                    "setup_attempt" => b.setup_attempt = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
