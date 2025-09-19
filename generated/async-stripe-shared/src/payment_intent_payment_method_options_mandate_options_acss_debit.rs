#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    /// A URL for custom mandate text
    pub custom_mandate_url: Option<String>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule:
        Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type:
        Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
}
#[doc(hidden)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder {
    custom_mandate_url: Option<Option<String>>,
    interval_description: Option<Option<String>>,
    payment_schedule:
        Option<Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>>,
    transaction_type:
        Option<Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>>,
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

    impl Deserialize for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
        builder: PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder {
        type Out = PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_mandate_url" => Deserialize::begin(&mut self.custom_mandate_url),
                "interval_description" => Deserialize::begin(&mut self.interval_description),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "transaction_type" => Deserialize::begin(&mut self.transaction_type),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                custom_mandate_url: Deserialize::default(),
                interval_description: Deserialize::default(),
                payment_schedule: Deserialize::default(),
                transaction_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(custom_mandate_url),
                Some(interval_description),
                Some(payment_schedule),
                Some(transaction_type),
            ) = (
                self.custom_mandate_url.take(),
                self.interval_description.take(),
                self.payment_schedule,
                self.transaction_type,
            )
            else {
                return None;
            };
            Some(Self::Out {
                custom_mandate_url,
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        type Builder = PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder;
    }

    impl FromValueOpt for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom_mandate_url" => b.custom_mandate_url = FromValueOpt::from_value(v),
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
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}
impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule"))
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}
impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType"))
    }
}
