#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionCardAwaitNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionCardAwaitNotification").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionCardAwaitNotificationBuilder {
    charge_attempt_at: Option<Option<stripe_types::Timestamp>>,
    customer_approval_required: Option<Option<bool>>,
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
                builder: PaymentIntentNextActionCardAwaitNotificationBuilder {
                    charge_attempt_at: Deserialize::default(),
                    customer_approval_required: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge_attempt_at" => Deserialize::begin(&mut self.builder.charge_attempt_at),
                "customer_approval_required" => {
                    Deserialize::begin(&mut self.builder.customer_approval_required)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(charge_attempt_at), Some(customer_approval_required)) =
                (self.builder.charge_attempt_at, self.builder.customer_approval_required)
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionCardAwaitNotification {
                charge_attempt_at,
                customer_approval_required,
            });
            Ok(())
        }
    }
};
