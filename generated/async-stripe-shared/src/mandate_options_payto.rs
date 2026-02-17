#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandateOptionsPayto {
    /// Amount that will be collected. It is required when `amount_type` is `fixed`.
    pub amount: Option<i64>,
    /// The type of amount that will be collected.
    /// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
    /// Defaults to `maximum`.
    pub amount_type: Option<MandateOptionsPaytoAmountType>,
    /// Date, in YYYY-MM-DD format, after which payments will not be collected. Defaults to no end date.
    pub end_date: Option<String>,
    /// The periodicity at which payments will be collected. Defaults to `adhoc`.
    pub payment_schedule: Option<MandateOptionsPaytoPaymentSchedule>,
    /// The number of payments that will be made during a payment period.
    /// Defaults to 1 except for when `payment_schedule` is `adhoc`.
    /// In that case, it defaults to no limit.
    pub payments_per_period: Option<i64>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    pub purpose: Option<MandateOptionsPaytoPurpose>,
    /// Date, in YYYY-MM-DD format, from which payments will be collected. Defaults to confirmation time.
    pub start_date: Option<String>,
}
#[doc(hidden)]
pub struct MandateOptionsPaytoBuilder {
    amount: Option<Option<i64>>,
    amount_type: Option<Option<MandateOptionsPaytoAmountType>>,
    end_date: Option<Option<String>>,
    payment_schedule: Option<Option<MandateOptionsPaytoPaymentSchedule>>,
    payments_per_period: Option<Option<i64>>,
    purpose: Option<Option<MandateOptionsPaytoPurpose>>,
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

    impl Deserialize for MandateOptionsPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateOptionsPayto>,
        builder: MandateOptionsPaytoBuilder,
    }

    impl Visitor for Place<MandateOptionsPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandateOptionsPaytoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandateOptionsPaytoBuilder {
        type Out = MandateOptionsPayto;
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

    impl ObjectDeser for MandateOptionsPayto {
        type Builder = MandateOptionsPaytoBuilder;
    }

    impl FromValueOpt for MandateOptionsPayto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandateOptionsPaytoBuilder::deser_default();
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
pub enum MandateOptionsPaytoAmountType {
    Fixed,
    Maximum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateOptionsPaytoAmountType {
    pub fn as_str(&self) -> &str {
        use MandateOptionsPaytoAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateOptionsPaytoAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateOptionsPaytoAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandateOptionsPaytoAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateOptionsPaytoAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateOptionsPaytoAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateOptionsPaytoAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandateOptionsPaytoAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateOptionsPaytoAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsPaytoAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateOptionsPaytoAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateOptionsPaytoAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The periodicity at which payments will be collected. Defaults to `adhoc`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateOptionsPaytoPaymentSchedule {
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
impl MandateOptionsPaytoPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use MandateOptionsPaytoPaymentSchedule::*;
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

impl std::str::FromStr for MandateOptionsPaytoPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateOptionsPaytoPaymentSchedule::*;
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
                    "MandateOptionsPaytoPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateOptionsPaytoPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateOptionsPaytoPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateOptionsPaytoPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandateOptionsPaytoPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateOptionsPaytoPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsPaytoPaymentSchedule::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateOptionsPaytoPaymentSchedule);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateOptionsPaytoPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The purpose for which payments are made. Has a default value based on your merchant category code.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateOptionsPaytoPurpose {
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
impl MandateOptionsPaytoPurpose {
    pub fn as_str(&self) -> &str {
        use MandateOptionsPaytoPurpose::*;
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

impl std::str::FromStr for MandateOptionsPaytoPurpose {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateOptionsPaytoPurpose::*;
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
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandateOptionsPaytoPurpose");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateOptionsPaytoPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateOptionsPaytoPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateOptionsPaytoPurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandateOptionsPaytoPurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateOptionsPaytoPurpose> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsPaytoPurpose::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateOptionsPaytoPurpose);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateOptionsPaytoPurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
