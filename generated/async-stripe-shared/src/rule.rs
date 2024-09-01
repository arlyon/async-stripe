#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Rule {
    /// The action taken on the payment.
    pub action: String,
    /// Unique identifier for the object.
    pub id: stripe_shared::RuleId,
    /// The predicate to evaluate the payment against.
    pub predicate: String,
}
#[doc(hidden)]
pub struct RuleBuilder {
    action: Option<String>,
    id: Option<stripe_shared::RuleId>,
    predicate: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Rule {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Rule>,
        builder: RuleBuilder,
    }

    impl Visitor for Place<Rule> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: RuleBuilder::deser_default() }))
        }
    }

    impl MapBuilder for RuleBuilder {
        type Out = Rule;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.action),
                "id" => Deserialize::begin(&mut self.id),
                "predicate" => Deserialize::begin(&mut self.predicate),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                action: Deserialize::default(),
                id: Deserialize::default(),
                predicate: Deserialize::default(),
            }
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

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Rule {
        type Builder = RuleBuilder;
    }

    impl FromValueOpt for Rule {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RuleBuilder::deser_default();
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
impl stripe_types::Object for Rule {
    type Id = stripe_shared::RuleId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(RuleId);
