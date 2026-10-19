#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    pub transaction_type: Option<stripe_shared::TransactionType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsAcssDebitMandateOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder {
    transaction_type: Option<Option<stripe_shared::TransactionType>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsAcssDebitMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsAcssDebitMandateOptions>,
        builder: InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsAcssDebitMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder {
        type Out = InvoicePaymentMethodOptionsAcssDebitMandateOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "transaction_type" => Deserialize::begin(&mut self.transaction_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { transaction_type: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(transaction_type),) = (self.transaction_type.take(),) else {
                return None;
            };
            Some(Self::Out { transaction_type })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsAcssDebitMandateOptions {
        type Builder = InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsAcssDebitMandateOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentMethodOptionsAcssDebitMandateOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "transaction_type" => b.transaction_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Transaction type of the mandate.
pub type InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType =
    stripe_shared::TransactionType;
