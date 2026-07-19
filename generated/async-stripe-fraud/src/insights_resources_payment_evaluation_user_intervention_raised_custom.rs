/// User intervention raised custom event details attached to this payment evaluation
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom {
    /// Custom type of user intervention raised.
    /// The string must use a snake case description for the type of intervention performed.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder {
    type_: Option<String>,
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
                builder: InsightsResourcesPaymentEvaluationUserInterventionRaisedCustomBuilder {
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(type_),) = (self.builder.type_.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom { type_ });
            Ok(())
        }
    }
};
