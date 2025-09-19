#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderRequirements {
    /// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    pub disabled_reason: Option<IssuingCardholderRequirementsDisabledReason>,
    /// Array of fields that need to be collected in order to verify and re-enable the cardholder.
    pub past_due: Option<Vec<IssuingCardholderRequirementsPastDue>>,
}
#[doc(hidden)]
pub struct IssuingCardholderRequirementsBuilder {
    disabled_reason: Option<Option<IssuingCardholderRequirementsDisabledReason>>,
    past_due: Option<Option<Vec<IssuingCardholderRequirementsPastDue>>>,
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

    impl Deserialize for IssuingCardholderRequirements {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderRequirements>,
        builder: IssuingCardholderRequirementsBuilder,
    }

    impl Visitor for Place<IssuingCardholderRequirements> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderRequirementsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardholderRequirementsBuilder {
        type Out = IssuingCardholderRequirements;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disabled_reason" => Deserialize::begin(&mut self.disabled_reason),
                "past_due" => Deserialize::begin(&mut self.past_due),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { disabled_reason: Deserialize::default(), past_due: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(disabled_reason), Some(past_due)) =
                (self.disabled_reason, self.past_due.take())
            else {
                return None;
            };
            Some(Self::Out { disabled_reason, past_due })
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

    impl ObjectDeser for IssuingCardholderRequirements {
        type Builder = IssuingCardholderRequirementsBuilder;
    }

    impl FromValueOpt for IssuingCardholderRequirements {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardholderRequirementsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "disabled_reason" => b.disabled_reason = FromValueOpt::from_value(v),
                    "past_due" => b.past_due = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    RejectedListed,
    RequirementsPastDue,
    UnderReview,
}
impl IssuingCardholderRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderRequirementsDisabledReason::*;
        match self {
            Listed => "listed",
            RejectedListed => "rejected.listed",
            RequirementsPastDue => "requirements.past_due",
            UnderReview => "under_review",
        }
    }
}

impl std::str::FromStr for IssuingCardholderRequirementsDisabledReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderRequirementsDisabledReason::*;
        match s {
            "listed" => Ok(Listed),
            "rejected.listed" => Ok(RejectedListed),
            "requirements.past_due" => Ok(RequirementsPastDue),
            "under_review" => Ok(UnderReview),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardholderRequirementsDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardholderRequirementsDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardholderRequirementsDisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingCardholderRequirementsDisabledReason::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardholderRequirementsDisabledReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardholderRequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingCardholderRequirementsDisabledReason",
            )
        })
    }
}
/// Array of fields that need to be collected in order to verify and re-enable the cardholder.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderRequirementsPastDue {
    CompanyTaxId,
    IndividualCardIssuingUserTermsAcceptanceDate,
    IndividualCardIssuingUserTermsAcceptanceIp,
    IndividualDobDay,
    IndividualDobMonth,
    IndividualDobYear,
    IndividualFirstName,
    IndividualLastName,
    IndividualVerificationDocument,
}
impl IssuingCardholderRequirementsPastDue {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderRequirementsPastDue::*;
        match self {
            CompanyTaxId => "company.tax_id",
            IndividualCardIssuingUserTermsAcceptanceDate => {
                "individual.card_issuing.user_terms_acceptance.date"
            }
            IndividualCardIssuingUserTermsAcceptanceIp => {
                "individual.card_issuing.user_terms_acceptance.ip"
            }
            IndividualDobDay => "individual.dob.day",
            IndividualDobMonth => "individual.dob.month",
            IndividualDobYear => "individual.dob.year",
            IndividualFirstName => "individual.first_name",
            IndividualLastName => "individual.last_name",
            IndividualVerificationDocument => "individual.verification.document",
        }
    }
}

impl std::str::FromStr for IssuingCardholderRequirementsPastDue {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderRequirementsPastDue::*;
        match s {
            "company.tax_id" => Ok(CompanyTaxId),
            "individual.card_issuing.user_terms_acceptance.date" => {
                Ok(IndividualCardIssuingUserTermsAcceptanceDate)
            }
            "individual.card_issuing.user_terms_acceptance.ip" => {
                Ok(IndividualCardIssuingUserTermsAcceptanceIp)
            }
            "individual.dob.day" => Ok(IndividualDobDay),
            "individual.dob.month" => Ok(IndividualDobMonth),
            "individual.dob.year" => Ok(IndividualDobYear),
            "individual.first_name" => Ok(IndividualFirstName),
            "individual.last_name" => Ok(IndividualLastName),
            "individual.verification.document" => Ok(IndividualVerificationDocument),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardholderRequirementsPastDue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardholderRequirementsPastDue {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardholderRequirementsPastDue> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingCardholderRequirementsPastDue::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardholderRequirementsPastDue);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardholderRequirementsPastDue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardholderRequirementsPastDue")
        })
    }
}
