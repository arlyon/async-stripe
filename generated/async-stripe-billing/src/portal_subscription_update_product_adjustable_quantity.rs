#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionUpdateProductAdjustableQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionUpdateProductAdjustableQuantity").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PortalSubscriptionUpdateProductAdjustableQuantityBuilder {
                    enabled: Deserialize::default(),
                    maximum: Deserialize::default(),
                    minimum: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "maximum" => Deserialize::begin(&mut self.builder.maximum),
                "minimum" => Deserialize::begin(&mut self.builder.minimum),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled), Some(maximum), Some(minimum)) =
                (self.builder.enabled, self.builder.maximum, self.builder.minimum)
            else {
                return Ok(());
            };
            *self.out = Some(PortalSubscriptionUpdateProductAdjustableQuantity {
                enabled,
                maximum,
                minimum,
            });
            Ok(())
        }
    }
};
