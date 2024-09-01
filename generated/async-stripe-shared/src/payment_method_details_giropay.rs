#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsGiropay {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// Owner's verified full name. Values are verified or provided by Giropay directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    /// Giropay rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsGiropayBuilder {
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    verified_name: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsGiropay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsGiropay>,
        builder: PaymentMethodDetailsGiropayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsGiropay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsGiropayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsGiropayBuilder {
        type Out = PaymentMethodDetailsGiropay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "bic" => Deserialize::begin(&mut self.bic),
                "verified_name" => Deserialize::begin(&mut self.verified_name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                bank_name: Deserialize::default(),
                bic: Deserialize::default(),
                verified_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_code), Some(bank_name), Some(bic), Some(verified_name)) = (
                self.bank_code.take(),
                self.bank_name.take(),
                self.bic.take(),
                self.verified_name.take(),
            ) else {
                return None;
            };
            Some(Self::Out { bank_code, bank_name, bic, verified_name })
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

    impl ObjectDeser for PaymentMethodDetailsGiropay {
        type Builder = PaymentMethodDetailsGiropayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsGiropay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsGiropayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_code" => b.bank_code = FromValueOpt::from_value(v),
                    "bank_name" => b.bank_name = FromValueOpt::from_value(v),
                    "bic" => b.bic = FromValueOpt::from_value(v),
                    "verified_name" => b.verified_name = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
