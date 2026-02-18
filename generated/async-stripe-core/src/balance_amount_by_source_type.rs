#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceAmountBySourceType {
    /// Amount coming from [legacy US ACH payments](https://docs.stripe.com/ach-deprecated).
    pub bank_account: Option<i64>,
    /// Amount coming from most payment methods, including cards as well as [non-legacy bank debits](https://docs.stripe.com/payments/bank-debits).
    pub card: Option<i64>,
    /// Amount coming from [FPX](https://docs.stripe.com/payments/fpx), a Malaysian payment method.
    pub fpx: Option<i64>,
}
#[doc(hidden)]
pub struct BalanceAmountBySourceTypeBuilder {
    bank_account: Option<Option<i64>>,
    card: Option<Option<i64>>,
    fpx: Option<Option<i64>>,
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

    impl Deserialize for BalanceAmountBySourceType {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceAmountBySourceType>,
        builder: BalanceAmountBySourceTypeBuilder,
    }

    impl Visitor for Place<BalanceAmountBySourceType> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceAmountBySourceTypeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceAmountBySourceTypeBuilder {
        type Out = BalanceAmountBySourceType;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_account" => Deserialize::begin(&mut self.bank_account),
                "card" => Deserialize::begin(&mut self.card),
                "fpx" => Deserialize::begin(&mut self.fpx),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_account: Deserialize::default(),
                card: Deserialize::default(),
                fpx: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_account), Some(card), Some(fpx)) =
                (self.bank_account, self.card, self.fpx)
            else {
                return None;
            };
            Some(Self::Out { bank_account, card, fpx })
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

    impl ObjectDeser for BalanceAmountBySourceType {
        type Builder = BalanceAmountBySourceTypeBuilder;
    }

    impl FromValueOpt for BalanceAmountBySourceType {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceAmountBySourceTypeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_account" => b.bank_account = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "fpx" => b.fpx = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
