#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsGiropay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsGiropay").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodDetailsGiropayBuilder {
                    bank_code: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    bic: Deserialize::default(),
                    verified_name: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.builder.bank_code),
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "bic" => Deserialize::begin(&mut self.builder.bic),
                "verified_name" => Deserialize::begin(&mut self.builder.verified_name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank_code), Some(bank_name), Some(bic), Some(verified_name)) = (
                self.builder.bank_code.take(),
                self.builder.bank_name.take(),
                self.builder.bic.take(),
                self.builder.verified_name.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentMethodDetailsGiropay { bank_code, bank_name, bic, verified_name });
            Ok(())
        }
    }
};
