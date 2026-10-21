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
            Self { action: None, id: None, predicate: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { action: Some(action), id: Some(id), predicate: Some(predicate) } => {
                    Some(Self::Out { action, id, predicate })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for Rule {
        type Builder = RuleBuilder;
    }

    impl FromValueOpt for Rule {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
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
