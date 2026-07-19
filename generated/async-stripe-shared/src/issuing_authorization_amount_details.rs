#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
    /// The amount of cash requested by the cardholder.
    pub cashback_amount: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationAmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationAmountDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationAmountDetailsBuilder {
    atm_fee: Option<Option<i64>>,
    cashback_amount: Option<Option<i64>>,
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

    impl Deserialize for IssuingAuthorizationAmountDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationAmountDetails>,
        builder: IssuingAuthorizationAmountDetailsBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationAmountDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationAmountDetailsBuilder {
                    atm_fee: Deserialize::default(),
                    cashback_amount: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "atm_fee" => Deserialize::begin(&mut self.builder.atm_fee),
                "cashback_amount" => Deserialize::begin(&mut self.builder.cashback_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(atm_fee), Some(cashback_amount)) =
                (self.builder.atm_fee, self.builder.cashback_amount)
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationAmountDetails { atm_fee, cashback_amount });
            Ok(())
        }
    }
};
