/// InboundTransfers contains inbound transfers features for a FinancialAccount.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
    pub ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceAchToggleSettings>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceInboundTransfersBuilder {
    ach: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceAchToggleSettings>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceInboundTransfers {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceInboundTransfers>,
        builder: TreasuryFinancialAccountsResourceInboundTransfersBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceInboundTransfers> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceInboundTransfersBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceInboundTransfersBuilder {
        type Out = TreasuryFinancialAccountsResourceInboundTransfers;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach" => Deserialize::begin(&mut self.ach),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { ach: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { ach: self.ach.take()? })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceInboundTransfers {
        type Builder = TreasuryFinancialAccountsResourceInboundTransfersBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceInboundTransfers {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceInboundTransfersBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ach" => b.ach = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
