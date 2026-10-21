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
            Self { atm_fee: Some(None), cashback_amount: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { atm_fee: Some(atm_fee), cashback_amount: Some(cashback_amount) } => {
                    Some(Self::Out { atm_fee, cashback_amount })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for IssuingAuthorizationAmountDetails {
        type Builder = IssuingAuthorizationAmountDetailsBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationAmountDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
