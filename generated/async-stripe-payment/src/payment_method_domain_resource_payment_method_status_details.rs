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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder {
                    error_message: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "error_message" => Deserialize::begin(&mut self.builder.error_message),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(error_message),) = (self.builder.error_message.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentMethodDomainResourcePaymentMethodStatusDetails { error_message });
            Ok(())
        }
    }
};
