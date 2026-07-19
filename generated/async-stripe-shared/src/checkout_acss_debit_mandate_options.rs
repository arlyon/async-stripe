#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutAcssDebitMandateOptions {
    /// A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    /// Returned when the Session is in `setup` mode.
    pub default_for: Option<Vec<CheckoutAcssDebitMandateOptionsDefaultFor>>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: Option<CheckoutAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type: Option<CheckoutAcssDebitMandateOptionsTransactionType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutAcssDebitMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutAcssDebitMandateOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutAcssDebitMandateOptionsBuilder {
    custom_mandate_url: Option<Option<String>>,
    default_for: Option<Option<Vec<CheckoutAcssDebitMandateOptionsDefaultFor>>>,
    interval_description: Option<Option<String>>,
    payment_schedule: Option<Option<CheckoutAcssDebitMandateOptionsPaymentSchedule>>,
    transaction_type: Option<Option<CheckoutAcssDebitMandateOptionsTransactionType>>,
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

    impl Deserialize for CheckoutAcssDebitMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutAcssDebitMandateOptions>,
        builder: CheckoutAcssDebitMandateOptionsBuilder,
    }

    impl Visitor for Place<CheckoutAcssDebitMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutAcssDebitMandateOptionsBuilder {
                    custom_mandate_url: Deserialize::default(),
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
                "custom_mandate_url" => Deserialize::begin(&mut self.builder.custom_mandate_url),
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
                Some(custom_mandate_url),
                Some(default_for),
                Some(interval_description),
                Some(payment_schedule),
                Some(transaction_type),
            ) = (
                self.builder.custom_mandate_url.take(),
                self.builder.default_for.take(),
                self.builder.interval_description.take(),
                self.builder.payment_schedule.take(),
                self.builder.transaction_type.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(CheckoutAcssDebitMandateOptions {
                custom_mandate_url,
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
/// Returned when the Session is in `setup` mode.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(&self) -> &str {
        use CheckoutAcssDebitMandateOptionsDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitMandateOptionsDefaultFor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitMandateOptionsDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutAcssDebitMandateOptionsDefaultFor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CheckoutAcssDebitMandateOptionsDefaultFor))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CheckoutAcssDebitMandateOptionsDefaultFor> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CheckoutAcssDebitMandateOptionsDefaultFor::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Payment schedule for the mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use CheckoutAcssDebitMandateOptionsPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitMandateOptionsPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutAcssDebitMandateOptionsPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CheckoutAcssDebitMandateOptionsPaymentSchedule))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CheckoutAcssDebitMandateOptionsPaymentSchedule> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CheckoutAcssDebitMandateOptionsPaymentSchedule::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Transaction type of the mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutAcssDebitMandateOptionsTransactionType {
    pub fn as_str(&self) -> &str {
        use CheckoutAcssDebitMandateOptionsTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitMandateOptionsTransactionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitMandateOptionsTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutAcssDebitMandateOptionsTransactionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CheckoutAcssDebitMandateOptionsTransactionType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutAcssDebitMandateOptionsTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CheckoutAcssDebitMandateOptionsTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CheckoutAcssDebitMandateOptionsTransactionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CheckoutAcssDebitMandateOptionsTransactionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitMandateOptionsTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
