/// Money Movement card details attached to this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementCard {
    /// Describes the presence of the customer during the payment.
    pub customer_presence: Option<stripe_shared::CreateSetupIntentUsage>,
    /// Describes the type of payment.
    pub payment_type: Option<stripe_shared::PaymentType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMoneyMovementCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationMoneyMovementCard")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder {
    customer_presence: Option<Option<stripe_shared::CreateSetupIntentUsage>>,
    payment_type: Option<Option<stripe_shared::PaymentType>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationMoneyMovementCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationMoneyMovementCard>,
        builder: InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationMoneyMovementCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder {
        type Out = InsightsResourcesPaymentEvaluationMoneyMovementCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_presence" => Deserialize::begin(&mut self.customer_presence),
                "payment_type" => Deserialize::begin(&mut self.payment_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { customer_presence: Some(None), payment_type: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer_presence), Some(payment_type)) =
                (self.customer_presence.take(), self.payment_type.take())
            else {
                return None;
            };
            Some(Self::Out { customer_presence, payment_type })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationMoneyMovementCard {
        type Builder = InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationMoneyMovementCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_presence" => b.customer_presence = FromValueOpt::from_value(v),
                    "payment_type" => b.payment_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Describes the presence of the customer during the payment.
pub type InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence =
    stripe_shared::CreateSetupIntentUsage;
/// Describes the type of payment.
pub type InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType =
    stripe_shared::PaymentType;
