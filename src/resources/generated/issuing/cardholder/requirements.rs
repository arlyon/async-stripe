#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Requirements {
    /// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    pub disabled_reason: Option<RequirementsDisabledReason>,
    /// Array of fields that need to be collected in order to verify and re-enable the cardholder.
    pub past_due: Option<Vec<RequirementsPastDue>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Requirements {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum RequirementsDisabledReason {
    Listed,
    #[serde(rename = "rejected.listed")]
    RejectedListed,
    UnderReview,
}

impl RequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Listed => "listed",
            Self::RejectedListed => "rejected.listed",
            Self::UnderReview => "under_review",
        }
    }
}

impl AsRef<str> for RequirementsDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Array of fields that need to be collected in order to verify and re-enable the cardholder.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum RequirementsPastDue {
    #[serde(rename = "company.tax_id")]
    CompanyTaxId,
    #[serde(rename = "individual.dob.day")]
    IndividualDobDay,
    #[serde(rename = "individual.dob.month")]
    IndividualDobMonth,
    #[serde(rename = "individual.dob.year")]
    IndividualDobYear,
    #[serde(rename = "individual.first_name")]
    IndividualFirstName,
    #[serde(rename = "individual.last_name")]
    IndividualLastName,
    #[serde(rename = "individual.verification.document")]
    IndividualVerificationDocument,
}

impl RequirementsPastDue {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompanyTaxId => "company.tax_id",
            Self::IndividualDobDay => "individual.dob.day",
            Self::IndividualDobMonth => "individual.dob.month",
            Self::IndividualDobYear => "individual.dob.year",
            Self::IndividualFirstName => "individual.first_name",
            Self::IndividualLastName => "individual.last_name",
            Self::IndividualVerificationDocument => "individual.verification.document",
        }
    }
}

impl AsRef<str> for RequirementsPastDue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
