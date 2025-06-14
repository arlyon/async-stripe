#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceAmountNet {
    /// Balance amount.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Breakdown of balance by destination.
    pub net_available: Option<Vec<stripe_core::BalanceNetAvailable>>,
    pub source_types: Option<stripe_core::BalanceAmountBySourceType>,
}
#[doc(hidden)]
pub struct BalanceAmountNetBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    net_available: Option<Option<Vec<stripe_core::BalanceNetAvailable>>>,
    source_types: Option<Option<stripe_core::BalanceAmountBySourceType>>,
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

    impl Deserialize for BalanceAmountNet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceAmountNet>,
        builder: BalanceAmountNetBuilder,
    }

    impl Visitor for Place<BalanceAmountNet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceAmountNetBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceAmountNetBuilder {
        type Out = BalanceAmountNet;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "net_available" => Deserialize::begin(&mut self.net_available),
                "source_types" => Deserialize::begin(&mut self.source_types),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                currency: Deserialize::default(),
                net_available: Deserialize::default(),
                source_types: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(currency), Some(net_available), Some(source_types)) =
                (self.amount, self.currency.take(), self.net_available.take(), self.source_types)
            else {
                return None;
            };
            Some(Self::Out { amount, currency, net_available, source_types })
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

    impl ObjectDeser for BalanceAmountNet {
        type Builder = BalanceAmountNetBuilder;
    }

    impl FromValueOpt for BalanceAmountNet {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceAmountNetBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "net_available" => b.net_available = FromValueOpt::from_value(v),
                    "source_types" => b.source_types = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
