#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCompletedSessions {
    /// The current number of checkout sessions that have been completed on the payment link which count towards the `completed_sessions` restriction to be met.
    pub count: u64,
    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}
#[doc(hidden)]
pub struct PaymentLinksResourceCompletedSessionsBuilder {
    count: Option<u64>,
    limit: Option<i64>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentLinksResourceCompletedSessionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCompletedSessionsBuilder {
        type Out = PaymentLinksResourceCompletedSessions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "count" => Deserialize::begin(&mut self.count),
                "limit" => Deserialize::begin(&mut self.limit),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { count: Deserialize::default(), limit: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { count: self.count?, limit: self.limit? })
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

    impl ObjectDeser for PaymentLinksResourceCompletedSessions {
        type Builder = PaymentLinksResourceCompletedSessionsBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCompletedSessions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCompletedSessionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "count" => b.count = Some(FromValueOpt::from_value(v)?),
                    "limit" => b.limit = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
