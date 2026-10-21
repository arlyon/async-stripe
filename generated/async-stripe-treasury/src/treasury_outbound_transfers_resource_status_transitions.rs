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
                builder: TreasuryOutboundTransfersResourceStatusTransitionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryOutboundTransfersResourceStatusTransitionsBuilder {
        type Out = TreasuryOutboundTransfersResourceStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "failed_at" => Deserialize::begin(&mut self.failed_at),
                "posted_at" => Deserialize::begin(&mut self.posted_at),
                "returned_at" => Deserialize::begin(&mut self.returned_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                canceled_at: Some(None),
                failed_at: Some(None),
                posted_at: Some(None),
                returned_at: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    canceled_at: Some(canceled_at),
                    failed_at: Some(failed_at),
                    posted_at: Some(posted_at),
                    returned_at: Some(returned_at),
                } => Some(Self::Out { canceled_at, failed_at, posted_at, returned_at }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryOutboundTransfersResourceStatusTransitions {
        type Builder = TreasuryOutboundTransfersResourceStatusTransitionsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundTransfersResourceStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
