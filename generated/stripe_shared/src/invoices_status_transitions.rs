#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesStatusTransitions {
    /// The time that the invoice draft was finalized.
    pub finalized_at: Option<stripe_types::Timestamp>,
    /// The time that the invoice was marked uncollectible.
    pub marked_uncollectible_at: Option<stripe_types::Timestamp>,
    /// The time that the invoice was paid.
    pub paid_at: Option<stripe_types::Timestamp>,
    /// The time that the invoice was voided.
    pub voided_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct InvoicesStatusTransitionsBuilder {
    finalized_at: Option<Option<stripe_types::Timestamp>>,
    marked_uncollectible_at: Option<Option<stripe_types::Timestamp>>,
    paid_at: Option<Option<stripe_types::Timestamp>>,
    voided_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesStatusTransitions>,
        builder: InvoicesStatusTransitionsBuilder,
    }

    impl Visitor for Place<InvoicesStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesStatusTransitionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesStatusTransitionsBuilder {
        type Out = InvoicesStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "finalized_at" => Deserialize::begin(&mut self.finalized_at),
                "marked_uncollectible_at" => Deserialize::begin(&mut self.marked_uncollectible_at),
                "paid_at" => Deserialize::begin(&mut self.paid_at),
                "voided_at" => Deserialize::begin(&mut self.voided_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                finalized_at: Deserialize::default(),
                marked_uncollectible_at: Deserialize::default(),
                paid_at: Deserialize::default(),
                voided_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                finalized_at: self.finalized_at?,
                marked_uncollectible_at: self.marked_uncollectible_at?,
                paid_at: self.paid_at?,
                voided_at: self.voided_at?,
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

    impl ObjectDeser for InvoicesStatusTransitions {
        type Builder = InvoicesStatusTransitionsBuilder;
    }

    impl FromValueOpt for InvoicesStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "finalized_at" => b.finalized_at = Some(FromValueOpt::from_value(v)?),
                    "marked_uncollectible_at" => {
                        b.marked_uncollectible_at = Some(FromValueOpt::from_value(v)?)
                    }
                    "paid_at" => b.paid_at = Some(FromValueOpt::from_value(v)?),
                    "voided_at" => b.voided_at = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};