/// Details of an rejected outcome attached to this payment evaluation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationRejected {
    pub card: Option<stripe_fraud::InsightsResourcesPaymentEvaluationRejectedCard>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationRejected").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationRejectedBuilder {
    card: Option<stripe_fraud::InsightsResourcesPaymentEvaluationRejectedCard>,
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
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { card } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { card })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationRejected {
        type Builder = InsightsResourcesPaymentEvaluationRejectedBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationRejected {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
