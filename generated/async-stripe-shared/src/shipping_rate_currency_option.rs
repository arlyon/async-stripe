#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ShippingRateCurrencyOption {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: stripe_shared::PriceTaxBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRateCurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingRateCurrencyOption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ShippingRateCurrencyOptionBuilder {
    amount: Option<i64>,
    tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
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

    impl Deserialize for ShippingRateCurrencyOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRateCurrencyOption>,
        builder: ShippingRateCurrencyOptionBuilder,
    }

    impl Visitor for Place<ShippingRateCurrencyOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ShippingRateCurrencyOptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ShippingRateCurrencyOptionBuilder {
        type Out = ShippingRateCurrencyOption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount: None, tax_behavior: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(tax_behavior)) = (self.amount, self.tax_behavior.take()) else {
                return None;
            };
            Some(Self::Out { amount, tax_behavior })
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

    impl ObjectDeser for ShippingRateCurrencyOption {
        type Builder = ShippingRateCurrencyOptionBuilder;
    }

    impl FromValueOpt for ShippingRateCurrencyOption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ShippingRateCurrencyOptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "tax_behavior" => b.tax_behavior = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
pub type ShippingRateCurrencyOptionTaxBehavior = stripe_shared::PriceTaxBehavior;
