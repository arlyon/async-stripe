#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    pub minimum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRateDeliveryEstimate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingRateDeliveryEstimate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ShippingRateDeliveryEstimateBuilder {
    maximum: Option<Option<stripe_shared::ShippingRateDeliveryEstimateBound>>,
    minimum: Option<Option<stripe_shared::ShippingRateDeliveryEstimateBound>>,
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
                builder: ShippingRateDeliveryEstimateBuilder {
                    maximum: Deserialize::default(),
                    minimum: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "maximum" => Deserialize::begin(&mut self.builder.maximum),
                "minimum" => Deserialize::begin(&mut self.builder.minimum),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(maximum), Some(minimum)) =
                (self.builder.maximum.take(), self.builder.minimum.take())
            else {
                return Ok(());
            };
            *self.out = Some(ShippingRateDeliveryEstimate { maximum, minimum });
            Ok(())
        }
    }
};
