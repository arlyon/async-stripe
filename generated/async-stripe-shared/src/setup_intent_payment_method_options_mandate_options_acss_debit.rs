#[derive(Clone, Debug)]
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
                builder:
                    SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder {
        type Out = SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_mandate_url" => Deserialize::begin(&mut self.custom_mandate_url),
                "default_for" => Deserialize::begin(&mut self.default_for),
                "interval_description" => Deserialize::begin(&mut self.interval_description),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "transaction_type" => Deserialize::begin(&mut self.transaction_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                custom_mandate_url: Deserialize::default(),
                default_for: Deserialize::default(),
                interval_description: Deserialize::default(),
                payment_schedule: Deserialize::default(),
                transaction_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(custom_mandate_url),
                Some(default_for),
                Some(interval_description),
                Some(payment_schedule),
                Some(transaction_type),
            ) = (
                self.custom_mandate_url.take(),
                self.default_for.take(),
                self.interval_description.take(),
                self.payment_schedule.take(),
                self.transaction_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                custom_mandate_url,
                default_for,
                interval_description,
                payment_schedule,
                transaction_type,
            })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        type Builder = SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom_mandate_url" => b.custom_mandate_url = FromValueOpt::from_value(v),
                    "default_for" => b.default_for = FromValueOpt::from_value(v),
                    "interval_description" => b.interval_description = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
                    "transaction_type" => b.transaction_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor
);
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

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
);
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

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
);
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
