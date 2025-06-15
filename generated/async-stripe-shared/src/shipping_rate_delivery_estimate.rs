#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    pub minimum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
}
#[doc(hidden)]
pub struct ShippingRateDeliveryEstimateBuilder {
    maximum: Option<Option<stripe_shared::ShippingRateDeliveryEstimateBound>>,
    minimum: Option<Option<stripe_shared::ShippingRateDeliveryEstimateBound>>,
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

    impl Deserialize for ShippingRateDeliveryEstimate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRateDeliveryEstimate>,
        builder: ShippingRateDeliveryEstimateBuilder,
    }

    impl Visitor for Place<ShippingRateDeliveryEstimate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ShippingRateDeliveryEstimateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ShippingRateDeliveryEstimateBuilder {
        type Out = ShippingRateDeliveryEstimate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "maximum" => Deserialize::begin(&mut self.maximum),
                "minimum" => Deserialize::begin(&mut self.minimum),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { maximum: Deserialize::default(), minimum: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(maximum), Some(minimum)) = (self.maximum, self.minimum) else {
                return None;
            };
            Some(Self::Out { maximum, minimum })
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

    impl ObjectDeser for ShippingRateDeliveryEstimate {
        type Builder = ShippingRateDeliveryEstimateBuilder;
    }

    impl FromValueOpt for ShippingRateDeliveryEstimate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ShippingRateDeliveryEstimateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "maximum" => b.maximum = FromValueOpt::from_value(v),
                    "minimum" => b.minimum = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
