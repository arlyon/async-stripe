#[derive(Copy, Clone, Debug)]
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
#[doc(hidden)]
pub struct IssuingAuthorizationFraudChallengeBuilder {
    channel: Option<IssuingAuthorizationFraudChallengeChannel>,
    status: Option<IssuingAuthorizationFraudChallengeStatus>,
    undeliverable_reason: Option<Option<IssuingAuthorizationFraudChallengeUndeliverableReason>>,
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
                builder: IssuingAuthorizationFraudChallengeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationFraudChallengeBuilder {
        type Out = IssuingAuthorizationFraudChallenge;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "channel" => Deserialize::begin(&mut self.channel),
                "status" => Deserialize::begin(&mut self.status),
                "undeliverable_reason" => Deserialize::begin(&mut self.undeliverable_reason),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                channel: Deserialize::default(),
                status: Deserialize::default(),
                undeliverable_reason: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(channel), Some(status), Some(undeliverable_reason)) =
                (self.channel, self.status, self.undeliverable_reason)
            else {
                return None;
            };
            Some(Self::Out { channel, status, undeliverable_reason })
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

    impl ObjectDeser for IssuingAuthorizationFraudChallenge {
        type Builder = IssuingAuthorizationFraudChallengeBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFraudChallenge {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationFraudChallengeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "channel" => b.channel = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "undeliverable_reason" => b.undeliverable_reason = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The method by which the fraud challenge was delivered to the cardholder.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationFraudChallengeChannel {
    Sms,
}
impl IssuingAuthorizationFraudChallengeChannel {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationFraudChallengeChannel::*;
        match self {
            Sms => "sms",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFraudChallengeChannel {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFraudChallengeChannel::*;
        match s {
            "sms" => Ok(Sms),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFraudChallengeChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationFraudChallengeChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingAuthorizationFraudChallengeChannel {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationFraudChallengeChannel> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationFraudChallengeChannel::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationFraudChallengeChannel);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFraudChallengeChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationFraudChallengeChannel")
        })
    }
}
/// The status of the fraud challenge.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationFraudChallengeStatus {
    Expired,
    Pending,
    Rejected,
    Undeliverable,
    Verified,
}
impl IssuingAuthorizationFraudChallengeStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationFraudChallengeStatus::*;
        match self {
            Expired => "expired",
            Pending => "pending",
            Rejected => "rejected",
            Undeliverable => "undeliverable",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFraudChallengeStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFraudChallengeStatus::*;
        match s {
            "expired" => Ok(Expired),
            "pending" => Ok(Pending),
            "rejected" => Ok(Rejected),
            "undeliverable" => Ok(Undeliverable),
            "verified" => Ok(Verified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFraudChallengeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationFraudChallengeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingAuthorizationFraudChallengeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationFraudChallengeStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationFraudChallengeStatus::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationFraudChallengeStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFraudChallengeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationFraudChallengeStatus")
        })
    }
}
/// If the challenge is not deliverable, the reason why.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationFraudChallengeUndeliverableReason {
    NoPhoneNumber,
    UnsupportedPhoneNumber,
}
impl IssuingAuthorizationFraudChallengeUndeliverableReason {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationFraudChallengeUndeliverableReason::*;
        match self {
            NoPhoneNumber => "no_phone_number",
            UnsupportedPhoneNumber => "unsupported_phone_number",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFraudChallengeUndeliverableReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFraudChallengeUndeliverableReason::*;
        match s {
            "no_phone_number" => Ok(NoPhoneNumber),
            "unsupported_phone_number" => Ok(UnsupportedPhoneNumber),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<IssuingAuthorizationFraudChallengeUndeliverableReason>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationFraudChallengeUndeliverableReason::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationFraudChallengeUndeliverableReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFraudChallengeUndeliverableReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationFraudChallengeUndeliverableReason",
            )
        })
    }
}
