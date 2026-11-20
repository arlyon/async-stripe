#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance {
    /// Amount of the insurance.
pub amount: Option<i64>,
    /// Currency for the insurance price.
pub currency: Option<stripe_types::Currency>,
    /// Name of the insurance company.
pub insurance_company_name: Option<String>,
    /// Type of insurance.
pub insurance_type: Option<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceBuilder {
    amount: Option<Option<i64>>,
currency: Option<Option<stripe_types::Currency>>,
insurance_company_name: Option<Option<String>>,
insurance_type: Option<Option<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType>>,

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

    impl Deserialize
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance,
        >,
        builder:
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceBuilder
    {
        type Out = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "insurance_company_name" => Deserialize::begin(&mut self.insurance_company_name),
                "insurance_type" => Deserialize::begin(&mut self.insurance_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Some(None),
                currency: Some(None),
                insurance_company_name: Some(None),
                insurance_type: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(currency), Some(insurance_company_name), Some(insurance_type)) = (
                self.amount,
                self.currency.take(),
                self.insurance_company_name.take(),
                self.insurance_type.take(),
            ) else {
                return None;
            };
            Some(Self::Out { amount, currency, insurance_company_name, insurance_type })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance
    {
        type Builder =
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsurance
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "insurance_company_name" => {
                        b.insurance_company_name = FromValueOpt::from_value(v)
                    }
                    "insurance_type" => b.insurance_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of insurance.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType
{
    LiabilitySupplement,
    LossDamageWaiver,
    Other,
    PartialDamageWaiver,
    PersonalAccident,
    PersonalEffects,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType::*;
        match self {
            LiabilitySupplement => "liability_supplement",
            LossDamageWaiver => "loss_damage_waiver",
            Other => "other",
            PartialDamageWaiver => "partial_damage_waiver",
            PersonalAccident => "personal_accident",
            PersonalEffects => "personal_effects",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType::*;
        match s {
    "liability_supplement" => Ok(LiabilitySupplement),
"loss_damage_waiver" => Ok(LossDamageWaiver),
"other" => Ok(Other),
"partial_damage_waiver" => Ok(PartialDamageWaiver),
"personal_accident" => Ok(PersonalAccident),
"personal_effects" => Ok(PersonalEffects),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryInsuranceInsuranceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
