#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    pub enabled: bool,
    /// The maximum quantity of this item the customer can purchase.
    /// By default this value is 99.
    /// You can specify a value up to 999999.
    pub maximum: Option<i64>,
    /// The minimum quantity of this item the customer must purchase, if they choose to purchase it.
    /// Because this item is optional, the customer will always be able to remove it from their order, even if the `minimum` configured here is greater than 0.
    /// By default this value is 0.
    pub minimum: Option<i64>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionOptionalItemAdjustableQuantityBuilder {
    enabled: Option<bool>,
    maximum: Option<Option<i64>>,
    minimum: Option<Option<i64>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity>,
        builder: PaymentPagesCheckoutSessionOptionalItemAdjustableQuantityBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentPagesCheckoutSessionOptionalItemAdjustableQuantityBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionOptionalItemAdjustableQuantityBuilder {
        type Out = PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity;
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

    impl ObjectDeser for PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity {
        type Builder = PaymentPagesCheckoutSessionOptionalItemAdjustableQuantityBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentPagesCheckoutSessionOptionalItemAdjustableQuantityBuilder::deser_default();
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
