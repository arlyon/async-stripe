#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCompletedSessions {
    /// The current number of checkout sessions that have been completed on the payment link which count towards the `completed_sessions` restriction to be met.
    pub count: u64,
    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCompletedSessions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCompletedSessions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCompletedSessionsBuilder {
    count: Option<u64>,
    limit: Option<i64>,
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

    impl Deserialize for PaymentLinksResourceCompletedSessions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCompletedSessions>,
        builder: PaymentLinksResourceCompletedSessionsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCompletedSessions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCompletedSessionsBuilder {
                    count: Deserialize::default(),
                    limit: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "count" => Deserialize::begin(&mut self.builder.count),
                "limit" => Deserialize::begin(&mut self.builder.limit),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(count), Some(limit)) = (self.builder.count, self.builder.limit) else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceCompletedSessions { count, limit });
            Ok(())
        }
    }
};
