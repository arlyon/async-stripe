#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountAnnualRevenue {
    /// A non-negative integer representing the amount in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// The close-out date of the preceding fiscal year in ISO 8601 format.
    /// E.g.
    /// 2023-12-31 for the 31st of December, 2023.
    pub fiscal_year_end: Option<String>,
}
#[doc(hidden)]
pub struct AccountAnnualRevenueBuilder {
    amount: Option<Option<i64>>,
    currency: Option<Option<stripe_types::Currency>>,
    fiscal_year_end: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountAnnualRevenue {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountAnnualRevenue>,
        builder: AccountAnnualRevenueBuilder,
    }

    impl Visitor for Place<AccountAnnualRevenue> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountAnnualRevenueBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountAnnualRevenueBuilder {
        type Out = AccountAnnualRevenue;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "fiscal_year_end" => Deserialize::begin(&mut self.fiscal_year_end),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                currency: Deserialize::default(),
                fiscal_year_end: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                currency: self.currency?,
                fiscal_year_end: self.fiscal_year_end.take()?,
            })
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

    impl ObjectDeser for AccountAnnualRevenue {
        type Builder = AccountAnnualRevenueBuilder;
    }

    impl FromValueOpt for AccountAnnualRevenue {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountAnnualRevenueBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "fiscal_year_end" => b.fiscal_year_end = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
