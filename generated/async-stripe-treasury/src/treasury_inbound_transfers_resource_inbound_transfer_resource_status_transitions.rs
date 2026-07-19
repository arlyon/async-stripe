#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    /// Timestamp describing when an InboundTransfer changed status to `canceled`.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an InboundTransfer changed status to `failed`.
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an InboundTransfer changed status to `succeeded`.
    pub succeeded_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder {
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    failed_at: Option<Option<stripe_types::Timestamp>>,
    succeeded_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
        >,
        builder: TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder { canceled_at: Deserialize::default(),
failed_at: Deserialize::default(),
succeeded_at: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "failed_at" => Deserialize::begin(&mut self.builder.failed_at),
                "succeeded_at" => Deserialize::begin(&mut self.builder.succeeded_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(canceled_at), Some(failed_at), Some(succeeded_at)) =
                (self.builder.canceled_at, self.builder.failed_at, self.builder.succeeded_at)
            else {
                return Ok(());
            };
            *self.out =
                Some(TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
                    canceled_at,
                    failed_at,
                    succeeded_at,
                });
            Ok(())
        }
    }
};
