#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Errors {
    /// The code for the type of error.
    pub code: ErrorsCode,
    /// An informative message that indicates the error type and provides additional details about the error.
    pub reason: String,
    /// The specific user onboarding requirement field (in the requirements hash) that needs to be resolved.
    pub requirement: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Errors {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The code for the type of error.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ErrorsCode {
    InvalidAddressCityStatePostalCode,
    #[serde(rename = "invalid_dob_age_under_18")]
    InvalidDobAgeUnder18,
    InvalidRepresentativeCountry,
    InvalidStreetAddress,
    InvalidTosAcceptance,
    InvalidValueOther,
    VerificationDocumentAddressMismatch,
    VerificationDocumentAddressMissing,
    VerificationDocumentCorrupt,
    VerificationDocumentCountryNotSupported,
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
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRequiresAdditionalMemorandumOfAssociations,
}

impl ErrorsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidAddressCityStatePostalCode => "invalid_address_city_state_postal_code",
            Self::InvalidDobAgeUnder18 => "invalid_dob_age_under_18",
            Self::InvalidRepresentativeCountry => "invalid_representative_country",
            Self::InvalidStreetAddress => "invalid_street_address",
            Self::InvalidTosAcceptance => "invalid_tos_acceptance",
            Self::InvalidValueOther => "invalid_value_other",
            Self::VerificationDocumentAddressMismatch => "verification_document_address_mismatch",
            Self::VerificationDocumentAddressMissing => "verification_document_address_missing",
            Self::VerificationDocumentCorrupt => "verification_document_corrupt",
            Self::VerificationDocumentCountryNotSupported => {
                "verification_document_country_not_supported"
            }
            Self::VerificationDocumentDobMismatch => "verification_document_dob_mismatch",
            Self::VerificationDocumentDuplicateType => "verification_document_duplicate_type",
            Self::VerificationDocumentExpired => "verification_document_expired",
            Self::VerificationDocumentFailedCopy => "verification_document_failed_copy",
            Self::VerificationDocumentFailedGreyscale => "verification_document_failed_greyscale",
            Self::VerificationDocumentFailedOther => "verification_document_failed_other",
            Self::VerificationDocumentFailedTestMode => "verification_document_failed_test_mode",
            Self::VerificationDocumentFraudulent => "verification_document_fraudulent",
            Self::VerificationDocumentIdNumberMismatch => {
                "verification_document_id_number_mismatch"
            }
            Self::VerificationDocumentIdNumberMissing => "verification_document_id_number_missing",
            Self::VerificationDocumentIncomplete => "verification_document_incomplete",
            Self::VerificationDocumentInvalid => "verification_document_invalid",
            Self::VerificationDocumentIssueOrExpiryDateMissing => {
                "verification_document_issue_or_expiry_date_missing"
            }
            Self::VerificationDocumentManipulated => "verification_document_manipulated",
            Self::VerificationDocumentMissingBack => "verification_document_missing_back",
            Self::VerificationDocumentMissingFront => "verification_document_missing_front",
            Self::VerificationDocumentNameMismatch => "verification_document_name_mismatch",
            Self::VerificationDocumentNameMissing => "verification_document_name_missing",
            Self::VerificationDocumentNationalityMismatch => {
                "verification_document_nationality_mismatch"
            }
            Self::VerificationDocumentNotReadable => "verification_document_not_readable",
            Self::VerificationDocumentNotSigned => "verification_document_not_signed",
            Self::VerificationDocumentNotUploaded => "verification_document_not_uploaded",
            Self::VerificationDocumentPhotoMismatch => "verification_document_photo_mismatch",
            Self::VerificationDocumentTooLarge => "verification_document_too_large",
            Self::VerificationDocumentTypeNotSupported => {
                "verification_document_type_not_supported"
            }
            Self::VerificationFailedAddressMatch => "verification_failed_address_match",
            Self::VerificationFailedBusinessIecNumber => "verification_failed_business_iec_number",
            Self::VerificationFailedDocumentMatch => "verification_failed_document_match",
            Self::VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            Self::VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            Self::VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            Self::VerificationFailedNameMatch => "verification_failed_name_match",
            Self::VerificationFailedOther => "verification_failed_other",
            Self::VerificationFailedResidentialAddress => "verification_failed_residential_address",
            Self::VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            Self::VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
            Self::VerificationMissingExecutives => "verification_missing_executives",
            Self::VerificationMissingOwners => "verification_missing_owners",
            Self::VerificationRequiresAdditionalMemorandumOfAssociations => {
                "verification_requires_additional_memorandum_of_associations"
            }
        }
    }
}

impl AsRef<str> for ErrorsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ErrorsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
