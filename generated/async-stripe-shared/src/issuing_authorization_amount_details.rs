#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
    /// The amount of cash requested by the cardholder.
    pub cashback_amount: Option<i64>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationAmountDetailsBuilder {
    atm_fee: Option<Option<i64>>,
    cashback_amount: Option<Option<i64>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: IssuingAuthorizationAmountDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationAmountDetailsBuilder {
        type Out = IssuingAuthorizationAmountDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "atm_fee" => Deserialize::begin(&mut self.atm_fee),
                "cashback_amount" => Deserialize::begin(&mut self.cashback_amount),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { atm_fee: Deserialize::default(), cashback_amount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(atm_fee), Some(cashback_amount)) = (self.atm_fee, self.cashback_amount)
            else {
                return None;
            };
            Some(Self::Out { atm_fee, cashback_amount })
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

    impl ObjectDeser for IssuingAuthorizationAmountDetails {
        type Builder = IssuingAuthorizationAmountDetailsBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationAmountDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationAmountDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "atm_fee" => b.atm_fee = FromValueOpt::from_value(v),
                    "cashback_amount" => b.cashback_amount = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
