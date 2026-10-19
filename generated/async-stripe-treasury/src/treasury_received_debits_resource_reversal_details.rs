#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    /// Time before which a ReceivedDebit can be reversed.
    pub deadline: Option<stripe_types::Timestamp>,
    /// Set if a ReceivedDebit can't be reversed.
    pub restricted_reason: Option<stripe_shared::RestrictedReason>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryReceivedDebitsResourceReversalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryReceivedDebitsResourceReversalDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryReceivedDebitsResourceReversalDetailsBuilder {
    deadline: Option<Option<stripe_types::Timestamp>>,
    restricted_reason: Option<Option<stripe_shared::RestrictedReason>>,
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

    impl Deserialize for TreasuryReceivedDebitsResourceReversalDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedDebitsResourceReversalDetails>,
        builder: TreasuryReceivedDebitsResourceReversalDetailsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedDebitsResourceReversalDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedDebitsResourceReversalDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedDebitsResourceReversalDetailsBuilder {
        type Out = TreasuryReceivedDebitsResourceReversalDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "deadline" => Deserialize::begin(&mut self.deadline),
                "restricted_reason" => Deserialize::begin(&mut self.restricted_reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { deadline: Some(None), restricted_reason: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(deadline), Some(restricted_reason)) =
                (self.deadline, self.restricted_reason.take())
            else {
                return None;
            };
            Some(Self::Out { deadline, restricted_reason })
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

    impl ObjectDeser for TreasuryReceivedDebitsResourceReversalDetails {
        type Builder = TreasuryReceivedDebitsResourceReversalDetailsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedDebitsResourceReversalDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryReceivedDebitsResourceReversalDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "deadline" => b.deadline = FromValueOpt::from_value(v),
                    "restricted_reason" => b.restricted_reason = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Set if a ReceivedDebit can't be reversed.
pub type TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason =
    stripe_shared::RestrictedReason;
