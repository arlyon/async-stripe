/// OutboundTransfers contains outbound transfers features for a FinancialAccount.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    pub ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,
    pub us_domestic_wire: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfersBuilder {
    ach:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>>,
    us_domestic_wire:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceOutboundTransfers {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceOutboundTransfers>,
        builder: TreasuryFinancialAccountsResourceOutboundTransfersBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceOutboundTransfers> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceOutboundTransfersBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceOutboundTransfersBuilder {
        type Out = TreasuryFinancialAccountsResourceOutboundTransfers;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach" => Deserialize::begin(&mut self.ach),
                "us_domestic_wire" => Deserialize::begin(&mut self.us_domestic_wire),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { ach: Deserialize::default(), us_domestic_wire: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(ach), Some(us_domestic_wire)) =
                (self.ach.take(), self.us_domestic_wire.take())
            else {
                return None;
            };
            Some(Self::Out { ach, us_domestic_wire })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceOutboundTransfers {
        type Builder = TreasuryFinancialAccountsResourceOutboundTransfersBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceOutboundTransfers {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceOutboundTransfersBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ach" => b.ach = FromValueOpt::from_value(v),
                    "us_domestic_wire" => b.us_domestic_wire = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
