/// Client device metadata attached to this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationClientDeviceMetadata {
    /// ID for the Radar Session associated with the payment evaluation.
    /// A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    pub radar_session: String,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationClientDeviceMetadataBuilder {
    radar_session: Option<String>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationClientDeviceMetadata {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationClientDeviceMetadata>,
        builder: InsightsResourcesPaymentEvaluationClientDeviceMetadataBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationClientDeviceMetadata> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    InsightsResourcesPaymentEvaluationClientDeviceMetadataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationClientDeviceMetadataBuilder {
        type Out = InsightsResourcesPaymentEvaluationClientDeviceMetadata;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "radar_session" => Deserialize::begin(&mut self.radar_session),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { radar_session: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(radar_session),) = (self.radar_session.take(),) else {
                return None;
            };
            Some(Self::Out { radar_session })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationClientDeviceMetadata {
        type Builder = InsightsResourcesPaymentEvaluationClientDeviceMetadataBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationClientDeviceMetadata {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InsightsResourcesPaymentEvaluationClientDeviceMetadataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "radar_session" => b.radar_session = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
