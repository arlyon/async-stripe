/// The amount of the tax rate when the `rate_type`` is `flat_amount`.
/// Tax rates with `rate_type` `percentage` can vary based on the transaction, resulting in this field being `null`.
/// This field exposes the amount and currency of the flat tax rate.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxRateFlatAmount {
    /// Amount of the tax when the `rate_type` is `flat_amount`.
    /// This positive integer represents how much to charge in the smallest currency unit (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Three-letter ISO currency code, in lowercase.
    pub currency: stripe_types::Currency,
}
#[doc(hidden)]
pub struct TaxRateFlatAmountBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
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

    impl Deserialize for TaxRateFlatAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxRateFlatAmount>,
        builder: TaxRateFlatAmountBuilder,
    }

    impl Visitor for Place<TaxRateFlatAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxRateFlatAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxRateFlatAmountBuilder {
        type Out = TaxRateFlatAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount: Deserialize::default(), currency: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(currency)) = (self.amount, self.currency) else {
                return None;
            };
            Some(Self::Out { amount, currency })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TaxRateFlatAmount {
        type Builder = TaxRateFlatAmountBuilder;
    }

    impl FromValueOpt for TaxRateFlatAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxRateFlatAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
