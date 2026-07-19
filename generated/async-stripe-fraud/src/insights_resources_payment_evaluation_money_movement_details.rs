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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: InsightsResourcesPaymentEvaluationMoneyMovementDetailsBuilder {
                    card: Deserialize::default(),
                    money_movement_type: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => Deserialize::begin(&mut self.builder.card),
                "money_movement_type" => Deserialize::begin(&mut self.builder.money_movement_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(card), Some(money_movement_type)) =
                (self.builder.card.take(), self.builder.money_movement_type.take())
            else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationMoneyMovementDetails {
                card,
                money_movement_type,
            });
            Ok(())
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
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType"
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
impl stripe_miniserde::Deserialize
    for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
