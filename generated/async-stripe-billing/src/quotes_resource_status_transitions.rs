#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceStatusTransitions {
    /// The time that the quote was accepted. Measured in seconds since Unix epoch.
    pub accepted_at: Option<stripe_types::Timestamp>,
    /// The time that the quote was canceled. Measured in seconds since Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// The time that the quote was finalized. Measured in seconds since Unix epoch.
    pub finalized_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct QuotesResourceStatusTransitionsBuilder {
    accepted_at: Option<Option<stripe_types::Timestamp>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    finalized_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceStatusTransitions>,
        builder: QuotesResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<QuotesResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceStatusTransitionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceStatusTransitionsBuilder {
        type Out = QuotesResourceStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accepted_at" => Deserialize::begin(&mut self.accepted_at),
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "finalized_at" => Deserialize::begin(&mut self.finalized_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                accepted_at: Deserialize::default(),
                canceled_at: Deserialize::default(),
                finalized_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(accepted_at), Some(canceled_at), Some(finalized_at)) =
                (self.accepted_at, self.canceled_at, self.finalized_at)
            else {
                return None;
            };
            Some(Self::Out { accepted_at, canceled_at, finalized_at })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for QuotesResourceStatusTransitions {
        type Builder = QuotesResourceStatusTransitionsBuilder;
    }

    impl FromValueOpt for QuotesResourceStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accepted_at" => b.accepted_at = FromValueOpt::from_value(v),
                    "canceled_at" => b.canceled_at = FromValueOpt::from_value(v),
                    "finalized_at" => b.finalized_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
