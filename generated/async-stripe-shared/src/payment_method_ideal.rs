#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodIdeal {
    /// The customer's bank, if provided.
    /// Can be one of `abn_amro`, `adyen`, `asn_bank`, `bunq`, `buut`, `finom`, `handelsbanken`, `ing`, `knab`, `mollie`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<stripe_shared::PaymentMethodIdealBank>,
    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<stripe_shared::Bic>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodIdeal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodIdealBuilder {
    bank: Option<Option<stripe_shared::PaymentMethodIdealBank>>,
    bic: Option<Option<stripe_shared::Bic>>,
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

    impl Deserialize for PaymentMethodIdeal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodIdeal>,
        builder: PaymentMethodIdealBuilder,
    }

    impl Visitor for Place<PaymentMethodIdeal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodIdealBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodIdealBuilder {
        type Out = PaymentMethodIdeal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank" => Deserialize::begin(&mut self.bank),
                "bic" => Deserialize::begin(&mut self.bic),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { bank: Some(None), bic: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank), Some(bic)) = (self.bank.take(), self.bic.take()) else {
                return None;
            };
            Some(Self::Out { bank, bic })
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

    impl ObjectDeser for PaymentMethodIdeal {
        type Builder = PaymentMethodIdealBuilder;
    }

    impl FromValueOpt for PaymentMethodIdeal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodIdealBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank" => b.bank = FromValueOpt::from_value(v),
                    "bic" => b.bic = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The customer's bank, if provided.
/// Can be one of `abn_amro`, `adyen`, `asn_bank`, `bunq`, `buut`, `finom`, `handelsbanken`, `ing`, `knab`, `mollie`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
pub type PaymentMethodIdealBank = stripe_shared::PaymentMethodIdealBank;
/// The Bank Identifier Code of the customer's bank, if the bank was provided.
pub type PaymentMethodIdealBic = stripe_shared::Bic;
