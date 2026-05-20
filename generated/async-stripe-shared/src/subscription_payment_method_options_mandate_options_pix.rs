#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPaymentMethodOptionsMandateOptionsPix {
    /// Amount to be charged for future payments.
    pub amount: Option<i64>,
    /// Determines if the amount includes the IOF tax.
    pub amount_includes_iof:
        Option<SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof>,
    /// Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`.
    pub end_date: Option<String>,
    /// Schedule at which the future payments will be charged.
    pub payment_schedule: Option<SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsMandateOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsMandateOptionsPix").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder {
    amount: Option<Option<i64>>,
    amount_includes_iof:
        Option<Option<SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof>>,
    end_date: Option<Option<String>>,
    payment_schedule:
        Option<Option<SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule>>,
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

    impl Deserialize for SubscriptionPaymentMethodOptionsMandateOptionsPix {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionPaymentMethodOptionsMandateOptionsPix>,
        builder: SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder,
    }

    impl Visitor for Place<SubscriptionPaymentMethodOptionsMandateOptionsPix> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder {
        type Out = SubscriptionPaymentMethodOptionsMandateOptionsPix;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_includes_iof" => Deserialize::begin(&mut self.amount_includes_iof),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Some(None),
                amount_includes_iof: Some(None),
                end_date: Some(None),
                payment_schedule: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(amount_includes_iof), Some(end_date), Some(payment_schedule)) = (
                self.amount,
                self.amount_includes_iof.take(),
                self.end_date.take(),
                self.payment_schedule.take(),
            ) else {
                return None;
            };
            Some(Self::Out { amount, amount_includes_iof, end_date, payment_schedule })
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

    impl ObjectDeser for SubscriptionPaymentMethodOptionsMandateOptionsPix {
        type Builder = SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder;
    }

    impl FromValueOpt for SubscriptionPaymentMethodOptionsMandateOptionsPix {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_includes_iof" => b.amount_includes_iof = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
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
pub enum SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    Always,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    pub fn as_str(&self) -> &str {
        use SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::*;
        match self {
            Always => "always",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Schedule at which the future payments will be charged.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::*;
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

impl std::str::FromStr for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::*;
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
                    "SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
