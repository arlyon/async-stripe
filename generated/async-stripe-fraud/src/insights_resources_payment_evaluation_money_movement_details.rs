/// Money Movement details attached to this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementDetails {
    /// Describes card money movement details for the payment evaluation.
    pub card: Option<stripe_fraud::InsightsResourcesPaymentEvaluationMoneyMovementCard>,
    /// Describes the type of money movement. Currently only `card` is supported.
    pub money_movement_type:
        InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationMoneyMovementDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder {
    card: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationMoneyMovementCard>>,
    money_movement_type:
        Option<InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationMoneyMovementDetails>,
        builder: InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationMoneyMovementDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder {
        type Out = InsightsResourcesPaymentEvaluationMoneyMovementDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.card),
                "money_movement_type" => Deserialize::begin(&mut self.money_movement_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: Some(None), money_movement_type: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card), Some(money_movement_type)) =
                (self.card.take(), self.money_movement_type.take())
            else {
                return None;
            };
            Some(Self::Out { card, money_movement_type })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
        type Builder = InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationMoneyMovementDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Describes the type of money movement. Currently only `card` is supported.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    Card,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::*;
        match self {
            Card => "card",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::*;
        match s {
            "card" => Ok(Card),
            v => {
                stripe_types::warn_unknown_enum_value(
                    v,
                    "InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType",
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = {
            let Ok(v) =
                InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::from_str(
                    s,
                );
            Some(v)
        };
        Ok(())
    }
}

impl stripe_types::miniserde_helpers::FromValueOpt
    for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType
{
    fn from_value(v: miniserde::json::Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        let Ok(v) = Self::from_str(&s);
        Ok(v)
    }
}
