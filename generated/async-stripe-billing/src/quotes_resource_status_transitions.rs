#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceStatusTransitions").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: QuotesResourceStatusTransitionsBuilder {
                    accepted_at: Deserialize::default(),
                    canceled_at: Deserialize::default(),
                    finalized_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accepted_at" => Deserialize::begin(&mut self.builder.accepted_at),
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "finalized_at" => Deserialize::begin(&mut self.builder.finalized_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(accepted_at), Some(canceled_at), Some(finalized_at)) =
                (self.builder.accepted_at, self.builder.canceled_at, self.builder.finalized_at)
            else {
                return Ok(());
            };
            *self.out =
                Some(QuotesResourceStatusTransitions { accepted_at, canceled_at, finalized_at });
            Ok(())
        }
    }
};
