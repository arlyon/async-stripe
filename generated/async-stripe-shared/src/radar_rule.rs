#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarRule {
    /// The action taken on the payment.
    pub action: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::RadarRuleId,
    /// The predicate to evaluate the payment against.
    pub predicate: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadarRule").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RadarRuleBuilder {
    action: Option<String>,
    id: Option<stripe_shared::RadarRuleId>,
    predicate: Option<String>,
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

    impl Deserialize for RadarRule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarRule>,
        builder: RadarRuleBuilder,
    }

    impl Visitor for Place<RadarRule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RadarRuleBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RadarRuleBuilder {
        type Out = RadarRule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.action),
                "id" => Deserialize::begin(&mut self.id),
                "predicate" => Deserialize::begin(&mut self.predicate),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { action: None, id: None, predicate: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(action), Some(id), Some(predicate)) =
                (self.action.take(), self.id.take(), self.predicate.take())
            else {
                return None;
            };
            Some(Self::Out { action, id, predicate })
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

    impl ObjectDeser for RadarRule {
        type Builder = RadarRuleBuilder;
    }

    impl FromValueOpt for RadarRule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RadarRuleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "action" => b.action = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "predicate" => b.predicate = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for RadarRule {
    type Id = stripe_shared::RadarRuleId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(RadarRuleId);
