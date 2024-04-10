#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentProcessingCustomerNotification {
    /// Whether customer approval has been requested for this payment.
    /// For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
    pub approval_requested: Option<bool>,
    /// If customer approval is required, they need to provide approval before this time.
    pub completes_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct PaymentIntentProcessingCustomerNotificationBuilder {
    approval_requested: Option<Option<bool>>,
    completes_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentProcessingCustomerNotification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentProcessingCustomerNotification>,
        builder: PaymentIntentProcessingCustomerNotificationBuilder,
    }

    impl Visitor for Place<PaymentIntentProcessingCustomerNotification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentProcessingCustomerNotificationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentProcessingCustomerNotificationBuilder {
        type Out = PaymentIntentProcessingCustomerNotification;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "approval_requested" => Deserialize::begin(&mut self.approval_requested),
                "completes_at" => Deserialize::begin(&mut self.completes_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                approval_requested: Deserialize::default(),
                completes_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                approval_requested: self.approval_requested?,
                completes_at: self.completes_at?,
            })
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

    impl ObjectDeser for PaymentIntentProcessingCustomerNotification {
        type Builder = PaymentIntentProcessingCustomerNotificationBuilder;
    }

    impl FromValueOpt for PaymentIntentProcessingCustomerNotification {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentProcessingCustomerNotificationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "approval_requested" => {
                        b.approval_requested = Some(FromValueOpt::from_value(v)?)
                    }
                    "completes_at" => b.completes_at = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
