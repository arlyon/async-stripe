// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::PersonId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Address, File};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Person".
///
/// For more details see <https://stripe.com/docs/api/persons/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Person {
    /// Unique identifier for the object.
    pub id: PersonId,

    /// The account the person is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tos_acceptances: Option<PersonAdditionalTosAcceptances>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The Kana variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    /// The Kanji variation of the person's address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<LegalEntityDob>,

    /// The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    /// The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    /// A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,

    /// Information about the [upcoming new requirements for this person](https://stripe.com/docs/connect/custom-accounts/future-requirements), including what information needs to be collected, and by when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<PersonFutureRequirements>,

    /// The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// Whether the person's `id_number` was provided.
    ///
    /// True if either the full ID number was provided or if only the required part of the ID number was provided (ex.
    /// last four of an individual's SSN for the US indicated by `ssn_last_4_provided`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_provided: Option<bool>,

    /// Whether the person's `id_number_secondary` was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary_provided: Option<bool>,

    /// The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    /// The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    /// The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The country where the person is a national.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,

    /// The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<PersonPoliticalExposure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PersonRelationship>,

    /// Information about the requirements for this person, including what information needs to be collected, and by when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<PersonRequirements>,

    /// Whether the last four digits of the person's Social Security number have been provided (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerification>,
}

impl Object for Person {
    type Id = PersonId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "person"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LegalEntityDob {
    /// The day of birth, between 1 and 31.
    pub day: Option<i64>,

    /// The month of birth, between 1 and 12.
    pub month: Option<i64>,

    /// The four-digit year of birth.
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonVerification {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocument>,

    /// A user-displayable string describing the verification state for the person.
    ///
    /// For example, this may say "Provided identity information could not be verified".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`.
    ///
    /// A machine-readable code specifying the verification state for the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocument>,

    /// The state of verification for the person.
    ///
    /// Possible values are `unverified`, `pending`, or `verified`.
    pub status: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonVerificationDocument {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<Expandable<File>>,

    /// A user-displayable string describing the verification state of this document.
    ///
    /// For example, if a document is uploaded and the picture is too fuzzy, this may say "Identity document is too unclear to read".
    pub details: Option<String>,

    /// One of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_failed_greyscale`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_missing_back`, `document_missing_front`, `document_not_readable`, `document_not_uploaded`, `document_photo_mismatch`, `document_too_large`, or `document_type_not_supported`.
    ///
    /// A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,

    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<Expandable<File>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonAdditionalTosAcceptances {
    pub account: PersonAdditionalTosAcceptance,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonAdditionalTosAcceptance {
    /// The Unix timestamp marking when the legal guardian accepted the service agreement.
    pub date: Option<Timestamp>,

    /// The IP address from which the legal guardian accepted the service agreement.
    pub ip: Option<String>,

    /// The user agent of the browser from which the legal guardian accepted the service agreement.
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonFutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,

    /// Fields that need to be collected to keep the person's account enabled.
    ///
    /// If not collected by the account's `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash, and may immediately become `past_due`, but the account may also be given a grace period depending on the account's enablement state prior to transition.
    pub currently_due: Vec<String>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and the account's `future_requirements[current_deadline]` becomes set.
    pub eventually_due: Vec<String>,

    /// Fields that weren't collected by the account's `requirements.current_deadline`.
    ///
    /// These fields need to be collected to enable the person's account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountRequirementsAlternative {
    /// Fields that can be provided to satisfy all fields in `original_fields_due`.
    pub alternative_fields_due: Vec<String>,

    /// Fields that are due and can be satisfied by providing all fields in `alternative_fields_due`.
    pub original_fields_due: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountRequirementsError {
    /// The code for the type of error.
    pub code: AccountRequirementsErrorCode,

    /// An informative message that indicates the error type and provides additional details about the error.
    pub reason: String,

    /// The specific user onboarding requirement field (in the requirements hash) that needs to be resolved.
    pub requirement: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonRelationship {
    /// Whether the person is a director of the account's legal entity.
    ///
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    pub director: Option<bool>,

    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    pub executive: Option<bool>,

    /// Whether the person is the legal guardian of the account's representative.
    pub legal_guardian: Option<bool>,

    /// Whether the person is an owner of the account’s legal entity.
    pub owner: Option<bool>,

    /// The percent owned by the person of the account's legal entity.
    pub percent_ownership: Option<f64>,

    /// Whether the person is authorized as the primary representative of the account.
    ///
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    pub representative: Option<bool>,

    /// The person's title (e.g., CEO, Support Engineer).
    pub title: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,

    /// Fields that need to be collected to keep the person's account enabled.
    ///
    /// If not collected by the account's `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Vec<String>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and the account's `current_deadline` becomes set.
    pub eventually_due: Vec<String>,

    /// Fields that weren't collected by the account's `current_deadline`.
    ///
    /// These fields need to be collected to enable the person's account.
    pub past_due: Vec<String>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}

/// An enum representing the possible values of an `AccountRequirementsError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountRequirementsErrorCode {
    InvalidAddressCityStatePostalCode,
    InvalidAddressHighwayContractBox,
    InvalidAddressPrivateMailbox,
    InvalidBusinessProfileName,
    InvalidBusinessProfileNameDenylisted,
    InvalidCompanyNameDenylisted,
    InvalidDobAgeOverMaximum,
    #[serde(rename = "invalid_dob_age_under_18")]
    InvalidDobAgeUnder18,
    InvalidDobAgeUnderMinimum,
    InvalidProductDescriptionLength,
    InvalidProductDescriptionUrlMatch,
    InvalidRepresentativeCountry,
    InvalidStatementDescriptorBusinessMismatch,
    InvalidStatementDescriptorDenylisted,
    InvalidStatementDescriptorLength,
    InvalidStatementDescriptorPrefixDenylisted,
    InvalidStatementDescriptorPrefixMismatch,
    InvalidStreetAddress,
    InvalidTaxId,
    InvalidTaxIdFormat,
    InvalidTosAcceptance,
    InvalidUrlDenylisted,
    InvalidUrlFormat,
    InvalidUrlLength,
    InvalidUrlWebPresenceDetected,
    InvalidUrlWebsiteBusinessInformationMismatch,
    InvalidUrlWebsiteEmpty,
    InvalidUrlWebsiteInaccessible,
    InvalidUrlWebsiteInaccessibleGeoblocked,
    InvalidUrlWebsiteInaccessiblePasswordProtected,
    InvalidUrlWebsiteIncomplete,
    InvalidUrlWebsiteIncompleteCancellationPolicy,
    InvalidUrlWebsiteIncompleteCustomerServiceDetails,
    InvalidUrlWebsiteIncompleteLegalRestrictions,
    InvalidUrlWebsiteIncompleteRefundPolicy,
    InvalidUrlWebsiteIncompleteReturnPolicy,
    InvalidUrlWebsiteIncompleteTermsAndConditions,
    InvalidUrlWebsiteIncompleteUnderConstruction,
    InvalidUrlWebsiteOther,
    InvalidValueOther,
    VerificationDirectorsMismatch,
    VerificationDocumentAddressMismatch,
    VerificationDocumentAddressMissing,
    VerificationDocumentCorrupt,
    VerificationDocumentCountryNotSupported,
    VerificationDocumentDirectorsMismatch,
    VerificationDocumentDobMismatch,
    VerificationDocumentDuplicateType,
    VerificationDocumentExpired,
    VerificationDocumentFailedCopy,
    VerificationDocumentFailedGreyscale,
    VerificationDocumentFailedOther,
    VerificationDocumentFailedTestMode,
    VerificationDocumentFraudulent,
    VerificationDocumentIdNumberMismatch,
    VerificationDocumentIdNumberMissing,
    VerificationDocumentIncomplete,
    VerificationDocumentInvalid,
    VerificationDocumentIssueOrExpiryDateMissing,
    VerificationDocumentManipulated,
    VerificationDocumentMissingBack,
    VerificationDocumentMissingFront,
    VerificationDocumentNameMismatch,
    VerificationDocumentNameMissing,
    VerificationDocumentNationalityMismatch,
    VerificationDocumentNotReadable,
    VerificationDocumentNotSigned,
    VerificationDocumentNotUploaded,
    VerificationDocumentPhotoMismatch,
    VerificationDocumentTooLarge,
    VerificationDocumentTypeNotSupported,
    VerificationExtraneousDirectors,
    VerificationFailedAddressMatch,
    VerificationFailedBusinessIecNumber,
    VerificationFailedDocumentMatch,
    VerificationFailedIdNumberMatch,
    VerificationFailedKeyedIdentity,
    VerificationFailedKeyedMatch,
    VerificationFailedNameMatch,
    VerificationFailedOther,
    VerificationFailedResidentialAddress,
    VerificationFailedTaxIdMatch,
    VerificationFailedTaxIdNotIssued,
    VerificationMissingDirectors,
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRequiresAdditionalMemorandumOfAssociations,
}

impl AccountRequirementsErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountRequirementsErrorCode::InvalidAddressCityStatePostalCode => "invalid_address_city_state_postal_code",
            AccountRequirementsErrorCode::InvalidAddressHighwayContractBox => "invalid_address_highway_contract_box",
            AccountRequirementsErrorCode::InvalidAddressPrivateMailbox => "invalid_address_private_mailbox",
            AccountRequirementsErrorCode::InvalidBusinessProfileName => "invalid_business_profile_name",
            AccountRequirementsErrorCode::InvalidBusinessProfileNameDenylisted => "invalid_business_profile_name_denylisted",
            AccountRequirementsErrorCode::InvalidCompanyNameDenylisted => "invalid_company_name_denylisted",
            AccountRequirementsErrorCode::InvalidDobAgeOverMaximum => "invalid_dob_age_over_maximum",
            AccountRequirementsErrorCode::InvalidDobAgeUnder18 => "invalid_dob_age_under_18",
            AccountRequirementsErrorCode::InvalidDobAgeUnderMinimum => "invalid_dob_age_under_minimum",
            AccountRequirementsErrorCode::InvalidProductDescriptionLength => "invalid_product_description_length",
            AccountRequirementsErrorCode::InvalidProductDescriptionUrlMatch => "invalid_product_description_url_match",
            AccountRequirementsErrorCode::InvalidRepresentativeCountry => "invalid_representative_country",
            AccountRequirementsErrorCode::InvalidStatementDescriptorBusinessMismatch => "invalid_statement_descriptor_business_mismatch",
            AccountRequirementsErrorCode::InvalidStatementDescriptorDenylisted => "invalid_statement_descriptor_denylisted",
            AccountRequirementsErrorCode::InvalidStatementDescriptorLength => "invalid_statement_descriptor_length",
            AccountRequirementsErrorCode::InvalidStatementDescriptorPrefixDenylisted => "invalid_statement_descriptor_prefix_denylisted",
            AccountRequirementsErrorCode::InvalidStatementDescriptorPrefixMismatch => "invalid_statement_descriptor_prefix_mismatch",
            AccountRequirementsErrorCode::InvalidStreetAddress => "invalid_street_address",
            AccountRequirementsErrorCode::InvalidTaxId => "invalid_tax_id",
            AccountRequirementsErrorCode::InvalidTaxIdFormat => "invalid_tax_id_format",
            AccountRequirementsErrorCode::InvalidTosAcceptance => "invalid_tos_acceptance",
            AccountRequirementsErrorCode::InvalidUrlDenylisted => "invalid_url_denylisted",
            AccountRequirementsErrorCode::InvalidUrlFormat => "invalid_url_format",
            AccountRequirementsErrorCode::InvalidUrlLength => "invalid_url_length",
            AccountRequirementsErrorCode::InvalidUrlWebPresenceDetected => "invalid_url_web_presence_detected",
            AccountRequirementsErrorCode::InvalidUrlWebsiteBusinessInformationMismatch => "invalid_url_website_business_information_mismatch",
            AccountRequirementsErrorCode::InvalidUrlWebsiteEmpty => "invalid_url_website_empty",
            AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessible => "invalid_url_website_inaccessible",
            AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessibleGeoblocked => "invalid_url_website_inaccessible_geoblocked",
            AccountRequirementsErrorCode::InvalidUrlWebsiteInaccessiblePasswordProtected => "invalid_url_website_inaccessible_password_protected",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncomplete => "invalid_url_website_incomplete",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteCancellationPolicy => "invalid_url_website_incomplete_cancellation_policy",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteCustomerServiceDetails => "invalid_url_website_incomplete_customer_service_details",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteLegalRestrictions => "invalid_url_website_incomplete_legal_restrictions",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteRefundPolicy => "invalid_url_website_incomplete_refund_policy",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteReturnPolicy => "invalid_url_website_incomplete_return_policy",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteTermsAndConditions => "invalid_url_website_incomplete_terms_and_conditions",
            AccountRequirementsErrorCode::InvalidUrlWebsiteIncompleteUnderConstruction => "invalid_url_website_incomplete_under_construction",
            AccountRequirementsErrorCode::InvalidUrlWebsiteOther => "invalid_url_website_other",
            AccountRequirementsErrorCode::InvalidValueOther => "invalid_value_other",
            AccountRequirementsErrorCode::VerificationDirectorsMismatch => "verification_directors_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentAddressMismatch => "verification_document_address_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentAddressMissing => "verification_document_address_missing",
            AccountRequirementsErrorCode::VerificationDocumentCorrupt => "verification_document_corrupt",
            AccountRequirementsErrorCode::VerificationDocumentCountryNotSupported => "verification_document_country_not_supported",
            AccountRequirementsErrorCode::VerificationDocumentDirectorsMismatch => "verification_document_directors_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentDobMismatch => "verification_document_dob_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentDuplicateType => "verification_document_duplicate_type",
            AccountRequirementsErrorCode::VerificationDocumentExpired => "verification_document_expired",
            AccountRequirementsErrorCode::VerificationDocumentFailedCopy => "verification_document_failed_copy",
            AccountRequirementsErrorCode::VerificationDocumentFailedGreyscale => "verification_document_failed_greyscale",
            AccountRequirementsErrorCode::VerificationDocumentFailedOther => "verification_document_failed_other",
            AccountRequirementsErrorCode::VerificationDocumentFailedTestMode => "verification_document_failed_test_mode",
            AccountRequirementsErrorCode::VerificationDocumentFraudulent => "verification_document_fraudulent",
            AccountRequirementsErrorCode::VerificationDocumentIdNumberMismatch => "verification_document_id_number_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentIdNumberMissing => "verification_document_id_number_missing",
            AccountRequirementsErrorCode::VerificationDocumentIncomplete => "verification_document_incomplete",
            AccountRequirementsErrorCode::VerificationDocumentInvalid => "verification_document_invalid",
            AccountRequirementsErrorCode::VerificationDocumentIssueOrExpiryDateMissing => "verification_document_issue_or_expiry_date_missing",
            AccountRequirementsErrorCode::VerificationDocumentManipulated => "verification_document_manipulated",
            AccountRequirementsErrorCode::VerificationDocumentMissingBack => "verification_document_missing_back",
            AccountRequirementsErrorCode::VerificationDocumentMissingFront => "verification_document_missing_front",
            AccountRequirementsErrorCode::VerificationDocumentNameMismatch => "verification_document_name_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentNameMissing => "verification_document_name_missing",
            AccountRequirementsErrorCode::VerificationDocumentNationalityMismatch => "verification_document_nationality_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentNotReadable => "verification_document_not_readable",
            AccountRequirementsErrorCode::VerificationDocumentNotSigned => "verification_document_not_signed",
            AccountRequirementsErrorCode::VerificationDocumentNotUploaded => "verification_document_not_uploaded",
            AccountRequirementsErrorCode::VerificationDocumentPhotoMismatch => "verification_document_photo_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentTooLarge => "verification_document_too_large",
            AccountRequirementsErrorCode::VerificationDocumentTypeNotSupported => "verification_document_type_not_supported",
            AccountRequirementsErrorCode::VerificationExtraneousDirectors => "verification_extraneous_directors",
            AccountRequirementsErrorCode::VerificationFailedAddressMatch => "verification_failed_address_match",
            AccountRequirementsErrorCode::VerificationFailedBusinessIecNumber => "verification_failed_business_iec_number",
            AccountRequirementsErrorCode::VerificationFailedDocumentMatch => "verification_failed_document_match",
            AccountRequirementsErrorCode::VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            AccountRequirementsErrorCode::VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            AccountRequirementsErrorCode::VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            AccountRequirementsErrorCode::VerificationFailedNameMatch => "verification_failed_name_match",
            AccountRequirementsErrorCode::VerificationFailedOther => "verification_failed_other",
            AccountRequirementsErrorCode::VerificationFailedResidentialAddress => "verification_failed_residential_address",
            AccountRequirementsErrorCode::VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            AccountRequirementsErrorCode::VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
            AccountRequirementsErrorCode::VerificationMissingDirectors => "verification_missing_directors",
            AccountRequirementsErrorCode::VerificationMissingExecutives => "verification_missing_executives",
            AccountRequirementsErrorCode::VerificationMissingOwners => "verification_missing_owners",
            AccountRequirementsErrorCode::VerificationRequiresAdditionalMemorandumOfAssociations => "verification_requires_additional_memorandum_of_associations",
        }
    }
}

impl AsRef<str> for AccountRequirementsErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountRequirementsErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountRequirementsErrorCode {
    fn default() -> Self {
        Self::InvalidAddressCityStatePostalCode
    }
}

/// An enum representing the possible values of an `Person`'s `political_exposure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PersonPoliticalExposure {
    Existing,
    None,
}

impl PersonPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        match self {
            PersonPoliticalExposure::Existing => "existing",
            PersonPoliticalExposure::None => "none",
        }
    }
}

impl AsRef<str> for PersonPoliticalExposure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PersonPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PersonPoliticalExposure {
    fn default() -> Self {
        Self::Existing
    }
}
