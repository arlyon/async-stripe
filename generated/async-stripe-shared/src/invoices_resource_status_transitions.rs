#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceStatusTransitions {
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
pub struct InvoicesResourceStatusTransitionsBuilder {
    finalized_at: Option<Option<stripe_types::Timestamp>>,
    marked_uncollectible_at: Option<Option<stripe_types::Timestamp>>,
    paid_at: Option<Option<stripe_types::Timestamp>>,
    voided_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceStatusTransitions>,
        builder: InvoicesResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<InvoicesResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesResourceStatusTransitionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesResourceStatusTransitionsBuilder {
        type Out = InvoicesResourceStatusTransitions;
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
            let (Some(finalized_at), Some(marked_uncollectible_at), Some(paid_at), Some(voided_at)) =
                (self.finalized_at, self.marked_uncollectible_at, self.paid_at, self.voided_at)
            else {
                return None;
            };
            Some(Self::Out { finalized_at, marked_uncollectible_at, paid_at, voided_at })
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

    impl ObjectDeser for InvoicesResourceStatusTransitions {
        type Builder = InvoicesResourceStatusTransitionsBuilder;
    }

    impl FromValueOpt for InvoicesResourceStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesResourceStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "finalized_at" => b.finalized_at = FromValueOpt::from_value(v),
                    "marked_uncollectible_at" => {
                        b.marked_uncollectible_at = FromValueOpt::from_value(v)
                    }
                    "paid_at" => b.paid_at = FromValueOpt::from_value(v),
                    "voided_at" => b.voided_at = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
