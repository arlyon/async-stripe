#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandateAcssDebit {
    /// List of Stripe products where this mandate can be selected automatically.
    pub default_for: Option<Vec<MandateAcssDebitDefaultFor>>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: MandateAcssDebitPaymentSchedule,
    /// Transaction type of the mandate.
    pub transaction_type: MandateAcssDebitTransactionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandateAcssDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct MandateAcssDebitBuilder {
    default_for: Option<Option<Vec<MandateAcssDebitDefaultFor>>>,
    interval_description: Option<Option<String>>,
    payment_schedule: Option<MandateAcssDebitPaymentSchedule>,
    transaction_type: Option<MandateAcssDebitTransactionType>,
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

    impl Deserialize for MandateAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateAcssDebit>,
        builder: MandateAcssDebitBuilder,
    }

    impl Visitor for Place<MandateAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandateAcssDebitBuilder {
                    default_for: Deserialize::default(),
                    interval_description: Deserialize::default(),
                    payment_schedule: Deserialize::default(),
                    transaction_type: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_for" => Deserialize::begin(&mut self.builder.default_for),
                "interval_description" => {
                    Deserialize::begin(&mut self.builder.interval_description)
                }
                "payment_schedule" => Deserialize::begin(&mut self.builder.payment_schedule),
                "transaction_type" => Deserialize::begin(&mut self.builder.transaction_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(default_for),
                Some(interval_description),
                Some(payment_schedule),
                Some(transaction_type),
            ) = (
                self.builder.default_for.take(),
                self.builder.interval_description.take(),
                self.builder.payment_schedule.take(),
                self.builder.transaction_type.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(MandateAcssDebit {
                default_for,
                interval_description,
                payment_schedule,
                transaction_type,
            });
            Ok(())
        }
    }
};
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateAcssDebitDefaultFor {
    Invoice,
    Subscription,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateAcssDebitDefaultFor {
    pub fn as_str(&self) -> &str {
        use MandateAcssDebitDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateAcssDebitDefaultFor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateAcssDebitDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandateAcssDebitDefaultFor");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandateAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandateAcssDebitDefaultFor)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateAcssDebitDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for MandateAcssDebitDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<MandateAcssDebitDefaultFor> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateAcssDebitDefaultFor::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateAcssDebitDefaultFor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Payment schedule for the mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateAcssDebitPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use MandateAcssDebitPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateAcssDebitPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateAcssDebitPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandateAcssDebitPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandateAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandateAcssDebitPaymentSchedule)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateAcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for MandateAcssDebitPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<MandateAcssDebitPaymentSchedule> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateAcssDebitPaymentSchedule::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateAcssDebitPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Transaction type of the mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateAcssDebitTransactionType {
    Business,
    Personal,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateAcssDebitTransactionType {
    pub fn as_str(&self) -> &str {
        use MandateAcssDebitTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateAcssDebitTransactionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateAcssDebitTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandateAcssDebitTransactionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandateAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandateAcssDebitTransactionType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateAcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for MandateAcssDebitTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<MandateAcssDebitTransactionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateAcssDebitTransactionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateAcssDebitTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
