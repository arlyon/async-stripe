#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceBillingMeterValue {
    /// The key in the meter event payload to use as the value for this meter.
    pub event_payload_key: String,
}
#[doc(hidden)]
pub struct BillingMeterResourceBillingMeterValueBuilder {
    event_payload_key: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: BillingMeterResourceBillingMeterValueBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterResourceBillingMeterValueBuilder {
        type Out = BillingMeterResourceBillingMeterValue;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "event_payload_key" => Deserialize::begin(&mut self.event_payload_key),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { event_payload_key: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(event_payload_key),) = (self.event_payload_key.take(),) else {
                return None;
            };
            Some(Self::Out { event_payload_key })
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

    impl ObjectDeser for BillingMeterResourceBillingMeterValue {
        type Builder = BillingMeterResourceBillingMeterValueBuilder;
    }

    impl FromValueOpt for BillingMeterResourceBillingMeterValue {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterResourceBillingMeterValueBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "event_payload_key" => b.event_payload_key = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
