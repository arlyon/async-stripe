/// Settings related to Outbound Payments features on a Financial Account
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
    pub ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,
    pub us_domestic_wire: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceOutboundPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountsResourceOutboundPayments").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceOutboundPaymentsBuilder {
    ach:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>>,
    us_domestic_wire:
        Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceOutboundPayments {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceOutboundPayments>,
        builder: TreasuryFinancialAccountsResourceOutboundPaymentsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceOutboundPayments> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceOutboundPaymentsBuilder {
                    ach: Deserialize::default(),
                    us_domestic_wire: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach" => Deserialize::begin(&mut self.builder.ach),
                "us_domestic_wire" => Deserialize::begin(&mut self.builder.us_domestic_wire),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(ach), Some(us_domestic_wire)) =
                (self.builder.ach.take(), self.builder.us_domestic_wire.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(TreasuryFinancialAccountsResourceOutboundPayments { ach, us_domestic_wire });
            Ok(())
        }
    }
};
