/// Contains additional details about the status of a payment method for a specific payment method domain.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {
    /// The error message associated with the status of the payment method on the domain.
    pub error_message: String,
}
#[doc(hidden)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder {
    error_message: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Self { error_message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { error_message: self.error_message.take()? })
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

    impl ObjectDeser for PaymentMethodDomainResourcePaymentMethodStatusDetails {
        type Builder = PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder;
    }

    impl FromValueOpt for PaymentMethodDomainResourcePaymentMethodStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentMethodDomainResourcePaymentMethodStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "error_message" => b.error_message = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
