#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardGooglePay {
    /// Google Pay Eligibility
    pub eligible: bool,
    /// Reason the card is ineligible for Google Pay
    pub ineligible_reason: Option<IssuingCardGooglePayIneligibleReason>,
}
#[doc(hidden)]
pub struct IssuingCardGooglePayBuilder {
    eligible: Option<bool>,
    ineligible_reason: Option<Option<IssuingCardGooglePayIneligibleReason>>,
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
                builder: IssuingCardGooglePayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardGooglePayBuilder {
        type Out = IssuingCardGooglePay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eligible" => Deserialize::begin(&mut self.eligible),
                "ineligible_reason" => Deserialize::begin(&mut self.ineligible_reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { eligible: Deserialize::default(), ineligible_reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(eligible), Some(ineligible_reason)) =
                (self.eligible, self.ineligible_reason.take())
            else {
                return None;
            };
            Some(Self::Out { eligible, ineligible_reason })
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

    impl ObjectDeser for IssuingCardGooglePay {
        type Builder = IssuingCardGooglePayBuilder;
    }

    impl FromValueOpt for IssuingCardGooglePay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardGooglePayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "eligible" => b.eligible = FromValueOpt::from_value(v),
                    "ineligible_reason" => b.ineligible_reason = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingCardGooglePayIneligibleReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardGooglePayIneligibleReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardGooglePayIneligibleReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardGooglePayIneligibleReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardGooglePayIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
