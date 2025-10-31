#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    /// If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to the ReceivedDebit return.
    pub received_debit: Option<String>,
}
#[doc(hidden)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder {
    received_debit: Option<Option<String>>,
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
            builder: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder {
        type Out = TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "received_debit" => Deserialize::begin(&mut self.received_debit),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { received_debit: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(received_debit),) = (self.received_debit.take(),) else {
                return None;
            };
            Some(Self::Out { received_debit })
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

    impl ObjectDeser for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
        type Builder = TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder;
    }

    impl FromValueOpt for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlowsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "received_debit" => b.received_debit = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
