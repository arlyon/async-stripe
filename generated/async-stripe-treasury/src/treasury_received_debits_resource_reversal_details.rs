#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    /// Time before which a ReceivedDebit can be reversed.
    pub deadline: Option<stripe_types::Timestamp>,
    /// Set if a ReceivedDebit can't be reversed.
    pub restricted_reason: Option<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>,
}
#[doc(hidden)]
pub struct TreasuryReceivedDebitsResourceReversalDetailsBuilder {
    deadline: Option<Option<stripe_types::Timestamp>>,
    restricted_reason:
        Option<Option<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>>,
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
            Self { deadline: Deserialize::default(), restricted_reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(deadline), Some(restricted_reason)) = (self.deadline, self.restricted_reason)
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
            *self.out = self.builder.take_out();
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}
impl TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::*;
        match self {
            AlreadyReversed => "already_reversed",
            DeadlinePassed => "deadline_passed",
            NetworkRestricted => "network_restricted",
            Other => "other",
            SourceFlowRestricted => "source_flow_restricted",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::*;
        match s {
            "already_reversed" => Ok(AlreadyReversed),
            "deadline_passed" => Ok(DeadlinePassed),
            "network_restricted" => Ok(NetworkRestricted),
            "other" => Ok(Other),
            "source_flow_restricted" => Ok(SourceFlowRestricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason",
            )
        })
    }
}
