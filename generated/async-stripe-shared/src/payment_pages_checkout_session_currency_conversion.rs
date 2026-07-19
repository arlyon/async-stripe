#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCurrencyConversion {
    /// Total of all items in source currency before discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total of all items in source currency after discounts and taxes are applied.
    pub amount_total: i64,
    /// Exchange rate used to convert source currency amounts to customer currency amounts
    pub fx_rate: String,
    /// Creation currency of the CheckoutSession before localization
    pub source_currency: stripe_types::Currency,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCurrencyConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionCurrencyConversion").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCurrencyConversionBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    fx_rate: Option<String>,
    source_currency: Option<stripe_types::Currency>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCurrencyConversion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCurrencyConversion>,
        builder: PaymentPagesCheckoutSessionCurrencyConversionBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCurrencyConversion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCurrencyConversionBuilder {
                    amount_subtotal: Deserialize::default(),
                    amount_total: Deserialize::default(),
                    fx_rate: Deserialize::default(),
                    source_currency: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.builder.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.builder.amount_total),
                "fx_rate" => Deserialize::begin(&mut self.builder.fx_rate),
                "source_currency" => Deserialize::begin(&mut self.builder.source_currency),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_subtotal), Some(amount_total), Some(fx_rate), Some(source_currency)) = (
                self.builder.amount_subtotal,
                self.builder.amount_total,
                self.builder.fx_rate.take(),
                self.builder.source_currency.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionCurrencyConversion {
                amount_subtotal,
                amount_total,
                fx_rate,
                source_currency,
            });
            Ok(())
        }
    }
};
