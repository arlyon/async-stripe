#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Rule").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RuleBuilder {
                    action: Deserialize::default(),
                    id: Deserialize::default(),
                    predicate: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "action" => Deserialize::begin(&mut self.builder.action),
                "id" => Deserialize::begin(&mut self.builder.id),
                "predicate" => Deserialize::begin(&mut self.builder.predicate),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(action), Some(id), Some(predicate)) =
                (self.builder.action.take(), self.builder.id.take(), self.builder.predicate.take())
            else {
                return Ok(());
            };
            *self.out = Some(Rule { action, id, predicate });
            Ok(())
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
