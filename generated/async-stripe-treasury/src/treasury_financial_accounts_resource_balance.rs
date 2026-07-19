/// Balance information for the FinancialAccount
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceBalance {
    /// Funds the user can spend right now.
    pub cash: std::collections::HashMap<String, i64>,
    /// Funds not spendable yet, but will become available at a later time.
    pub inbound_pending: std::collections::HashMap<String, i64>,
    /// Funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: std::collections::HashMap<String, i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountsResourceBalance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceBalanceBuilder {
    cash: Option<std::collections::HashMap<String, i64>>,
    inbound_pending: Option<std::collections::HashMap<String, i64>>,
    outbound_pending: Option<std::collections::HashMap<String, i64>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceBalance>,
        builder: TreasuryFinancialAccountsResourceBalanceBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceBalanceBuilder {
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
            let (Some(cash), Some(inbound_pending), Some(outbound_pending)) = (
                self.builder.cash.take(),
                self.builder.inbound_pending.take(),
                self.builder.outbound_pending.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TreasuryFinancialAccountsResourceBalance {
                cash,
                inbound_pending,
                outbound_pending,
            });
            Ok(())
        }
    }
};
