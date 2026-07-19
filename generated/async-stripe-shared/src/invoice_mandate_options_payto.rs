#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceMandateOptionsPayto {
    /// The maximum amount that can be collected in a single invoice.
    /// If you don't specify a maximum, then there is no limit.
    pub amount: Option<i64>,
    /// Only `maximum` is supported.
    pub amount_type: Option<InvoiceMandateOptionsPaytoAmountType>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    pub purpose: Option<InvoiceMandateOptionsPaytoPurpose>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceMandateOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceMandateOptionsPayto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceMandateOptionsPaytoBuilder {
    amount: Option<Option<i64>>,
    amount_type: Option<Option<InvoiceMandateOptionsPaytoAmountType>>,
    purpose: Option<Option<InvoiceMandateOptionsPaytoPurpose>>,
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

    impl Deserialize for InvoiceMandateOptionsPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceMandateOptionsPayto>,
        builder: InvoiceMandateOptionsPaytoBuilder,
    }

    impl Visitor for Place<InvoiceMandateOptionsPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceMandateOptionsPaytoBuilder {
                    amount: Deserialize::default(),
                    amount_type: Deserialize::default(),
                    purpose: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_type" => Deserialize::begin(&mut self.builder.amount_type),
                "purpose" => Deserialize::begin(&mut self.builder.purpose),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(amount_type), Some(purpose)) =
                (self.builder.amount, self.builder.amount_type.take(), self.builder.purpose.take())
            else {
                return Ok(());
            };
            *self.out = Some(InvoiceMandateOptionsPayto { amount, amount_type, purpose });
            Ok(())
        }
    }
};
/// Only `maximum` is supported.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceMandateOptionsPaytoAmountType {
    Fixed,
    Maximum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoiceMandateOptionsPaytoAmountType {
    pub fn as_str(&self) -> &str {
        use InvoiceMandateOptionsPaytoAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoiceMandateOptionsPaytoAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceMandateOptionsPaytoAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoiceMandateOptionsPaytoAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceMandateOptionsPaytoAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoiceMandateOptionsPaytoAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceMandateOptionsPaytoAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoiceMandateOptionsPaytoAmountType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceMandateOptionsPaytoAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InvoiceMandateOptionsPaytoAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InvoiceMandateOptionsPaytoAmountType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceMandateOptionsPaytoAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceMandateOptionsPaytoAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The purpose for which payments are made. Has a default value based on your merchant category code.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceMandateOptionsPaytoPurpose {
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
impl InvoiceMandateOptionsPaytoPurpose {
    pub fn as_str(&self) -> &str {
        use InvoiceMandateOptionsPaytoPurpose::*;
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

impl std::str::FromStr for InvoiceMandateOptionsPaytoPurpose {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceMandateOptionsPaytoPurpose::*;
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
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoiceMandateOptionsPaytoPurpose"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceMandateOptionsPaytoPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoiceMandateOptionsPaytoPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceMandateOptionsPaytoPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoiceMandateOptionsPaytoPurpose)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceMandateOptionsPaytoPurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InvoiceMandateOptionsPaytoPurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InvoiceMandateOptionsPaytoPurpose> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceMandateOptionsPaytoPurpose::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceMandateOptionsPaytoPurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
