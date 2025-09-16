#[derive(Clone, Debug)]
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
                builder: PaymentPagesCheckoutSessionCurrencyConversionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCurrencyConversionBuilder {
        type Out = PaymentPagesCheckoutSessionCurrencyConversion;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "fx_rate" => Deserialize::begin(&mut self.fx_rate),
                "source_currency" => Deserialize::begin(&mut self.source_currency),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
                fx_rate: Deserialize::default(),
                source_currency: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_subtotal), Some(amount_total), Some(fx_rate), Some(source_currency)) = (
                self.amount_subtotal,
                self.amount_total,
                self.fx_rate.take(),
                self.source_currency.take(),
            ) else {
                return None;
            };
            Some(Self::Out { amount_subtotal, amount_total, fx_rate, source_currency })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCurrencyConversion {
        type Builder = PaymentPagesCheckoutSessionCurrencyConversionBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCurrencyConversion {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCurrencyConversionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_subtotal" => b.amount_subtotal = FromValueOpt::from_value(v),
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),
                    "fx_rate" => b.fx_rate = FromValueOpt::from_value(v),
                    "source_currency" => b.source_currency = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
