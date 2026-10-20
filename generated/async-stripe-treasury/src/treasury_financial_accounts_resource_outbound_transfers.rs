/// OutboundTransfers contains outbound transfers features for a FinancialAccount.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    pub ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,
    pub us_domestic_wire: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceOutboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountsResourceOutboundTransfers").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfersBuilder {
    ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,
    us_domestic_wire: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
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
                "ach" => stripe_types::miniserde_helpers::begin_nullable(&mut self.ach),
                "us_domestic_wire" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.us_domestic_wire)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { ach: None, us_domestic_wire: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (ach, us_domestic_wire) = (self.ach.take(), self.us_domestic_wire.take()) else {
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryFinancialAccountsResourceOutboundTransfers {
        type Builder = TreasuryFinancialAccountsResourceOutboundTransfersBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceOutboundTransfers {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
