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
    maximum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
    minimum: Option<stripe_shared::ShippingRateDeliveryEstimateBound>,
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
                "maximum" => stripe_types::miniserde_helpers::begin_nullable(&mut self.maximum),
                "minimum" => stripe_types::miniserde_helpers::begin_nullable(&mut self.minimum),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { maximum: None, minimum: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { maximum, minimum } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { maximum, minimum })
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

    impl ObjectDeser for ShippingRateDeliveryEstimate {
        type Builder = ShippingRateDeliveryEstimateBuilder;
    }

    impl FromValueOpt for ShippingRateDeliveryEstimate {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
