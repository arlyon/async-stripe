/// Change to a FinancialAccount's balance
#[derive(Copy, Clone, Debug)]
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
#[doc(hidden)]
pub struct TreasuryTransactionsResourceBalanceImpactBuilder {
    cash: Option<i64>,
    inbound_pending: Option<i64>,
    outbound_pending: Option<i64>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: TreasuryTransactionsResourceBalanceImpactBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryTransactionsResourceBalanceImpactBuilder {
        type Out = TreasuryTransactionsResourceBalanceImpact;
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
            Some(Self::Out {
                cash: self.cash?,
                inbound_pending: self.inbound_pending?,
                outbound_pending: self.outbound_pending?,
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

    impl ObjectDeser for TreasuryTransactionsResourceBalanceImpact {
        type Builder = TreasuryTransactionsResourceBalanceImpactBuilder;
    }

    impl FromValueOpt for TreasuryTransactionsResourceBalanceImpact {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryTransactionsResourceBalanceImpactBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cash" => b.cash = Some(FromValueOpt::from_value(v)?),
                    "inbound_pending" => b.inbound_pending = Some(FromValueOpt::from_value(v)?),
                    "outbound_pending" => b.outbound_pending = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
