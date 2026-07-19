#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ShippingRateFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            stripe_shared::ShippingRateCurrencyOption,
        >,
    >,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRateFixedAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingRateFixedAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ShippingRateFixedAmountBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    currency_options: Option<
        Option<
            std::collections::HashMap<
                stripe_types::Currency,
                stripe_shared::ShippingRateCurrencyOption,
            >,
        >,
    >,
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

    impl Deserialize for ShippingRateFixedAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRateFixedAmount>,
        builder: ShippingRateFixedAmountBuilder,
    }

    impl Visitor for Place<ShippingRateFixedAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ShippingRateFixedAmountBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    currency_options: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "currency_options" => Deserialize::begin(&mut self.builder.currency_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(currency), Some(currency_options)) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.currency_options.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(ShippingRateFixedAmount { amount, currency, currency_options });
            Ok(())
        }
    }
};
