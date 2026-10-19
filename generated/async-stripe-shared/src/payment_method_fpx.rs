#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodFpx {
    /// Account holder type, if provided. Can be one of `individual` or `company`.
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// The customer's bank, if provided.
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bnp_paribas`, `bank_rakyat`, `bsn`, `cimb`, `citibank`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `mbsb_bank`, `pb_enterprise`, or `bank_of_china`.
    pub bank: stripe_shared::PaymentMethodFpxBank,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodFpx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodFpx").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodFpxBuilder {
    account_holder_type: Option<Option<stripe_shared::IssuingCardholderType>>,
    bank: Option<stripe_shared::PaymentMethodFpxBank>,
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

    impl Deserialize for PaymentMethodFpx {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodFpx>,
        builder: PaymentMethodFpxBuilder,
    }

    impl Visitor for Place<PaymentMethodFpx> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodFpxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodFpxBuilder {
        type Out = PaymentMethodFpx;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "bank" => Deserialize::begin(&mut self.bank),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { account_holder_type: Some(None), bank: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account_holder_type), Some(bank)) =
                (self.account_holder_type.take(), self.bank.take())
            else {
                return None;
            };
            Some(Self::Out { account_holder_type, bank })
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

    impl ObjectDeser for PaymentMethodFpx {
        type Builder = PaymentMethodFpxBuilder;
    }

    impl FromValueOpt for PaymentMethodFpx {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodFpxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => b.account_holder_type = FromValueOpt::from_value(v),
                    "bank" => b.bank = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Account holder type, if provided. Can be one of `individual` or `company`.
pub type PaymentMethodFpxAccountHolderType = stripe_shared::IssuingCardholderType;
/// The customer's bank, if provided.
/// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bnp_paribas`, `bank_rakyat`, `bsn`, `cimb`, `citibank`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `mbsb_bank`, `pb_enterprise`, or `bank_of_china`.
pub type PaymentMethodFpxBank = stripe_shared::PaymentMethodFpxBank;
