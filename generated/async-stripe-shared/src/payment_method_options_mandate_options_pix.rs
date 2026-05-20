#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsMandateOptionsPix {
    /// Amount to be charged for future payments.
    pub amount: Option<i64>,
    /// Determines if the amount includes the IOF tax.
    pub amount_includes_iof: Option<PaymentMethodOptionsMandateOptionsPixAmountIncludesIof>,
    /// Type of amount.
    pub amount_type: Option<PaymentMethodOptionsMandateOptionsPixAmountType>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub currency: Option<stripe_types::Currency>,
    /// Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`.
    pub end_date: Option<String>,
    /// Schedule at which the future payments will be charged.
    pub payment_schedule: Option<PaymentMethodOptionsMandateOptionsPixPaymentSchedule>,
    /// Subscription name displayed to buyers in their bank app.
    pub reference: Option<String>,
    /// Start date of the mandate, in `YYYY-MM-DD`.
    pub start_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsMandateOptionsPix").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsMandateOptionsPixBuilder {
    amount: Option<Option<i64>>,
    amount_includes_iof: Option<Option<PaymentMethodOptionsMandateOptionsPixAmountIncludesIof>>,
    amount_type: Option<Option<PaymentMethodOptionsMandateOptionsPixAmountType>>,
    currency: Option<Option<stripe_types::Currency>>,
    end_date: Option<Option<String>>,
    payment_schedule: Option<Option<PaymentMethodOptionsMandateOptionsPixPaymentSchedule>>,
    reference: Option<Option<String>>,
    start_date: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodOptionsMandateOptionsPix {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsMandateOptionsPix>,
        builder: PaymentMethodOptionsMandateOptionsPixBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsMandateOptionsPix> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsMandateOptionsPixBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsMandateOptionsPixBuilder {
        type Out = PaymentMethodOptionsMandateOptionsPix;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_includes_iof" => Deserialize::begin(&mut self.amount_includes_iof),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "currency" => Deserialize::begin(&mut self.currency),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "reference" => Deserialize::begin(&mut self.reference),
                "start_date" => Deserialize::begin(&mut self.start_date),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Some(None),
                amount_includes_iof: Some(None),
                amount_type: Some(None),
                currency: Some(None),
                end_date: Some(None),
                payment_schedule: Some(None),
                reference: Some(None),
                start_date: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_includes_iof),
                Some(amount_type),
                Some(currency),
                Some(end_date),
                Some(payment_schedule),
                Some(reference),
                Some(start_date),
            ) = (
                self.amount,
                self.amount_includes_iof.take(),
                self.amount_type.take(),
                self.currency.take(),
                self.end_date.take(),
                self.payment_schedule.take(),
                self.reference.take(),
                self.start_date.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_includes_iof,
                amount_type,
                currency,
                end_date,
                payment_schedule,
                reference,
                start_date,
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

    impl ObjectDeser for PaymentMethodOptionsMandateOptionsPix {
        type Builder = PaymentMethodOptionsMandateOptionsPixBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsMandateOptionsPix {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsMandateOptionsPixBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_includes_iof" => b.amount_includes_iof = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "start_date" => b.start_date = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines if the amount includes the IOF tax.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    Always,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsMandateOptionsPixAmountIncludesIof::*;
        match self {
            Always => "always",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsMandateOptionsPixAmountIncludesIof::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsMandateOptionsPixAmountIncludesIof"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodOptionsMandateOptionsPixAmountIncludesIof))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentMethodOptionsMandateOptionsPixAmountIncludesIof>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsMandateOptionsPixAmountIncludesIof::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsMandateOptionsPixAmountIncludesIof);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Type of amount.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsMandateOptionsPixAmountType {
    Fixed,
    Maximum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsMandateOptionsPixAmountType {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsMandateOptionsPixAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsMandateOptionsPixAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsMandateOptionsPixAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsMandateOptionsPixAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsMandateOptionsPixAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsPixAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsPixAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodOptionsMandateOptionsPixAmountType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsMandateOptionsPixAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsMandateOptionsPixAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsMandateOptionsPixAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodOptionsMandateOptionsPixAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsMandateOptionsPixAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsMandateOptionsPixAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Schedule at which the future payments will be charged.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsMandateOptionsPixPaymentSchedule::*;
        match self {
            Halfyearly => "halfyearly",
            Monthly => "monthly",
            Quarterly => "quarterly",
            Weekly => "weekly",
            Yearly => "yearly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsMandateOptionsPixPaymentSchedule::*;
        match s {
            "halfyearly" => Ok(Halfyearly),
            "monthly" => Ok(Monthly),
            "quarterly" => Ok(Quarterly),
            "weekly" => Ok(Weekly),
            "yearly" => Ok(Yearly),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsMandateOptionsPixPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodOptionsMandateOptionsPixPaymentSchedule))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsMandateOptionsPixPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsMandateOptionsPixPaymentSchedule::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsMandateOptionsPixPaymentSchedule);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
