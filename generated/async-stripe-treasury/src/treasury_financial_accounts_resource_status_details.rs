#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    /// Details related to the closure of this FinancialAccount
    pub closed: Option<stripe_treasury::TreasuryFinancialAccountsResourceClosedStatusDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountsResourceStatusDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceStatusDetailsBuilder {
    closed: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceClosedStatusDetails>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceStatusDetails>,
        builder: TreasuryFinancialAccountsResourceStatusDetailsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceStatusDetailsBuilder {
                    closed: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "closed" => Deserialize::begin(&mut self.builder.closed),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(closed),) = (self.builder.closed.take(),) else {
                return Ok(());
            };
            *self.out = Some(TreasuryFinancialAccountsResourceStatusDetails { closed });
            Ok(())
        }
    }
};
