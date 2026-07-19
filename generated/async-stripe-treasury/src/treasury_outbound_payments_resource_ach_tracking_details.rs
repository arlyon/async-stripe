#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceAchTrackingDetails {
    /// ACH trace ID of the OutboundPayment for payments sent over the `ach` network.
    pub trace_id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundPaymentsResourceAchTrackingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundPaymentsResourceAchTrackingDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder {
    trace_id: Option<String>,
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
                builder: TreasuryOutboundPaymentsResourceAchTrackingDetailsBuilder {
                    trace_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "trace_id" => Deserialize::begin(&mut self.builder.trace_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(trace_id),) = (self.builder.trace_id.take(),) else {
                return Ok(());
            };
            *self.out = Some(TreasuryOutboundPaymentsResourceAchTrackingDetails { trace_id });
            Ok(())
        }
    }
};
