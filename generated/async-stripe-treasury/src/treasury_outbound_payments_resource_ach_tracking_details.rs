#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceAchTrackingDetails {
    /// ACH trace ID of the OutboundPayment for payments sent over the `ach` network.
    pub trace_id: String,
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder {
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

    impl Deserialize for TreasuryOutboundPaymentsResourceAchTrackingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundPaymentsResourceAchTrackingDetails>,
        builder: TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceAchTrackingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder {
        type Out = TreasuryOutboundPaymentsResourceAchTrackingDetails;
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

    impl ObjectDeser for TreasuryOutboundPaymentsResourceAchTrackingDetails {
        type Builder = TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundPaymentsResourceAchTrackingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder::deser_default();
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
