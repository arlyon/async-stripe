#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    /// If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to the ReceivedDebit return.
    pub received_debit: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder {
    received_debit: Option<Option<String>>,
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

    impl Deserialize for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows>,
        builder: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder,
    }

    impl Visitor for Place<TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder {
                        received_debit: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "received_debit" => Deserialize::begin(&mut self.builder.received_debit),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(received_debit),) = (self.builder.received_debit.take(),) else {
                return Ok(());
            };
            *self.out = Some(TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
                received_debit,
            });
            Ok(())
        }
    }
};
