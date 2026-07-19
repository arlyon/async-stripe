#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    /// A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    pub default_for: Option<Vec<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule:
        Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type:
        Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder {
    custom_mandate_url: Option<Option<String>>,
    default_for:
        Option<Option<Vec<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>>>,
    interval_description: Option<Option<String>>,
    payment_schedule:
        Option<Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>>,
    transaction_type:
        Option<Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
        builder: SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder {
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
            *self.out = Some(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    Invoice,
    Subscription,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    pub fn as_str(&self) -> &str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Payment schedule for the mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Transaction type of the mandate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    pub fn as_str(&self) -> &str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
