#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceRestrictions {
    pub completed_sessions: stripe_shared::PaymentLinksResourceCompletedSessions,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceRestrictions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceRestrictionsBuilder {
    completed_sessions: Option<stripe_shared::PaymentLinksResourceCompletedSessions>,
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
                builder: PaymentLinksResourceRestrictionsBuilder {
                    completed_sessions: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "completed_sessions" => Deserialize::begin(&mut self.builder.completed_sessions),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(completed_sessions),) = (self.builder.completed_sessions,) else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceRestrictions { completed_sessions });
            Ok(())
        }
    }
};
