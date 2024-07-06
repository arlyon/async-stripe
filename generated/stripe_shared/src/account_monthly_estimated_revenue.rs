#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountMonthlyEstimatedRevenue {
    /// A non-negative integer representing how much to charge in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
}
#[doc(hidden)]
pub struct AccountMonthlyEstimatedRevenueBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountMonthlyEstimatedRevenue {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountMonthlyEstimatedRevenue>,
        builder: AccountMonthlyEstimatedRevenueBuilder,
    }

    impl Visitor for Place<AccountMonthlyEstimatedRevenue> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountMonthlyEstimatedRevenueBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountMonthlyEstimatedRevenueBuilder {
        type Out = AccountMonthlyEstimatedRevenue;
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
            Some(Self::Out { amount: self.amount?, currency: self.currency? })
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

    impl ObjectDeser for AccountMonthlyEstimatedRevenue {
        type Builder = AccountMonthlyEstimatedRevenueBuilder;
    }

    impl FromValueOpt for AccountMonthlyEstimatedRevenue {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountMonthlyEstimatedRevenueBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
