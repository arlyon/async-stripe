#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    /// Timestamp describing when the CreditReversal changed status to `posted`
    pub posted_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryReceivedCreditsResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryReceivedCreditsResourceStatusTransitions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryReceivedCreditsResourceStatusTransitionsBuilder {
    posted_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for TreasuryReceivedCreditsResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceStatusTransitions>,
        builder: TreasuryReceivedCreditsResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedCreditsResourceStatusTransitionsBuilder {
                    posted_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "posted_at" => Deserialize::begin(&mut self.builder.posted_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(posted_at),) = (self.builder.posted_at,) else {
                return Ok(());
            };
            *self.out = Some(TreasuryReceivedCreditsResourceStatusTransitions { posted_at });
            Ok(())
        }
    }
};
