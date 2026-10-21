/// Contains additional details about the status of a payment method for a specific payment method domain.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {
    /// The error message associated with the status of the payment method on the domain.
    pub error_message: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDomainResourcePaymentMethodStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDomainResourcePaymentMethodStatusDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder {
    error_message: Option<String>,
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

    impl Deserialize for PaymentMethodDomainResourcePaymentMethodStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDomainResourcePaymentMethodStatusDetails>,
        builder: PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder,
    }

    impl Visitor for Place<PaymentMethodDomainResourcePaymentMethodStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder {
        type Out = PaymentMethodDomainResourcePaymentMethodStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "error_message" => Deserialize::begin(&mut self.error_message),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { error_message: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { error_message: Some(error_message) } => Some(Self::Out { error_message }),
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

    impl ObjectDeser for PaymentMethodDomainResourcePaymentMethodStatusDetails {
        type Builder = PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder;
    }

    impl FromValueOpt for PaymentMethodDomainResourcePaymentMethodStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
