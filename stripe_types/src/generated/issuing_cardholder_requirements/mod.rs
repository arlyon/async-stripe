#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderRequirements {
    /// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    pub disabled_reason: Option<IssuingCardholderRequirementsDisabledReason>,
    /// Array of fields that need to be collected in order to verify and re-enable the cardholder.
    pub past_due: Option<Vec<IssuingCardholderRequirementsPastDue>>,
}
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderRequirementsDisabledReason::*;
        match s {
            "listed" => Ok(Listed),
            "rejected.listed" => Ok(RejectedListed),
            "requirements.past_due" => Ok(RequirementsPastDue),
            "under_review" => Ok(UnderReview),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl serde::Serialize for IssuingCardholderRequirementsDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderRequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
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
    type Err = ();
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
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsPastDue {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl serde::Serialize for IssuingCardholderRequirementsPastDue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderRequirementsPastDue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardholderRequirementsPastDue")
        })
    }
}
