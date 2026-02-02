/// User intervention raised custom event details attached to this payment evaluation
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom {
    /// Custom type of user intervention raised.
    /// The string must use a snake case description for the type of intervention performed.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder {
    type_: Option<String>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom>,
        builder: InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder {
        type Out = InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(type_),) = (self.type_.take(),) else {
                return None;
            };
            Some(Self::Out { type_ })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom {
        type Builder = InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
