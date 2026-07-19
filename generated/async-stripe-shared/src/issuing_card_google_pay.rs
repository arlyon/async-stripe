#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardGooglePay {
    /// Google Pay Eligibility
    pub eligible: bool,
    /// Reason the card is ineligible for Google Pay
    pub ineligible_reason: Option<IssuingCardGooglePayIneligibleReason>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardGooglePay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardGooglePay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardGooglePayBuilder {
    eligible: Option<bool>,
    ineligible_reason: Option<Option<IssuingCardGooglePayIneligibleReason>>,
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

    impl Deserialize for IssuingCardGooglePay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardGooglePay>,
        builder: IssuingCardGooglePayBuilder,
    }

    impl Visitor for Place<IssuingCardGooglePay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardGooglePayBuilder {
                    eligible: Deserialize::default(),
                    ineligible_reason: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eligible" => Deserialize::begin(&mut self.builder.eligible),
                "ineligible_reason" => Deserialize::begin(&mut self.builder.ineligible_reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(eligible), Some(ineligible_reason)) =
                (self.builder.eligible, self.builder.ineligible_reason.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingCardGooglePay { eligible, ineligible_reason });
            Ok(())
        }
    }
};
/// Reason the card is ineligible for Google Pay
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingCardGooglePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardGooglePayIneligibleReason {
    pub fn as_str(&self) -> &str {
        use IssuingCardGooglePayIneligibleReason::*;
        match self {
            MissingAgreement => "missing_agreement",
            MissingCardholderContact => "missing_cardholder_contact",
            UnsupportedRegion => "unsupported_region",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingCardGooglePayIneligibleReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardGooglePayIneligibleReason::*;
        match s {
            "missing_agreement" => Ok(MissingAgreement),
            "missing_cardholder_contact" => Ok(MissingCardholderContact),
            "unsupported_region" => Ok(UnsupportedRegion),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingCardGooglePayIneligibleReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingCardGooglePayIneligibleReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardGooglePayIneligibleReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingCardGooglePayIneligibleReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingCardGooglePayIneligibleReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardGooglePayIneligibleReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardGooglePayIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
