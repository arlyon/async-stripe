/// The amount of the tax rate when the `rate_type`` is `flat_amount`.
/// Tax rates with `rate_type` `percentage` can vary based on the transaction, resulting in this field being `null`.
/// This field exposes the amount and currency of the flat tax rate.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxRateFlatAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxRateFlatAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxRateFlatAmountBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
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
                builder: TaxRateFlatAmountBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(currency)) =
                (self.builder.amount, self.builder.currency.take())
            else {
                return Ok(());
            };
            *self.out = Some(TaxRateFlatAmount { amount, currency });
            Ok(())
        }
    }
};
