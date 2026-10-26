/// Details about installment payments
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceInstallment {
    /// A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: Option<i64>,
    pub expiry: stripe_payment::ThreeDSecureStandaloneResourceExpiry,
    /// The unit of time for `interval_count`.
    pub interval: ThreeDSecureStandaloneResourceInstallmentInterval,
    /// The minimum number of time intervals between authorizations.
    pub interval_count: u64,
    /// The maximum number of installments.
    pub number: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceInstallment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceInstallment").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceInstallmentBuilder {
    amount: Option<Option<i64>>,
    expiry: Option<stripe_payment::ThreeDSecureStandaloneResourceExpiry>,
    interval: Option<ThreeDSecureStandaloneResourceInstallmentInterval>,
    interval_count: Option<u64>,
    number: Option<i64>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceInstallment {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceInstallment>,
        builder: ThreeDSecureStandaloneResourceInstallmentBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceInstallment> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceInstallmentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceInstallmentBuilder {
        type Out = ThreeDSecureStandaloneResourceInstallment;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "expiry" => Deserialize::begin(&mut self.expiry),
                "interval" => Deserialize::begin(&mut self.interval),
                "interval_count" => Deserialize::begin(&mut self.interval_count),
                "number" => Deserialize::begin(&mut self.number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Some(None),
                expiry: None,
                interval: None,
                interval_count: None,
                number: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(expiry), Some(interval), Some(interval_count), Some(number)) = (
                self.amount,
                self.expiry.take(),
                self.interval.take(),
                self.interval_count,
                self.number,
            ) else {
                return None;
            };
            Some(Self::Out { amount, expiry, interval, interval_count, number })
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceInstallment {
        type Builder = ThreeDSecureStandaloneResourceInstallmentBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceInstallment {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceInstallmentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "expiry" => b.expiry = FromValueOpt::from_value(v),
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "interval_count" => b.interval_count = FromValueOpt::from_value(v),
                    "number" => b.number = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The unit of time for `interval_count`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceInstallmentInterval {
    Day,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceInstallmentInterval {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceInstallmentInterval::*;
        match self {
            Day => "day",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceInstallmentInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceInstallmentInterval::*;
        match s {
            "day" => Ok(Day),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceInstallmentInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureStandaloneResourceInstallmentInterval))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureStandaloneResourceInstallmentInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureStandaloneResourceInstallmentInterval::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureStandaloneResourceInstallmentInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
