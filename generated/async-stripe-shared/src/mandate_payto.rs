#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandatePayto {
    /// Amount that will be collected. It is required when `amount_type` is `fixed`.
    pub amount: Option<i64>,
    /// The type of amount that will be collected.
    /// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
    /// Defaults to `maximum`.
    pub amount_type: MandatePaytoAmountType,
    /// Date, in YYYY-MM-DD format, after which payments will not be collected. Defaults to no end date.
    pub end_date: Option<String>,
    /// The periodicity at which payments will be collected. Defaults to `adhoc`.
    pub payment_schedule: MandatePaytoPaymentSchedule,
    /// The number of payments that will be made during a payment period.
    /// Defaults to 1 except for when `payment_schedule` is `adhoc`.
    /// In that case, it defaults to no limit.
    pub payments_per_period: Option<i64>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    pub purpose: Option<MandatePaytoPurpose>,
    /// Date, in YYYY-MM-DD format, from which payments will be collected. Defaults to confirmation time.
    pub start_date: Option<String>,
}
#[doc(hidden)]
pub struct MandatePaytoBuilder {
    amount: Option<Option<i64>>,
    amount_type: Option<MandatePaytoAmountType>,
    end_date: Option<Option<String>>,
    payment_schedule: Option<MandatePaytoPaymentSchedule>,
    payments_per_period: Option<Option<i64>>,
    purpose: Option<Option<MandatePaytoPurpose>>,
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

    impl Deserialize for MandatePayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandatePayto>,
        builder: MandatePaytoBuilder,
    }

    impl Visitor for Place<MandatePayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandatePaytoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandatePaytoBuilder {
        type Out = MandatePayto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "payments_per_period" => Deserialize::begin(&mut self.payments_per_period),
                "purpose" => Deserialize::begin(&mut self.purpose),
                "start_date" => Deserialize::begin(&mut self.start_date),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_type: Deserialize::default(),
                end_date: Deserialize::default(),
                payment_schedule: Deserialize::default(),
                payments_per_period: Deserialize::default(),
                purpose: Deserialize::default(),
                start_date: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_type),
                Some(end_date),
                Some(payment_schedule),
                Some(payments_per_period),
                Some(purpose),
                Some(start_date),
            ) = (
                self.amount,
                self.amount_type.take(),
                self.end_date.take(),
                self.payment_schedule.take(),
                self.payments_per_period,
                self.purpose.take(),
                self.start_date.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_type,
                end_date,
                payment_schedule,
                payments_per_period,
                purpose,
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

    impl ObjectDeser for MandatePayto {
        type Builder = MandatePaytoBuilder;
    }

    impl FromValueOpt for MandatePayto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandatePaytoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
                    "payments_per_period" => b.payments_per_period = FromValueOpt::from_value(v),
                    "purpose" => b.purpose = FromValueOpt::from_value(v),
                    "start_date" => b.start_date = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of amount that will be collected.
/// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
/// Defaults to `maximum`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandatePaytoAmountType {
    Fixed,
    Maximum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePaytoAmountType {
    pub fn as_str(&self) -> &str {
        use MandatePaytoAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandatePaytoAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePaytoAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandatePaytoAmountType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePaytoAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandatePaytoAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandatePaytoAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePaytoAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePaytoAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePaytoAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePaytoAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePaytoAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The periodicity at which payments will be collected. Defaults to `adhoc`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandatePaytoPaymentSchedule {
    Adhoc,
    Annual,
    Daily,
    Fortnightly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Weekly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePaytoPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use MandatePaytoPaymentSchedule::*;
        match self {
            Adhoc => "adhoc",
            Annual => "annual",
            Daily => "daily",
            Fortnightly => "fortnightly",
            Monthly => "monthly",
            Quarterly => "quarterly",
            SemiAnnual => "semi_annual",
            Weekly => "weekly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandatePaytoPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePaytoPaymentSchedule::*;
        match s {
            "adhoc" => Ok(Adhoc),
            "annual" => Ok(Annual),
            "daily" => Ok(Daily),
            "fortnightly" => Ok(Fortnightly),
            "monthly" => Ok(Monthly),
            "quarterly" => Ok(Quarterly),
            "semi_annual" => Ok(SemiAnnual),
            "weekly" => Ok(Weekly),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandatePaytoPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePaytoPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandatePaytoPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandatePaytoPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePaytoPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePaytoPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePaytoPaymentSchedule::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePaytoPaymentSchedule);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePaytoPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The purpose for which payments are made. Has a default value based on your merchant category code.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandatePaytoPurpose {
    DependantSupport,
    Government,
    Loan,
    Mortgage,
    Other,
    Pension,
    Personal,
    Retail,
    Salary,
    Tax,
    Utility,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePaytoPurpose {
    pub fn as_str(&self) -> &str {
        use MandatePaytoPurpose::*;
        match self {
            DependantSupport => "dependant_support",
            Government => "government",
            Loan => "loan",
            Mortgage => "mortgage",
            Other => "other",
            Pension => "pension",
            Personal => "personal",
            Retail => "retail",
            Salary => "salary",
            Tax => "tax",
            Utility => "utility",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandatePaytoPurpose {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePaytoPurpose::*;
        match s {
            "dependant_support" => Ok(DependantSupport),
            "government" => Ok(Government),
            "loan" => Ok(Loan),
            "mortgage" => Ok(Mortgage),
            "other" => Ok(Other),
            "pension" => Ok(Pension),
            "personal" => Ok(Personal),
            "retail" => Ok(Retail),
            "salary" => Ok(Salary),
            "tax" => Ok(Tax),
            "utility" => Ok(Utility),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandatePaytoPurpose");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePaytoPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandatePaytoPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandatePaytoPurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePaytoPurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePaytoPurpose> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePaytoPurpose::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePaytoPurpose);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePaytoPurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
