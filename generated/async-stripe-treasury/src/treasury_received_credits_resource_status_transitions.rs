#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    /// Timestamp describing when the CreditReversal changed status to `posted`
    pub posted_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct TreasuryReceivedCreditsResourceStatusTransitionsBuilder {
    posted_at: Option<Option<stripe_types::Timestamp>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedCreditsResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceStatusTransitions>,
        builder: TreasuryReceivedCreditsResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedCreditsResourceStatusTransitionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditsResourceStatusTransitionsBuilder {
        type Out = TreasuryReceivedCreditsResourceStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "posted_at" => Deserialize::begin(&mut self.posted_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { posted_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(posted_at),) = (self.posted_at,) else {
                return None;
            };
            Some(Self::Out { posted_at })
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

    impl ObjectDeser for TreasuryReceivedCreditsResourceStatusTransitions {
        type Builder = TreasuryReceivedCreditsResourceStatusTransitionsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedCreditsResourceStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryReceivedCreditsResourceStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "posted_at" => b.posted_at = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
