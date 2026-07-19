#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundTransfersResourceStatusTransitions {
    /// Timestamp describing when an OutboundTransfer changed status to `canceled`
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundTransfer changed status to `failed`
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundTransfer changed status to `posted`
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundTransfer changed status to `returned`
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundTransfersResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundTransfersResourceStatusTransitions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryOutboundTransfersResourceStatusTransitionsBuilder {
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    failed_at: Option<Option<stripe_types::Timestamp>>,
    posted_at: Option<Option<stripe_types::Timestamp>>,
    returned_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for TreasuryOutboundTransfersResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundTransfersResourceStatusTransitions>,
        builder: TreasuryOutboundTransfersResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundTransfersResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundTransfersResourceStatusTransitionsBuilder {
                    canceled_at: Deserialize::default(),
                    failed_at: Deserialize::default(),
                    posted_at: Deserialize::default(),
                    returned_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "failed_at" => Deserialize::begin(&mut self.builder.failed_at),
                "posted_at" => Deserialize::begin(&mut self.builder.posted_at),
                "returned_at" => Deserialize::begin(&mut self.builder.returned_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(canceled_at), Some(failed_at), Some(posted_at), Some(returned_at)) = (
                self.builder.canceled_at,
                self.builder.failed_at,
                self.builder.posted_at,
                self.builder.returned_at,
            ) else {
                return Ok(());
            };
            *self.out = Some(TreasuryOutboundTransfersResourceStatusTransitions {
                canceled_at,
                failed_at,
                posted_at,
                returned_at,
            });
            Ok(())
        }
    }
};
