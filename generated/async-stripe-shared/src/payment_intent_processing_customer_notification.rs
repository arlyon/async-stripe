#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentProcessingCustomerNotification {
    /// Whether customer approval has been requested for this payment.
    /// For payments greater than INR 15000 or mandate amount, the customer must provide explicit approval of the payment with their bank.
    pub approval_requested: Option<bool>,
    /// If customer approval is required, they need to provide approval before this time.
    pub completes_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentProcessingCustomerNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentProcessingCustomerNotification").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentProcessingCustomerNotificationBuilder {
    approval_requested: Option<Option<bool>>,
    completes_at: Option<Option<stripe_types::Timestamp>>,
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
                builder: PaymentIntentProcessingCustomerNotificationBuilder {
                    approval_requested: Deserialize::default(),
                    completes_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "approval_requested" => Deserialize::begin(&mut self.builder.approval_requested),
                "completes_at" => Deserialize::begin(&mut self.builder.completes_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(approval_requested), Some(completes_at)) =
                (self.builder.approval_requested, self.builder.completes_at)
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentProcessingCustomerNotification {
                approval_requested,
                completes_at,
            });
            Ok(())
        }
    }
};
