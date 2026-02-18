#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditBalance {
    pub available_balance: stripe_shared::BillingCreditGrantsResourceAmount,
    pub ledger_balance: stripe_shared::BillingCreditGrantsResourceAmount,
}
#[doc(hidden)]
pub struct CreditBalanceBuilder {
    available_balance: Option<stripe_shared::BillingCreditGrantsResourceAmount>,
    ledger_balance: Option<stripe_shared::BillingCreditGrantsResourceAmount>,
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
                builder: CreditBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditBalanceBuilder {
        type Out = CreditBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available_balance" => Deserialize::begin(&mut self.available_balance),
                "ledger_balance" => Deserialize::begin(&mut self.ledger_balance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                available_balance: Deserialize::default(),
                ledger_balance: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(available_balance), Some(ledger_balance)) =
                (self.available_balance.take(), self.ledger_balance.take())
            else {
                return None;
            };
            Some(Self::Out { available_balance, ledger_balance })
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

    impl ObjectDeser for CreditBalance {
        type Builder = CreditBalanceBuilder;
    }

    impl FromValueOpt for CreditBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available_balance" => b.available_balance = FromValueOpt::from_value(v),
                    "ledger_balance" => b.ledger_balance = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
