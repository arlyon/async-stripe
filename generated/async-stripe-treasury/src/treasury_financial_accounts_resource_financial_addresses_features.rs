/// Settings related to Financial Addresses features on a Financial Account
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    pub aba: Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaToggleSettings>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder {
    aba: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaToggleSettings>>,
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
            builder: TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder {
        type Out = TreasuryFinancialAccountsResourceFinancialAddressesFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "aba" => Deserialize::begin(&mut self.aba),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { aba: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(aba),) = (self.aba.take(),) else {
                return None;
            };
            Some(Self::Out { aba })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
        type Builder = TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TreasuryFinancialAccountsResourceFinancialAddressesFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "aba" => b.aba = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
