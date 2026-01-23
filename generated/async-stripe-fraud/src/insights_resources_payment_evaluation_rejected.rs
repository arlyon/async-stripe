/// Details of an rejected outcome attached to this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationRejected {
    pub card: Option<stripe_fraud::InsightsResourcesPaymentEvaluationRejectedCard>,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationRejectedBuilder {
    card: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationRejectedCard>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationRejected {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationRejected>,
        builder: InsightsResourcesPaymentEvaluationRejectedBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationRejected> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationRejectedBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationRejectedBuilder {
        type Out = InsightsResourcesPaymentEvaluationRejected;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.card),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card),) = (self.card.take(),) else {
                return None;
            };
            Some(Self::Out { card })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationRejected {
        type Builder = InsightsResourcesPaymentEvaluationRejectedBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationRejected {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationRejectedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card" => b.card = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
