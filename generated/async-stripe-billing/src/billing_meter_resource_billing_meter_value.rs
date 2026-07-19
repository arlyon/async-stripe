#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceBillingMeterValue {
    /// The key in the meter event payload to use as the value for this meter.
    pub event_payload_key: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterResourceBillingMeterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeterResourceBillingMeterValue").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingMeterResourceBillingMeterValueBuilder {
    event_payload_key: Option<String>,
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

    impl Deserialize for BillingMeterResourceBillingMeterValue {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterResourceBillingMeterValue>,
        builder: BillingMeterResourceBillingMeterValueBuilder,
    }

    impl Visitor for Place<BillingMeterResourceBillingMeterValue> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterResourceBillingMeterValueBuilder {
                    event_payload_key: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "event_payload_key" => Deserialize::begin(&mut self.builder.event_payload_key),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(event_payload_key),) = (self.builder.event_payload_key.take(),) else {
                return Ok(());
            };
            *self.out = Some(BillingMeterResourceBillingMeterValue { event_payload_key });
            Ok(())
        }
    }
};
