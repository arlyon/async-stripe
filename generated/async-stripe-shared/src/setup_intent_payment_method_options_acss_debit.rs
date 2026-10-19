#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsAcssDebit {
    /// Currency supported by the bank account
    pub currency: Option<stripe_shared::Currency>,
    pub mandate_options:
        Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
    /// Bank account verification method. The default value is `automatic`.
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsAcssDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsAcssDebitBuilder {
    currency: Option<Option<stripe_shared::Currency>>,
    mandate_options:
        Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>>,
    verification_method: Option<Option<stripe_shared::VerificationMethod>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsAcssDebit>,
        builder: SetupIntentPaymentMethodOptionsAcssDebitBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsAcssDebitBuilder {
        type Out = SetupIntentPaymentMethodOptionsAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.currency),
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                "verification_method" => Deserialize::begin(&mut self.verification_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                currency: Some(None),
                mandate_options: Some(None),
                verification_method: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(currency), Some(mandate_options), Some(verification_method)) = (
                self.currency.take(),
                self.mandate_options.take(),
                self.verification_method.take(),
            ) else {
                return None;
            };
            Some(Self::Out { currency, mandate_options, verification_method })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsAcssDebit {
        type Builder = SetupIntentPaymentMethodOptionsAcssDebitBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsAcssDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    "verification_method" => b.verification_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Currency supported by the bank account
pub type SetupIntentPaymentMethodOptionsAcssDebitCurrency = stripe_shared::Currency;
/// Bank account verification method. The default value is `automatic`.
pub type SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_shared::VerificationMethod;
