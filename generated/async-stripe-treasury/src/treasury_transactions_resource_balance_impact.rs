/// Change to a FinancialAccount's balance
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryTransactionsResourceBalanceImpact {
    /// The change made to funds the user can spend right now.
    pub cash: i64,
    /// The change made to funds that are not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,
    /// The change made to funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionsResourceBalanceImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryTransactionsResourceBalanceImpact").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryTransactionsResourceBalanceImpactBuilder {
    cash: Option<i64>,
    inbound_pending: Option<i64>,
    outbound_pending: Option<i64>,
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

    impl Deserialize for TreasuryTransactionsResourceBalanceImpact {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransactionsResourceBalanceImpact>,
        builder: TreasuryTransactionsResourceBalanceImpactBuilder,
    }

    impl Visitor for Place<TreasuryTransactionsResourceBalanceImpact> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryTransactionsResourceBalanceImpactBuilder {
                    cash: Deserialize::default(),
                    inbound_pending: Deserialize::default(),
                    outbound_pending: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cash" => Deserialize::begin(&mut self.builder.cash),
                "inbound_pending" => Deserialize::begin(&mut self.builder.inbound_pending),
                "outbound_pending" => Deserialize::begin(&mut self.builder.outbound_pending),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(cash), Some(inbound_pending), Some(outbound_pending)) =
                (self.builder.cash, self.builder.inbound_pending, self.builder.outbound_pending)
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryTransactionsResourceBalanceImpact {
                cash,
                inbound_pending,
                outbound_pending,
            });
            Ok(())
        }
    }
};
