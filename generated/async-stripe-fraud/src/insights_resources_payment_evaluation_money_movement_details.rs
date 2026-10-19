/// Money Movement details attached to this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementDetails {
    /// Describes card money movement details for the payment evaluation.
    pub card: Option<stripe_fraud::InsightsResourcesPaymentEvaluationMoneyMovementCard>,
    /// Describes the type of money movement. Currently only `card` is supported.
    pub money_movement_type: stripe_shared::PaymentIntentProcessingType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationMoneyMovementDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder {
    card: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationMoneyMovementCard>>,
    money_movement_type: Option<stripe_shared::PaymentIntentProcessingType>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationMoneyMovementDetails>,
        builder: InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationMoneyMovementDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder {
        type Out = InsightsResourcesPaymentEvaluationMoneyMovementDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.card),
                "money_movement_type" => Deserialize::begin(&mut self.money_movement_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: Some(None), money_movement_type: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card), Some(money_movement_type)) =
                (self.card.take(), self.money_movement_type.take())
            else {
                return None;
            };
            Some(Self::Out { card, money_movement_type })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
        type Builder = InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card" => b.card = FromValueOpt::from_value(v),
                    "money_movement_type" => b.money_movement_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Describes the type of money movement. Currently only `card` is supported.
pub type InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType =
    stripe_shared::PaymentIntentProcessingType;
