/// Balance information for the FinancialAccount
#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceBalanceBuilder {
    cash: Option<std::collections::HashMap<String, i64>>,
    inbound_pending: Option<std::collections::HashMap<String, i64>>,
    outbound_pending: Option<std::collections::HashMap<String, i64>>,
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
                builder: TreasuryFinancialAccountsResourceBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceBalanceBuilder {
        type Out = TreasuryFinancialAccountsResourceBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cash" => Deserialize::begin(&mut self.cash),
                "inbound_pending" => Deserialize::begin(&mut self.inbound_pending),
                "outbound_pending" => Deserialize::begin(&mut self.outbound_pending),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                cash: Deserialize::default(),
                inbound_pending: Deserialize::default(),
                outbound_pending: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(cash), Some(inbound_pending), Some(outbound_pending)) =
                (self.cash.take(), self.inbound_pending.take(), self.outbound_pending.take())
            else {
                return None;
            };
            Some(Self::Out { cash, inbound_pending, outbound_pending })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceBalance {
        type Builder = TreasuryFinancialAccountsResourceBalanceBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cash" => b.cash = FromValueOpt::from_value(v),
                    "inbound_pending" => b.inbound_pending = FromValueOpt::from_value(v),
                    "outbound_pending" => b.outbound_pending = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
