#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionUpdateProductAdjustableQuantity {
    /// If true, the quantity can be adjusted to any non-negative integer.
    pub enabled: bool,
    /// The maximum quantity that can be set for the product.
    pub maximum: Option<i64>,
    /// The minimum quantity that can be set for the product.
    pub minimum: i64,
}
#[doc(hidden)]
pub struct PortalSubscriptionUpdateProductAdjustableQuantityBuilder {
    enabled: Option<bool>,
    maximum: Option<Option<i64>>,
    minimum: Option<i64>,
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

    impl Deserialize for PortalSubscriptionUpdateProductAdjustableQuantity {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionUpdateProductAdjustableQuantity>,
        builder: PortalSubscriptionUpdateProductAdjustableQuantityBuilder,
    }

    impl Visitor for Place<PortalSubscriptionUpdateProductAdjustableQuantity> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionUpdateProductAdjustableQuantityBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionUpdateProductAdjustableQuantityBuilder {
        type Out = PortalSubscriptionUpdateProductAdjustableQuantity;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "maximum" => Deserialize::begin(&mut self.maximum),
                "minimum" => Deserialize::begin(&mut self.minimum),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                enabled: Deserialize::default(),
                maximum: Deserialize::default(),
                minimum: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled), Some(maximum), Some(minimum)) =
                (self.enabled, self.maximum, self.minimum)
            else {
                return None;
            };
            Some(Self::Out { enabled, maximum, minimum })
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

    impl ObjectDeser for PortalSubscriptionUpdateProductAdjustableQuantity {
        type Builder = PortalSubscriptionUpdateProductAdjustableQuantityBuilder;
    }

    impl FromValueOpt for PortalSubscriptionUpdateProductAdjustableQuantity {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionUpdateProductAdjustableQuantityBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "maximum" => b.maximum = FromValueOpt::from_value(v),
                    "minimum" => b.minimum = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
