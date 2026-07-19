#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountMonthlyEstimatedRevenue {
    /// A non-negative integer representing how much to charge in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountMonthlyEstimatedRevenue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountMonthlyEstimatedRevenue").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountMonthlyEstimatedRevenueBuilder {
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
                builder: AccountMonthlyEstimatedRevenueBuilder {
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
            *self.out = Some(AccountMonthlyEstimatedRevenue { amount, currency });
            Ok(())
        }
    }
};
