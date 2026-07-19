/// Settings related to Financial Addresses features on a Financial Account
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    pub aba: Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaToggleSettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountsResourceFinancialAddressesFeatures")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder {
    aba: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaToggleSettings>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
        builder: TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceFinancialAddressesFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder {
                    aba: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aba" => Deserialize::begin(&mut self.builder.aba),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(aba),) = (self.builder.aba.take(),) else {
                return Ok(());
            };
            *self.out = Some(TreasuryFinancialAccountsResourceFinancialAddressesFeatures { aba });
            Ok(())
        }
    }
};
