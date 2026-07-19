#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceOptionalItemAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    pub enabled: bool,
    /// The maximum quantity of this item the customer can purchase. By default this value is 99.
    pub maximum: Option<i64>,
    /// The minimum quantity of this item the customer must purchase, if they choose to purchase it.
    /// Because this item is optional, the customer will always be able to remove it from their order, even if the `minimum` configured here is greater than 0.
    /// By default this value is 0.
    pub minimum: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceOptionalItemAdjustableQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceOptionalItemAdjustableQuantity").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceOptionalItemAdjustableQuantityBuilder {
    enabled: Option<bool>,
    maximum: Option<Option<i64>>,
    minimum: Option<Option<i64>>,
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

    impl Deserialize for PaymentLinksResourceOptionalItemAdjustableQuantity {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceOptionalItemAdjustableQuantity>,
        builder: PaymentLinksResourceOptionalItemAdjustableQuantityBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceOptionalItemAdjustableQuantity> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceOptionalItemAdjustableQuantityBuilder {
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
            *self.out = Some(PaymentLinksResourceOptionalItemAdjustableQuantity {
                enabled,
                maximum,
                minimum,
            });
            Ok(())
        }
    }
};
