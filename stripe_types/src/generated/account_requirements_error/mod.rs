#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountRequirementsError {
    /// The code for the type of error.
    pub code: AccountRequirementsErrorCode,
    /// An informative message that indicates the error type and provides additional details about the error.
    pub reason: String,
    /// The specific user onboarding requirement field (in the requirements hash) that needs to be resolved.
    pub requirement: String,
}
/// The code for the type of error.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountRequirementsErrorCode {
    InvalidAddressCityStatePostalCode,
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

impl AccountRequirementsErrorCode {
    pub fn as_str(self) -> &'static str {
        use AccountRequirementsErrorCode::*;
        match self {
            InvalidAddressCityStatePostalCode => "invalid_address_city_state_postal_code",
            InvalidDobAgeUnder18 => "invalid_dob_age_under_18",
            InvalidRepresentativeCountry => "invalid_representative_country",
            InvalidStreetAddress => "invalid_street_address",
            InvalidTosAcceptance => "invalid_tos_acceptance",
            InvalidValueOther => "invalid_value_other",
            VerificationDocumentAddressMismatch => "verification_document_address_mismatch",
            VerificationDocumentAddressMissing => "verification_document_address_missing",
            VerificationDocumentCorrupt => "verification_document_corrupt",
            VerificationDocumentCountryNotSupported => {
                "verification_document_country_not_supported"
            }
            VerificationDocumentDobMismatch => "verification_document_dob_mismatch",
            VerificationDocumentDuplicateType => "verification_document_duplicate_type",
            VerificationDocumentExpired => "verification_document_expired",
            VerificationDocumentFailedCopy => "verification_document_failed_copy",
            VerificationDocumentFailedGreyscale => "verification_document_failed_greyscale",
            VerificationDocumentFailedOther => "verification_document_failed_other",
            VerificationDocumentFailedTestMode => "verification_document_failed_test_mode",
            VerificationDocumentFraudulent => "verification_document_fraudulent",
            VerificationDocumentIdNumberMismatch => "verification_document_id_number_mismatch",
            VerificationDocumentIdNumberMissing => "verification_document_id_number_missing",
            VerificationDocumentIncomplete => "verification_document_incomplete",
            VerificationDocumentInvalid => "verification_document_invalid",
            VerificationDocumentIssueOrExpiryDateMissing => {
                "verification_document_issue_or_expiry_date_missing"
            }
            VerificationDocumentManipulated => "verification_document_manipulated",
            VerificationDocumentMissingBack => "verification_document_missing_back",
            VerificationDocumentMissingFront => "verification_document_missing_front",
            VerificationDocumentNameMismatch => "verification_document_name_mismatch",
            VerificationDocumentNameMissing => "verification_document_name_missing",
            VerificationDocumentNationalityMismatch => "verification_document_nationality_mismatch",
            VerificationDocumentNotReadable => "verification_document_not_readable",
            VerificationDocumentNotSigned => "verification_document_not_signed",
            VerificationDocumentNotUploaded => "verification_document_not_uploaded",
            VerificationDocumentPhotoMismatch => "verification_document_photo_mismatch",
            VerificationDocumentTooLarge => "verification_document_too_large",
            VerificationDocumentTypeNotSupported => "verification_document_type_not_supported",
            VerificationFailedAddressMatch => "verification_failed_address_match",
            VerificationFailedBusinessIecNumber => "verification_failed_business_iec_number",
            VerificationFailedDocumentMatch => "verification_failed_document_match",
            VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            VerificationFailedNameMatch => "verification_failed_name_match",
            VerificationFailedOther => "verification_failed_other",
            VerificationFailedResidentialAddress => "verification_failed_residential_address",
            VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
            VerificationMissingExecutives => "verification_missing_executives",
            VerificationMissingOwners => "verification_missing_owners",
            VerificationRequiresAdditionalMemorandumOfAssociations => {
                "verification_requires_additional_memorandum_of_associations"
            }
        }
    }
}

impl std::str::FromStr for AccountRequirementsErrorCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountRequirementsErrorCode::*;
        match s {
            "invalid_address_city_state_postal_code" => Ok(InvalidAddressCityStatePostalCode),
            "invalid_dob_age_under_18" => Ok(InvalidDobAgeUnder18),
            "invalid_representative_country" => Ok(InvalidRepresentativeCountry),
            "invalid_street_address" => Ok(InvalidStreetAddress),
            "invalid_tos_acceptance" => Ok(InvalidTosAcceptance),
            "invalid_value_other" => Ok(InvalidValueOther),
            "verification_document_address_mismatch" => Ok(VerificationDocumentAddressMismatch),
            "verification_document_address_missing" => Ok(VerificationDocumentAddressMissing),
            "verification_document_corrupt" => Ok(VerificationDocumentCorrupt),
            "verification_document_country_not_supported" => {
                Ok(VerificationDocumentCountryNotSupported)
            }
            "verification_document_dob_mismatch" => Ok(VerificationDocumentDobMismatch),
            "verification_document_duplicate_type" => Ok(VerificationDocumentDuplicateType),
            "verification_document_expired" => Ok(VerificationDocumentExpired),
            "verification_document_failed_copy" => Ok(VerificationDocumentFailedCopy),
            "verification_document_failed_greyscale" => Ok(VerificationDocumentFailedGreyscale),
            "verification_document_failed_other" => Ok(VerificationDocumentFailedOther),
            "verification_document_failed_test_mode" => Ok(VerificationDocumentFailedTestMode),
            "verification_document_fraudulent" => Ok(VerificationDocumentFraudulent),
            "verification_document_id_number_mismatch" => Ok(VerificationDocumentIdNumberMismatch),
            "verification_document_id_number_missing" => Ok(VerificationDocumentIdNumberMissing),
            "verification_document_incomplete" => Ok(VerificationDocumentIncomplete),
            "verification_document_invalid" => Ok(VerificationDocumentInvalid),
            "verification_document_issue_or_expiry_date_missing" => {
                Ok(VerificationDocumentIssueOrExpiryDateMissing)
            }
            "verification_document_manipulated" => Ok(VerificationDocumentManipulated),
            "verification_document_missing_back" => Ok(VerificationDocumentMissingBack),
            "verification_document_missing_front" => Ok(VerificationDocumentMissingFront),
            "verification_document_name_mismatch" => Ok(VerificationDocumentNameMismatch),
            "verification_document_name_missing" => Ok(VerificationDocumentNameMissing),
            "verification_document_nationality_mismatch" => {
                Ok(VerificationDocumentNationalityMismatch)
            }
            "verification_document_not_readable" => Ok(VerificationDocumentNotReadable),
            "verification_document_not_signed" => Ok(VerificationDocumentNotSigned),
            "verification_document_not_uploaded" => Ok(VerificationDocumentNotUploaded),
            "verification_document_photo_mismatch" => Ok(VerificationDocumentPhotoMismatch),
            "verification_document_too_large" => Ok(VerificationDocumentTooLarge),
            "verification_document_type_not_supported" => Ok(VerificationDocumentTypeNotSupported),
            "verification_failed_address_match" => Ok(VerificationFailedAddressMatch),
            "verification_failed_business_iec_number" => Ok(VerificationFailedBusinessIecNumber),
            "verification_failed_document_match" => Ok(VerificationFailedDocumentMatch),
            "verification_failed_id_number_match" => Ok(VerificationFailedIdNumberMatch),
            "verification_failed_keyed_identity" => Ok(VerificationFailedKeyedIdentity),
            "verification_failed_keyed_match" => Ok(VerificationFailedKeyedMatch),
            "verification_failed_name_match" => Ok(VerificationFailedNameMatch),
            "verification_failed_other" => Ok(VerificationFailedOther),
            "verification_failed_residential_address" => Ok(VerificationFailedResidentialAddress),
            "verification_failed_tax_id_match" => Ok(VerificationFailedTaxIdMatch),
            "verification_failed_tax_id_not_issued" => Ok(VerificationFailedTaxIdNotIssued),
            "verification_missing_executives" => Ok(VerificationMissingExecutives),
            "verification_missing_owners" => Ok(VerificationMissingOwners),
            "verification_requires_additional_memorandum_of_associations" => {
                Ok(VerificationRequiresAdditionalMemorandumOfAssociations)
            }
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountRequirementsErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountRequirementsErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountRequirementsErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountRequirementsErrorCode"))
    }
}
