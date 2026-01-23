/// Collection of scores and insights for this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationInsights {
    /// Stripe Radar's evaluation of the likelihood of a card issuer decline on this payment.
    pub card_issuer_decline:
        Option<stripe_fraud::InsightsResourcesPaymentEvaluationCardIssuerDecline>,
    /// The timestamp when the evaluation was performed.
    pub evaluated_at: stripe_types::Timestamp,
    pub fraudulent_dispute: stripe_fraud::InsightsResourcesPaymentEvaluationScorer,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationInsightsBuilder {
    card_issuer_decline:
        Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationCardIssuerDecline>>,
    evaluated_at: Option<stripe_types::Timestamp>,
    fraudulent_dispute: Option<stripe_fraud::InsightsResourcesPaymentEvaluationScorer>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationInsights {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationInsights>,
        builder: InsightsResourcesPaymentEvaluationInsightsBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationInsights> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationInsightsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationInsightsBuilder {
        type Out = InsightsResourcesPaymentEvaluationInsights;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_issuer_decline" => Deserialize::begin(&mut self.card_issuer_decline),
                "evaluated_at" => Deserialize::begin(&mut self.evaluated_at),
                "fraudulent_dispute" => Deserialize::begin(&mut self.fraudulent_dispute),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card_issuer_decline: Deserialize::default(),
                evaluated_at: Deserialize::default(),
                fraudulent_dispute: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card_issuer_decline), Some(evaluated_at), Some(fraudulent_dispute)) = (
                self.card_issuer_decline.take(),
                self.evaluated_at,
                self.fraudulent_dispute.take(),
            ) else {
                return None;
            };
            Some(Self::Out { card_issuer_decline, evaluated_at, fraudulent_dispute })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for InsightsResourcesPaymentEvaluationInsights {
        type Builder = InsightsResourcesPaymentEvaluationInsightsBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationInsights {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationInsightsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_issuer_decline" => b.card_issuer_decline = FromValueOpt::from_value(v),
                    "evaluated_at" => b.evaluated_at = FromValueOpt::from_value(v),
                    "fraudulent_dispute" => b.fraudulent_dispute = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
