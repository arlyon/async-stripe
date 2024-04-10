#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionCardAwaitNotification {
    /// The time that payment will be attempted.
    /// If customer approval is required, they need to provide approval before this time.
    pub charge_attempt_at: Option<stripe_types::Timestamp>,
    /// For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank.
    /// For payments of lower amount, no customer action is required.
    pub customer_approval_required: Option<bool>,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionCardAwaitNotificationBuilder {
    charge_attempt_at: Option<Option<stripe_types::Timestamp>>,
    customer_approval_required: Option<Option<bool>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionCardAwaitNotification {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionCardAwaitNotification>,
        builder: PaymentIntentNextActionCardAwaitNotificationBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionCardAwaitNotification> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionCardAwaitNotificationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionCardAwaitNotificationBuilder {
        type Out = PaymentIntentNextActionCardAwaitNotification;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge_attempt_at" => Deserialize::begin(&mut self.charge_attempt_at),
                "customer_approval_required" => {
                    Deserialize::begin(&mut self.customer_approval_required)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                charge_attempt_at: Deserialize::default(),
                customer_approval_required: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                charge_attempt_at: self.charge_attempt_at?,
                customer_approval_required: self.customer_approval_required?,
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

    impl ObjectDeser for PaymentIntentNextActionCardAwaitNotification {
        type Builder = PaymentIntentNextActionCardAwaitNotificationBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionCardAwaitNotification {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionCardAwaitNotificationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "charge_attempt_at" => b.charge_attempt_at = Some(FromValueOpt::from_value(v)?),
                    "customer_approval_required" => {
                        b.customer_approval_required = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
