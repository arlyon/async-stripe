/// Money Movement card details attached to this payment.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementCard {
    /// Describes the presence of the customer during the payment.
    pub customer_presence:
        Option<InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence>,
    /// Describes the type of payment.
    pub payment_type: Option<InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType>,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder {
    customer_presence:
        Option<Option<InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence>>,
    payment_type: Option<Option<InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationMoneyMovementCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationMoneyMovementCard>,
        builder: InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationMoneyMovementCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder {
        type Out = InsightsResourcesPaymentEvaluationMoneyMovementCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_presence" => Deserialize::begin(&mut self.customer_presence),
                "payment_type" => Deserialize::begin(&mut self.payment_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { customer_presence: Deserialize::default(), payment_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer_presence), Some(payment_type)) =
                (self.customer_presence.take(), self.payment_type.take())
            else {
                return None;
            };
            Some(Self::Out { customer_presence, payment_type })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationMoneyMovementCard {
        type Builder = InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationMoneyMovementCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationMoneyMovementCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_presence" => b.customer_presence = FromValueOpt::from_value(v),
                    "payment_type" => b.payment_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Describes the presence of the customer during the payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Describes the type of payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    OneOff,
    Recurring,
    SetupOneOff,
    SetupRecurring,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::*;
        match self {
            OneOff => "one_off",
            Recurring => "recurring",
            SetupOneOff => "setup_one_off",
            SetupRecurring => "setup_recurring",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::*;
        match s {
            "one_off" => Ok(OneOff),
            "recurring" => Ok(Recurring),
            "setup_one_off" => Ok(SetupOneOff),
            "setup_recurring" => Ok(SetupRecurring),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
