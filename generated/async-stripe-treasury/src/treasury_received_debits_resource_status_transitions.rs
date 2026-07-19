#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    /// Timestamp describing when the DebitReversal changed status to `completed`.
    pub completed_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryReceivedDebitsResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryReceivedDebitsResourceStatusTransitions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryReceivedDebitsResourceStatusTransitionsBuilder {
    completed_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for TreasuryReceivedDebitsResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedDebitsResourceStatusTransitions>,
        builder: TreasuryReceivedDebitsResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedDebitsResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedDebitsResourceStatusTransitionsBuilder {
                    completed_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "completed_at" => Deserialize::begin(&mut self.builder.completed_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(completed_at),) = (self.builder.completed_at,) else {
                return Ok(());
            };
            *self.out = Some(TreasuryReceivedDebitsResourceStatusTransitions { completed_at });
            Ok(())
        }
    }
};
