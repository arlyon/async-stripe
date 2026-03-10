#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundTransfersResourceAchTrackingDetails {
    /// ACH trace ID of the OutboundTransfer for transfers sent over the `ach` network.
    pub trace_id: String,
}
#[doc(hidden)]
pub struct TreasuryOutboundTransfersResourceAchTrackingDetailsBuilder {
    trace_id: Option<String>,
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

    impl Deserialize for TreasuryOutboundTransfersResourceAchTrackingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundTransfersResourceAchTrackingDetails>,
        builder: TreasuryOutboundTransfersResourceAchTrackingDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundTransfersResourceAchTrackingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundTransfersResourceAchTrackingDetailsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TreasuryOutboundTransfersResourceAchTrackingDetailsBuilder {
        type Out = TreasuryOutboundTransfersResourceAchTrackingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "trace_id" => Deserialize::begin(&mut self.trace_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { trace_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(trace_id),) = (self.trace_id.take(),) else {
                return None;
            };
            Some(Self::Out { trace_id })
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

    impl ObjectDeser for TreasuryOutboundTransfersResourceAchTrackingDetails {
        type Builder = TreasuryOutboundTransfersResourceAchTrackingDetailsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundTransfersResourceAchTrackingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundTransfersResourceAchTrackingDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "trace_id" => b.trace_id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
