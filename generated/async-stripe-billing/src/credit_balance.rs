#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditBalance {
    pub available_balance: stripe_shared::BillingCreditGrantsResourceAmount,
    pub ledger_balance: stripe_shared::BillingCreditGrantsResourceAmount,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditBalance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CreditBalanceBuilder {
    available_balance: Option<stripe_shared::BillingCreditGrantsResourceAmount>,
    ledger_balance: Option<stripe_shared::BillingCreditGrantsResourceAmount>,
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

    impl Deserialize for CreditBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditBalance>,
        builder: CreditBalanceBuilder,
    }

    impl Visitor for Place<CreditBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditBalanceBuilder {
                    available_balance: Deserialize::default(),
                    ledger_balance: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available_balance" => Deserialize::begin(&mut self.builder.available_balance),
                "ledger_balance" => Deserialize::begin(&mut self.builder.ledger_balance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available_balance), Some(ledger_balance)) =
                (self.builder.available_balance.take(), self.builder.ledger_balance.take())
            else {
                return Ok(());
            };
            *self.out = Some(CreditBalance { available_balance, ledger_balance });
            Ok(())
        }
    }
};
