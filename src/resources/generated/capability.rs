// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{CapabilityId};
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{Account};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "AccountCapability".
///
/// For more details see <https://stripe.com/docs/api/capabilities/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Capability {
    /// The identifier for the capability.
    pub id: CapabilityId,

    /// The account for which the capability enables functionality.
    pub account: Expandable<Account>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<AccountCapabilityFutureRequirements>,

    /// Whether the capability has been requested.
    pub requested: bool,

    /// Time at which the capability was requested.
    ///
    /// Measured in seconds since the Unix epoch.
    pub requested_at: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountCapabilityRequirements>,

    /// The status of the capability.
    pub status: CapabilityStatus,
}

impl Object for Capability {
    type Id = CapabilityId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "capability"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountCapabilityFutureRequirements {

    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,

    /// Date on which `future_requirements` becomes the main `requirements` hash and `future_requirements` becomes empty.
    ///
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on the capability's enablement state prior to transitioning.
    pub current_deadline: Option<Timestamp>,

    /// Fields that need to be collected to keep the capability enabled.
    ///
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Vec<String>,

    /// This is typed as an enum for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is null because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Option<AccountCapabilityFutureRequirementsDisabledReason>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,

    /// Fields you must collect when all thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Vec<String>,

    /// Fields that weren't collected by `requirements.current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,

    /// Fields that might become required depending on the results of verification or review.
    ///
    /// It's an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    /// Fields might appear in `eventually_due` or `currently_due` and in `pending_verification` if verification fails but another verification is still pending.
    pub pending_verification: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountCapabilityRequirements {

    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,

    /// Date by which the fields in `currently_due` must be collected to keep the capability enabled for the account.
    ///
    /// These fields may disable the capability sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<Timestamp>,

    /// Fields that need to be collected to keep the capability enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the capability is disabled.
    pub currently_due: Vec<String>,

    /// Description of why the capability is disabled.
    ///
    /// [Learn more about handling verification issues](https://stripe.com/docs/connect/handling-api-verification).
    pub disabled_reason: Option<AccountCapabilityRequirementsDisabledReason>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,

    /// Fields you must collect when all thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Vec<String>,

    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    pub past_due: Vec<String>,

    /// Fields that might become required depending on the results of verification or review.
    ///
    /// It's an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    /// Fields might appear in `eventually_due`, `currently_due`, or `past_due` and in `pending_verification` if verification fails but another verification is still pending.
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

/// An enum representing the possible values of an `AccountCapabilityFutureRequirements`'s `disabled_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilityFutureRequirementsDisabledReason {
    Other,
    #[serde(rename = "paused.inactivity")]
    PausedInactivity,
    #[serde(rename = "pending.onboarding")]
    PendingOnboarding,
    #[serde(rename = "pending.review")]
    PendingReview,
    PlatformDisabled,
    PlatformPaused,
    #[serde(rename = "rejected.inactivity")]
    RejectedInactivity,
    #[serde(rename = "rejected.other")]
    RejectedOther,
    #[serde(rename = "rejected.unsupported_business")]
    RejectedUnsupportedBusiness,
    #[serde(rename = "requirements.fields_needed")]
    RequirementsFieldsNeeded,
}

impl AccountCapabilityFutureRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilityFutureRequirementsDisabledReason::Other => "other",
            AccountCapabilityFutureRequirementsDisabledReason::PausedInactivity => "paused.inactivity",
            AccountCapabilityFutureRequirementsDisabledReason::PendingOnboarding => "pending.onboarding",
            AccountCapabilityFutureRequirementsDisabledReason::PendingReview => "pending.review",
            AccountCapabilityFutureRequirementsDisabledReason::PlatformDisabled => "platform_disabled",
            AccountCapabilityFutureRequirementsDisabledReason::PlatformPaused => "platform_paused",
            AccountCapabilityFutureRequirementsDisabledReason::RejectedInactivity => "rejected.inactivity",
            AccountCapabilityFutureRequirementsDisabledReason::RejectedOther => "rejected.other",
            AccountCapabilityFutureRequirementsDisabledReason::RejectedUnsupportedBusiness => "rejected.unsupported_business",
            AccountCapabilityFutureRequirementsDisabledReason::RequirementsFieldsNeeded => "requirements.fields_needed",
        }
    }
}

impl AsRef<str> for AccountCapabilityFutureRequirementsDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilityFutureRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilityFutureRequirementsDisabledReason {
    fn default() -> Self {
        Self::Other
    }
}

/// An enum representing the possible values of an `AccountCapabilityRequirements`'s `disabled_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilityRequirementsDisabledReason {
    Other,
    #[serde(rename = "paused.inactivity")]
    PausedInactivity,
    #[serde(rename = "pending.onboarding")]
    PendingOnboarding,
    #[serde(rename = "pending.review")]
    PendingReview,
    PlatformDisabled,
    PlatformPaused,
    #[serde(rename = "rejected.inactivity")]
    RejectedInactivity,
    #[serde(rename = "rejected.other")]
    RejectedOther,
    #[serde(rename = "rejected.unsupported_business")]
    RejectedUnsupportedBusiness,
    #[serde(rename = "requirements.fields_needed")]
    RequirementsFieldsNeeded,
}

impl AccountCapabilityRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilityRequirementsDisabledReason::Other => "other",
            AccountCapabilityRequirementsDisabledReason::PausedInactivity => "paused.inactivity",
            AccountCapabilityRequirementsDisabledReason::PendingOnboarding => "pending.onboarding",
            AccountCapabilityRequirementsDisabledReason::PendingReview => "pending.review",
            AccountCapabilityRequirementsDisabledReason::PlatformDisabled => "platform_disabled",
            AccountCapabilityRequirementsDisabledReason::PlatformPaused => "platform_paused",
            AccountCapabilityRequirementsDisabledReason::RejectedInactivity => "rejected.inactivity",
            AccountCapabilityRequirementsDisabledReason::RejectedOther => "rejected.other",
            AccountCapabilityRequirementsDisabledReason::RejectedUnsupportedBusiness => "rejected.unsupported_business",
            AccountCapabilityRequirementsDisabledReason::RequirementsFieldsNeeded => "requirements.fields_needed",
        }
    }
}

impl AsRef<str> for AccountCapabilityRequirementsDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilityRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilityRequirementsDisabledReason {
    fn default() -> Self {
        Self::Other
    }
}

/// An enum representing the possible values of an `AccountRequirementsError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountRequirementsErrorCode {
    InformationMissing,
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
    InvalidSignator,
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
    VerificationFailedAuthorizerAuthority,
    VerificationFailedBusinessIecNumber,
    VerificationFailedDocumentMatch,
    VerificationFailedIdNumberMatch,
    VerificationFailedKeyedIdentity,
    VerificationFailedKeyedMatch,
    VerificationFailedNameMatch,
    VerificationFailedOther,
    VerificationFailedRepresentativeAuthority,
    VerificationFailedResidentialAddress,
    VerificationFailedTaxIdMatch,
    VerificationFailedTaxIdNotIssued,
    VerificationMissingDirectors,
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRejectedOwnershipExemptionReason,
    VerificationRequiresAdditionalMemorandumOfAssociations,
    VerificationRequiresAdditionalProofOfRegistration,
    VerificationSupportability,
}

impl AccountRequirementsErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountRequirementsErrorCode::InformationMissing => "information_missing",
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
            AccountRequirementsErrorCode::InvalidSignator => "invalid_signator",
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
            AccountRequirementsErrorCode::VerificationFailedAuthorizerAuthority => "verification_failed_authorizer_authority",
            AccountRequirementsErrorCode::VerificationFailedBusinessIecNumber => "verification_failed_business_iec_number",
            AccountRequirementsErrorCode::VerificationFailedDocumentMatch => "verification_failed_document_match",
            AccountRequirementsErrorCode::VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            AccountRequirementsErrorCode::VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            AccountRequirementsErrorCode::VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            AccountRequirementsErrorCode::VerificationFailedNameMatch => "verification_failed_name_match",
            AccountRequirementsErrorCode::VerificationFailedOther => "verification_failed_other",
            AccountRequirementsErrorCode::VerificationFailedRepresentativeAuthority => "verification_failed_representative_authority",
            AccountRequirementsErrorCode::VerificationFailedResidentialAddress => "verification_failed_residential_address",
            AccountRequirementsErrorCode::VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            AccountRequirementsErrorCode::VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
            AccountRequirementsErrorCode::VerificationMissingDirectors => "verification_missing_directors",
            AccountRequirementsErrorCode::VerificationMissingExecutives => "verification_missing_executives",
            AccountRequirementsErrorCode::VerificationMissingOwners => "verification_missing_owners",
            AccountRequirementsErrorCode::VerificationRejectedOwnershipExemptionReason => "verification_rejected_ownership_exemption_reason",
            AccountRequirementsErrorCode::VerificationRequiresAdditionalMemorandumOfAssociations => "verification_requires_additional_memorandum_of_associations",
            AccountRequirementsErrorCode::VerificationRequiresAdditionalProofOfRegistration => "verification_requires_additional_proof_of_registration",
            AccountRequirementsErrorCode::VerificationSupportability => "verification_supportability",
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
        Self::InformationMissing
    }
}

/// An enum representing the possible values of an `Capability`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Active,
    Disabled,
    Inactive,
    Pending,
    Unrequested,
}

impl CapabilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CapabilityStatus::Active => "active",
            CapabilityStatus::Disabled => "disabled",
            CapabilityStatus::Inactive => "inactive",
            CapabilityStatus::Pending => "pending",
            CapabilityStatus::Unrequested => "unrequested",
        }
    }
}

impl AsRef<str> for CapabilityStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CapabilityStatus {
    fn default() -> Self {
        Self::Active
    }
}
