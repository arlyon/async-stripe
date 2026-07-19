#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountAnnualRevenue {
    /// A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// The close-out date of the preceding fiscal year in ISO 8601 format.
    /// E.g.
    /// 2023-12-31 for the 31st of December, 2023.
    pub fiscal_year_end: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountAnnualRevenue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountAnnualRevenue").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountAnnualRevenueBuilder {
    amount: Option<Option<i64>>,
    currency: Option<Option<stripe_types::Currency>>,
    fiscal_year_end: Option<Option<String>>,
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
                builder: AccountAnnualRevenueBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    fiscal_year_end: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "fiscal_year_end" => Deserialize::begin(&mut self.builder.fiscal_year_end),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(currency), Some(fiscal_year_end)) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.fiscal_year_end.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(AccountAnnualRevenue { amount, currency, fiscal_year_end });
            Ok(())
        }
    }
};
