#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandatePix {
    /// Determines if the amount includes the IOF tax.
    pub amount_includes_iof: Option<MandatePixAmountIncludesIof>,
    /// Type of amount.
    pub amount_type: Option<MandatePixAmountType>,
    /// Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`.
    pub end_date: Option<String>,
    /// Schedule at which the future payments will be charged.
    pub payment_schedule: Option<MandatePixPaymentSchedule>,
    /// Subscription name displayed to buyers in their bank app.
    pub reference: Option<String>,
    /// Start date of the mandate, in `YYYY-MM-DD`.
    pub start_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandatePix").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct MandatePixBuilder {
    amount_includes_iof: Option<Option<MandatePixAmountIncludesIof>>,
    amount_type: Option<Option<MandatePixAmountType>>,
    end_date: Option<Option<String>>,
    payment_schedule: Option<Option<MandatePixPaymentSchedule>>,
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

    impl Deserialize for MandatePix {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandatePix>,
        builder: MandatePixBuilder,
    }

    impl Visitor for Place<MandatePix> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandatePixBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandatePixBuilder {
        type Out = MandatePix;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_includes_iof" => Deserialize::begin(&mut self.amount_includes_iof),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "reference" => Deserialize::begin(&mut self.reference),
                "start_date" => Deserialize::begin(&mut self.start_date),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_includes_iof: Some(None),
                amount_type: Some(None),
                end_date: Some(None),
                payment_schedule: Some(None),
                reference: Some(None),
                start_date: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_includes_iof),
                Some(amount_type),
                Some(end_date),
                Some(payment_schedule),
                Some(reference),
                Some(start_date),
            ) = (
                self.amount_includes_iof.take(),
                self.amount_type.take(),
                self.end_date.take(),
                self.payment_schedule.take(),
                self.reference.take(),
                self.start_date.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount_includes_iof,
                amount_type,
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

    impl ObjectDeser for MandatePix {
        type Builder = MandatePixBuilder;
    }

    impl FromValueOpt for MandatePix {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandatePixBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_includes_iof" => b.amount_includes_iof = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
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
pub enum MandatePixAmountIncludesIof {
    Always,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePixAmountIncludesIof {
    pub fn as_str(&self) -> &str {
        use MandatePixAmountIncludesIof::*;
        match self {
            Always => "always",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandatePixAmountIncludesIof {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePixAmountIncludesIof::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandatePixAmountIncludesIof"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandatePixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePixAmountIncludesIof {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandatePixAmountIncludesIof)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandatePixAmountIncludesIof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePixAmountIncludesIof {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePixAmountIncludesIof> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePixAmountIncludesIof::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePixAmountIncludesIof);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePixAmountIncludesIof {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Type of amount.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandatePixAmountType {
    Fixed,
    Maximum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePixAmountType {
    pub fn as_str(&self) -> &str {
        use MandatePixAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandatePixAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePixAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandatePixAmountType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePixAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandatePixAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePixAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandatePixAmountType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandatePixAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePixAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePixAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePixAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePixAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePixAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Schedule at which the future payments will be charged.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandatePixPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePixPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use MandatePixPaymentSchedule::*;
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

impl std::str::FromStr for MandatePixPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePixPaymentSchedule::*;
        match s {
            "halfyearly" => Ok(Halfyearly),
            "monthly" => Ok(Monthly),
            "quarterly" => Ok(Quarterly),
            "weekly" => Ok(Weekly),
            "yearly" => Ok(Yearly),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandatePixPaymentSchedule");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandatePixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandatePixPaymentSchedule)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandatePixPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePixPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePixPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePixPaymentSchedule::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePixPaymentSchedule);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePixPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
