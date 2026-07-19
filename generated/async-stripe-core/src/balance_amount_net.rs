#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceAmountNet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceAmountNet").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BalanceAmountNetBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    net_available: Deserialize::default(),
                    source_types: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "net_available" => Deserialize::begin(&mut self.builder.net_available),
                "source_types" => Deserialize::begin(&mut self.builder.source_types),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(currency), Some(net_available), Some(source_types)) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.net_available.take(),
                self.builder.source_types,
            ) else {
                return Ok(());
            };
            *self.out = Some(BalanceAmountNet { amount, currency, net_available, source_types });
            Ok(())
        }
    }
};
