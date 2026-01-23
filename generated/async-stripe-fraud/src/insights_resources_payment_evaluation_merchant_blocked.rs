/// Details of a merchant_blocked outcome attached to this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationMerchantBlocked {
    /// The reason the payment was blocked by the merchant.
    pub reason: InsightsResourcesPaymentEvaluationMerchantBlockedReason,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationMerchantBlockedBuilder {
    reason: Option<InsightsResourcesPaymentEvaluationMerchantBlockedReason>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationMerchantBlocked {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationMerchantBlocked>,
        builder: InsightsResourcesPaymentEvaluationMerchantBlockedBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationMerchantBlocked> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationMerchantBlockedBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationMerchantBlockedBuilder {
        type Out = InsightsResourcesPaymentEvaluationMerchantBlocked;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reason" => Deserialize::begin(&mut self.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(reason),) = (self.reason.take(),) else {
                return None;
            };
            Some(Self::Out { reason })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationMerchantBlocked {
        type Builder = InsightsResourcesPaymentEvaluationMerchantBlockedBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationMerchantBlocked {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationMerchantBlockedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reason the payment was blocked by the merchant.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    AuthenticationRequired,
    BlockedForFraud,
    InvalidPayment,
    Other,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationMerchantBlockedReason::*;
        match self {
            AuthenticationRequired => "authentication_required",
            BlockedForFraud => "blocked_for_fraud",
            InvalidPayment => "invalid_payment",
            Other => "other",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationMerchantBlockedReason::*;
        match s {
            "authentication_required" => Ok(AuthenticationRequired),
            "blocked_for_fraud" => Ok(BlockedForFraud),
            "invalid_payment" => Ok(InvalidPayment),
            "other" => Ok(Other),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationMerchantBlockedReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationMerchantBlockedReason>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationMerchantBlockedReason::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InsightsResourcesPaymentEvaluationMerchantBlockedReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationMerchantBlockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
