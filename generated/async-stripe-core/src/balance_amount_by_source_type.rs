#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceAmountBySourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceAmountBySourceType").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BalanceAmountBySourceTypeBuilder {
                    bank_account: Deserialize::default(),
                    card: Deserialize::default(),
                    fpx: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_account" => Deserialize::begin(&mut self.builder.bank_account),
                "card" => Deserialize::begin(&mut self.builder.card),
                "fpx" => Deserialize::begin(&mut self.builder.fpx),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank_account), Some(card), Some(fpx)) =
                (self.builder.bank_account, self.builder.card, self.builder.fpx)
            else {
                return Ok(());
            };
            *self.out = Some(BalanceAmountBySourceType { bank_account, card, fpx });
            Ok(())
        }
    }
};
