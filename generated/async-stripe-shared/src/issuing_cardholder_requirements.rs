#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderRequirements {
    /// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    pub disabled_reason: Option<IssuingCardholderRequirementsDisabledReason>,
    /// Array of fields that need to be collected in order to verify and re-enable the cardholder.
    pub past_due: Option<Vec<IssuingCardholderRequirementsPastDue>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardholderRequirements").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardholderRequirementsBuilder {
    disabled_reason: Option<Option<IssuingCardholderRequirementsDisabledReason>>,
    past_due: Option<Option<Vec<IssuingCardholderRequirementsPastDue>>>,
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
                builder: IssuingCardholderRequirementsBuilder {
                    disabled_reason: Deserialize::default(),
                    past_due: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disabled_reason" => Deserialize::begin(&mut self.builder.disabled_reason),
                "past_due" => Deserialize::begin(&mut self.builder.past_due),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(disabled_reason), Some(past_due)) =
                (self.builder.disabled_reason.take(), self.builder.past_due.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingCardholderRequirements { disabled_reason, past_due });
            Ok(())
        }
    }
};
/// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    RejectedListed,
    RequirementsPastDue,
    UnderReview,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardholderRequirementsDisabledReason {
    pub fn as_str(&self) -> &str {
        use IssuingCardholderRequirementsDisabledReason::*;
        match self {
            Listed => "listed",
            RejectedListed => "rejected.listed",
            RequirementsPastDue => "requirements.past_due",
            UnderReview => "under_review",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingCardholderRequirementsDisabledReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderRequirementsDisabledReason::*;
        match s {
            "listed" => Ok(Listed),
            "rejected.listed" => Ok(RejectedListed),
            "requirements.past_due" => Ok(RequirementsPastDue),
            "under_review" => Ok(UnderReview),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingCardholderRequirementsDisabledReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingCardholderRequirementsDisabledReason))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingCardholderRequirementsDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingCardholderRequirementsDisabledReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingCardholderRequirementsDisabledReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardholderRequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Array of fields that need to be collected in order to verify and re-enable the cardholder.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardholderRequirementsPastDue {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingCardholderRequirementsPastDue {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingCardholderRequirementsPastDue"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingCardholderRequirementsPastDue)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingCardholderRequirementsPastDue {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingCardholderRequirementsPastDue> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardholderRequirementsPastDue::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardholderRequirementsPastDue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
