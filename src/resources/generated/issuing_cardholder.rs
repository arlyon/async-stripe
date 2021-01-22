// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingCardholderId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Address, Currency, File, MerchantCategory, SpendingLimit};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingCardholder".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholder {
    /// Unique identifier for the object.
    pub id: IssuingCardholderId,

    pub billing: IssuingCardholderAddress,

    /// Additional information about a `company` cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<IssuingCardholderCompany>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The cardholder's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Additional information about an `individual` cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<IssuingCardholderIndividual>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The cardholder's name.
    ///
    /// This will be printed on cards issued to them.
    pub name: String,

    /// The cardholder's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    pub requirements: IssuingCardholderRequirements,

    /// Spending rules that give you some control over how this cardholder's cards can be used.
    ///
    /// Refer to our [authorizations](https://stripe.com/docs/issuing/purchases/authorizations) documentation for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<IssuingCardholderAuthorizationControls>,

    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub status: IssuingCardholderStatus,

    /// One of `individual` or `company`.
    #[serde(rename = "type")]
    pub type_: IssuingCardholderType,
}

impl Object for IssuingCardholder {
    type Id = IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.cardholder"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderAddress {
    pub address: Address,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderAuthorizationControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations permitted on this cardholder's cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<MerchantCategory>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to always decline on this cardholder's cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<MerchantCategory>>,

    /// Limit the spending with rules based on time intervals and categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimit>>,

    /// Currency for the amounts within spending_limits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<Currency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderCompany {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderIndividual {
    /// The date of birth of this cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<IssuingCardholderIndividualDob>,

    /// The first name of this cardholder.
    pub first_name: String,

    /// The last name of this cardholder.
    pub last_name: String,

    /// Government-issued ID document for this cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<IssuingCardholderVerification>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderIndividualDob {
    /// The day of birth, between 1 and 31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,

    /// The month of birth, between 1 and 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,

    /// The four-digit year of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderRequirements {
    /// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<IssuingCardholderRequirementsDisabledReason>,

    /// Array of fields that need to be collected in order to verify and re-enable the cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<IssuingCardholderRequirementsPastDue>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderVerification {
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<IssuingCardholderIdDocument>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderIdDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<Expandable<File>>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<Expandable<File>>,
}

/// An enum representing the possible values of an `IssuingCardholderRequirements`'s `disabled_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    #[serde(rename = "rejected.listed")]
    RejectedListed,
    UnderReview,
}

impl IssuingCardholderRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderRequirementsDisabledReason::Listed => "listed",
            IssuingCardholderRequirementsDisabledReason::RejectedListed => "rejected.listed",
            IssuingCardholderRequirementsDisabledReason::UnderReview => "under_review",
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
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardholderRequirements`'s `past_due` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsPastDue {
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

impl IssuingCardholderRequirementsPastDue {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderRequirementsPastDue::CompanyTaxId => "company.tax_id",
            IssuingCardholderRequirementsPastDue::IndividualDobDay => "individual.dob.day",
            IssuingCardholderRequirementsPastDue::IndividualDobMonth => "individual.dob.month",
            IssuingCardholderRequirementsPastDue::IndividualDobYear => "individual.dob.year",
            IssuingCardholderRequirementsPastDue::IndividualFirstName => "individual.first_name",
            IssuingCardholderRequirementsPastDue::IndividualLastName => "individual.last_name",
            IssuingCardholderRequirementsPastDue::IndividualVerificationDocument => {
                "individual.verification.document"
            }
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
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardholder`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderStatus {
    Active,
    Blocked,
    Inactive,
}

impl IssuingCardholderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderStatus::Active => "active",
            IssuingCardholderStatus::Blocked => "blocked",
            IssuingCardholderStatus::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for IssuingCardholderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardholder`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderType {
    Company,
    Individual,
}

impl IssuingCardholderType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderType::Company => "company",
            IssuingCardholderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for IssuingCardholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
