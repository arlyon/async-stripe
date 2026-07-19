#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFraudChallenge {
    /// The method by which the fraud challenge was delivered to the cardholder.
    pub channel: IssuingAuthorizationFraudChallengeChannel,
    /// The status of the fraud challenge.
    pub status: IssuingAuthorizationFraudChallengeStatus,
    /// If the challenge is not deliverable, the reason why.
    pub undeliverable_reason: Option<IssuingAuthorizationFraudChallengeUndeliverableReason>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFraudChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationFraudChallenge").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationFraudChallengeBuilder {
    channel: Option<IssuingAuthorizationFraudChallengeChannel>,
    status: Option<IssuingAuthorizationFraudChallengeStatus>,
    undeliverable_reason: Option<Option<IssuingAuthorizationFraudChallengeUndeliverableReason>>,
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

    impl Deserialize for IssuingAuthorizationFraudChallenge {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFraudChallenge>,
        builder: IssuingAuthorizationFraudChallengeBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFraudChallenge> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFraudChallengeBuilder {
                    channel: Deserialize::default(),
                    status: Deserialize::default(),
                    undeliverable_reason: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "channel" => Deserialize::begin(&mut self.builder.channel),
                "status" => Deserialize::begin(&mut self.builder.status),
                "undeliverable_reason" => {
                    Deserialize::begin(&mut self.builder.undeliverable_reason)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(channel), Some(status), Some(undeliverable_reason)) = (
                self.builder.channel.take(),
                self.builder.status.take(),
                self.builder.undeliverable_reason.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(IssuingAuthorizationFraudChallenge { channel, status, undeliverable_reason });
            Ok(())
        }
    }
};
/// The method by which the fraud challenge was delivered to the cardholder.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationFraudChallengeChannel {
    Sms,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationFraudChallengeChannel {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationFraudChallengeChannel::*;
        match self {
            Sms => "sms",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFraudChallengeChannel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFraudChallengeChannel::*;
        match s {
            "sms" => Ok(Sms),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationFraudChallengeChannel"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFraudChallengeChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationFraudChallengeChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFraudChallengeChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationFraudChallengeChannel))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationFraudChallengeChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingAuthorizationFraudChallengeChannel {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingAuthorizationFraudChallengeChannel> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingAuthorizationFraudChallengeChannel::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFraudChallengeChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The status of the fraud challenge.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationFraudChallengeStatus {
    Expired,
    Pending,
    Rejected,
    Undeliverable,
    Verified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationFraudChallengeStatus {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationFraudChallengeStatus::*;
        match self {
            Expired => "expired",
            Pending => "pending",
            Rejected => "rejected",
            Undeliverable => "undeliverable",
            Verified => "verified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFraudChallengeStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFraudChallengeStatus::*;
        match s {
            "expired" => Ok(Expired),
            "pending" => Ok(Pending),
            "rejected" => Ok(Rejected),
            "undeliverable" => Ok(Undeliverable),
            "verified" => Ok(Verified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationFraudChallengeStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFraudChallengeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationFraudChallengeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFraudChallengeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationFraudChallengeStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationFraudChallengeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingAuthorizationFraudChallengeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingAuthorizationFraudChallengeStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationFraudChallengeStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFraudChallengeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// If the challenge is not deliverable, the reason why.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationFraudChallengeUndeliverableReason {
    NoPhoneNumber,
    UnsupportedPhoneNumber,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationFraudChallengeUndeliverableReason {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationFraudChallengeUndeliverableReason::*;
        match self {
            NoPhoneNumber => "no_phone_number",
            UnsupportedPhoneNumber => "unsupported_phone_number",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFraudChallengeUndeliverableReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFraudChallengeUndeliverableReason::*;
        match s {
            "no_phone_number" => Ok(NoPhoneNumber),
            "unsupported_phone_number" => Ok(UnsupportedPhoneNumber),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationFraudChallengeUndeliverableReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationFraudChallengeUndeliverableReason))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<IssuingAuthorizationFraudChallengeUndeliverableReason>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationFraudChallengeUndeliverableReason::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
