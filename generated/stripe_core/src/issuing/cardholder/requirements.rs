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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequirementsDisabledReason {
    Listed,
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

impl std::str::FromStr for RequirementsDisabledReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "listed" => Ok(Self::Listed),
            "rejected.listed" => Ok(Self::RejectedListed),
            "under_review" => Ok(Self::UnderReview),

            _ => Err(()),
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
impl serde::Serialize for RequirementsDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RequirementsDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RequirementsDisabledReason"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RequirementsDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<RequirementsDisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RequirementsDisabledReason::from_str(s)?);
        Ok(())
    }
}
/// Array of fields that need to be collected in order to verify and re-enable the cardholder.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequirementsPastDue {
    CompanyTaxId,
    IndividualDobDay,
    IndividualDobMonth,
    IndividualDobYear,
    IndividualFirstName,
    IndividualLastName,
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

impl std::str::FromStr for RequirementsPastDue {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "company.tax_id" => Ok(Self::CompanyTaxId),
            "individual.dob.day" => Ok(Self::IndividualDobDay),
            "individual.dob.month" => Ok(Self::IndividualDobMonth),
            "individual.dob.year" => Ok(Self::IndividualDobYear),
            "individual.first_name" => Ok(Self::IndividualFirstName),
            "individual.last_name" => Ok(Self::IndividualLastName),
            "individual.verification.document" => Ok(Self::IndividualVerificationDocument),

            _ => Err(()),
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
impl serde::Serialize for RequirementsPastDue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RequirementsPastDue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RequirementsPastDue"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RequirementsPastDue {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<RequirementsPastDue> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RequirementsPastDue::from_str(s)?);
        Ok(())
    }
}
