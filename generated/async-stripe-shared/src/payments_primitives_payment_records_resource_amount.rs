/// A representation of an amount of money, consisting of an amount and a currency.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceAmount {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A positive integer representing the amount in the currency's [minor unit](https://docs.stripe.com/currencies#zero-decimal).
    /// For example, `100` can represent 1 USD or 100 JPY.
    pub value: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourceAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentsPrimitivesPaymentRecordsResourceAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceAmountBuilder {
    currency: Option<stripe_types::Currency>,
    value: Option<i64>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourceAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentsPrimitivesPaymentRecordsResourceAmount>,
        builder: PaymentsPrimitivesPaymentRecordsResourceAmountBuilder,
    }

    impl Visitor for Place<PaymentsPrimitivesPaymentRecordsResourceAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentsPrimitivesPaymentRecordsResourceAmountBuilder {
                    currency: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(currency), Some(value)) = (self.builder.currency.take(), self.builder.value)
            else {
                return Ok(());
            };
            *self.out = Some(PaymentsPrimitivesPaymentRecordsResourceAmount { currency, value });
            Ok(())
        }
    }
};
