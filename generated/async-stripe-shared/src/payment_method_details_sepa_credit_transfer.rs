#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsSepaCreditTransfer {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// IBAN of the bank account to transfer funds to.
    pub iban: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsSepaCreditTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsSepaCreditTransfer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsSepaCreditTransferBuilder {
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    iban: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsSepaCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsSepaCreditTransfer>,
        builder: PaymentMethodDetailsSepaCreditTransferBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsSepaCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsSepaCreditTransferBuilder {
                    bank_name: Deserialize::default(),
                    bic: Deserialize::default(),
                    iban: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "bic" => Deserialize::begin(&mut self.builder.bic),
                "iban" => Deserialize::begin(&mut self.builder.iban),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank_name), Some(bic), Some(iban)) =
                (self.builder.bank_name.take(), self.builder.bic.take(), self.builder.iban.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsSepaCreditTransfer { bank_name, bic, iban });
            Ok(())
        }
    }
};
