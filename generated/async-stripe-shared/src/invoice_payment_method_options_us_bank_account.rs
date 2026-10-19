#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsUsBankAccount {
    pub financial_connections:
        Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions>,
    /// Bank account verification method. The default value is `automatic`.
    pub verification_method: Option<stripe_shared::VerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsUsBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsUsBankAccountBuilder {
    financial_connections:
        Option<Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsUsBankAccount>,
        builder: InvoicePaymentMethodOptionsUsBankAccountBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsUsBankAccountBuilder {
        type Out = InvoicePaymentMethodOptionsUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "financial_connections" => Deserialize::begin(&mut self.financial_connections),
                "verification_method" => Deserialize::begin(&mut self.verification_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { financial_connections: Some(None), verification_method: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(financial_connections), Some(verification_method)) =
                (self.financial_connections.take(), self.verification_method.take())
            else {
                return None;
            };
            Some(Self::Out { financial_connections, verification_method })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsUsBankAccount {
        type Builder = InvoicePaymentMethodOptionsUsBankAccountBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentMethodOptionsUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "financial_connections" => {
                        b.financial_connections = FromValueOpt::from_value(v)
                    }
                    "verification_method" => b.verification_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Bank account verification method. The default value is `automatic`.
pub type InvoicePaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_shared::VerificationMethod;
