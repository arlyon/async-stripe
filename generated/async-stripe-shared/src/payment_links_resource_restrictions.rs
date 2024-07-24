#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceRestrictions {
    pub completed_sessions: stripe_shared::PaymentLinksResourceCompletedSessions,
}
#[doc(hidden)]
pub struct PaymentLinksResourceRestrictionsBuilder {
    completed_sessions: Option<stripe_shared::PaymentLinksResourceCompletedSessions>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceRestrictions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceRestrictions>,
        builder: PaymentLinksResourceRestrictionsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceRestrictions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceRestrictionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceRestrictionsBuilder {
        type Out = PaymentLinksResourceRestrictions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "completed_sessions" => Deserialize::begin(&mut self.completed_sessions),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { completed_sessions: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { completed_sessions: self.completed_sessions? })
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

    impl ObjectDeser for PaymentLinksResourceRestrictions {
        type Builder = PaymentLinksResourceRestrictionsBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceRestrictions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceRestrictionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "completed_sessions" => {
                        b.completed_sessions = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
