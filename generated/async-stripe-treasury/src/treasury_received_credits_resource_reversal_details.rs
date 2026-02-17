#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceReversalDetails {
    /// Time before which a ReceivedCredit can be reversed.
    pub deadline: Option<stripe_types::Timestamp>,
    /// Set if a ReceivedCredit cannot be reversed.
    pub restricted_reason: Option<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>,
}
#[doc(hidden)]
pub struct TreasuryReceivedCreditsResourceReversalDetailsBuilder {
    deadline: Option<Option<stripe_types::Timestamp>>,
    restricted_reason:
        Option<Option<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>>,
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

    impl Deserialize for TreasuryReceivedCreditsResourceReversalDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceReversalDetails>,
        builder: TreasuryReceivedCreditsResourceReversalDetailsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceReversalDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedCreditsResourceReversalDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditsResourceReversalDetailsBuilder {
        type Out = TreasuryReceivedCreditsResourceReversalDetails;
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryReceivedCreditsResourceReversalDetails {
        type Builder = TreasuryReceivedCreditsResourceReversalDetailsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedCreditsResourceReversalDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryReceivedCreditsResourceReversalDetailsBuilder::deser_default();
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
/// Set if a ReceivedCredit cannot be reversed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    pub fn as_str(&self) -> &str {
        use TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::*;
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

impl std::str::FromStr for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::*;
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
                    "TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
