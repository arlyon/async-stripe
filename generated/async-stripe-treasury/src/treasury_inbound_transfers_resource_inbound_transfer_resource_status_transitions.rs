#[derive(Copy, Clone, Debug)]
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
#[doc(hidden)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder {
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    failed_at: Option<Option<stripe_types::Timestamp>>,
    succeeded_at: Option<Option<stripe_types::Timestamp>>,
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
            builder: TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder
    {
        type Out = TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "failed_at" => Deserialize::begin(&mut self.failed_at),
                "succeeded_at" => Deserialize::begin(&mut self.succeeded_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                canceled_at: Deserialize::default(),
                failed_at: Deserialize::default(),
                succeeded_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(canceled_at), Some(failed_at), Some(succeeded_at)) =
                (self.canceled_at, self.failed_at, self.succeeded_at)
            else {
                return None;
            };
            Some(Self::Out { canceled_at, failed_at, succeeded_at })
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

    impl ObjectDeser for TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
        type Builder =
            TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder;
    }

    impl FromValueOpt for TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "canceled_at" => b.canceled_at = FromValueOpt::from_value(v),
                    "failed_at" => b.failed_at = FromValueOpt::from_value(v),
                    "succeeded_at" => b.succeeded_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
