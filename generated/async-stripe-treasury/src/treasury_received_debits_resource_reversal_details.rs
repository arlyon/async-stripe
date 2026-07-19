#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    /// Time before which a ReceivedDebit can be reversed.
    pub deadline: Option<stripe_types::Timestamp>,
    /// Set if a ReceivedDebit can't be reversed.
    pub restricted_reason: Option<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>,
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
    restricted_reason:
        Option<Option<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>>,
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
                builder: TreasuryReceivedDebitsResourceReversalDetailsBuilder {
                    deadline: Deserialize::default(),
                    restricted_reason: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "deadline" => Deserialize::begin(&mut self.builder.deadline),
                "restricted_reason" => Deserialize::begin(&mut self.builder.restricted_reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(deadline), Some(restricted_reason)) =
                (self.builder.deadline, self.builder.restricted_reason.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(TreasuryReceivedDebitsResourceReversalDetails { deadline, restricted_reason });
            Ok(())
        }
    }
};
/// Set if a ReceivedDebit can't be reversed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    pub fn as_str(&self) -> &str {
        use TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::*;
        match self {
            AlreadyReversed => "already_reversed",
            DeadlinePassed => "deadline_passed",
            NetworkRestricted => "network_restricted",
            Other => "other",
            SourceFlowRestricted => "source_flow_restricted",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::*;
        match s {
            "already_reversed" => Ok(AlreadyReversed),
            "deadline_passed" => Ok(DeadlinePassed),
            "network_restricted" => Ok(NetworkRestricted),
            "other" => Ok(Other),
            "source_flow_restricted" => Ok(SourceFlowRestricted),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
