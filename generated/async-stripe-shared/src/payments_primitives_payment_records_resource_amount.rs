/// A representation of an amount of money, consisting of an amount and a currency.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceAmount {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A positive integer representing the amount in the currency's [minor unit](https://stripe.com/docs/currencies#zero-decimal).
    /// For example, `100` can represent 1 USD or 100 JPY.
    pub value: i64,
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceAmountBuilder {
    currency: Option<stripe_types::Currency>,
    value: Option<i64>,
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
                builder: PaymentsPrimitivesPaymentRecordsResourceAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentsPrimitivesPaymentRecordsResourceAmountBuilder {
        type Out = PaymentsPrimitivesPaymentRecordsResourceAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.currency),
                "value" => Deserialize::begin(&mut self.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { currency: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(currency), Some(value)) = (self.currency.take(), self.value) else {
                return None;
            };
            Some(Self::Out { currency, value })
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

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourceAmount {
        type Builder = PaymentsPrimitivesPaymentRecordsResourceAmountBuilder;
    }

    impl FromValueOpt for PaymentsPrimitivesPaymentRecordsResourceAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentsPrimitivesPaymentRecordsResourceAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
