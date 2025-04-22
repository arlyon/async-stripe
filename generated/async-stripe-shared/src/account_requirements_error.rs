#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountRequirementsError {
    /// The code for the type of error.
    pub code: AccountRequirementsErrorCode,
    /// An informative message that indicates the error type and provides additional details about the error.
    pub reason: String,
    /// The specific user onboarding requirement field (in the requirements hash) that needs to be resolved.
    pub requirement: String,
}
#[doc(hidden)]
pub struct AccountRequirementsErrorBuilder {
    code: Option<AccountRequirementsErrorCode>,
    reason: Option<String>,
    requirement: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountRequirementsError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountRequirementsError>,
        builder: AccountRequirementsErrorBuilder,
    }

    impl Visitor for Place<AccountRequirementsError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountRequirementsErrorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountRequirementsErrorBuilder {
        type Out = AccountRequirementsError;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "reason" => Deserialize::begin(&mut self.reason),
                "requirement" => Deserialize::begin(&mut self.requirement),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                code: Deserialize::default(),
                reason: Deserialize::default(),
                requirement: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(reason), Some(requirement)) =
                (self.code.take(), self.reason.take(), self.requirement.take())
            else {
                return None;
            };
            Some(Self::Out { code, reason, requirement })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for AccountRequirementsError {
        type Builder = AccountRequirementsErrorBuilder;
    }

    impl FromValueOpt for AccountRequirementsError {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountRequirementsErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    "requirement" => b.requirement = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The code for the type of error.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountRequirementsErrorCode {
    InformationMissing,
    InvalidAddressCityStatePostalCode,
    InvalidAddressHighwayContractBox,
    InvalidAddressPrivateMailbox,
    InvalidBusinessProfileName,
    InvalidBusinessProfileNameDenylisted,
    InvalidCompanyNameDenylisted,
    InvalidDobAgeOverMaximum,
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
    VerificationLegalEntityStructureMismatch,
    VerificationMissingDirectors,
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRejectedOwnershipExemptionReason,
    VerificationRequiresAdditionalMemorandumOfAssociations,
    VerificationRequiresAdditionalProofOfRegistration,
    VerificationSupportability,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountRequirementsErrorCode {
    pub fn as_str(&self) -> &str {
        use AccountRequirementsErrorCode::*;
        match self {
            InformationMissing => "information_missing",
            InvalidAddressCityStatePostalCode => "invalid_address_city_state_postal_code",
            InvalidAddressHighwayContractBox => "invalid_address_highway_contract_box",
            InvalidAddressPrivateMailbox => "invalid_address_private_mailbox",
            InvalidBusinessProfileName => "invalid_business_profile_name",
            InvalidBusinessProfileNameDenylisted => "invalid_business_profile_name_denylisted",
            InvalidCompanyNameDenylisted => "invalid_company_name_denylisted",
            InvalidDobAgeOverMaximum => "invalid_dob_age_over_maximum",
            InvalidDobAgeUnder18 => "invalid_dob_age_under_18",
            InvalidDobAgeUnderMinimum => "invalid_dob_age_under_minimum",
            InvalidProductDescriptionLength => "invalid_product_description_length",
            InvalidProductDescriptionUrlMatch => "invalid_product_description_url_match",
            InvalidRepresentativeCountry => "invalid_representative_country",
            InvalidSignator => "invalid_signator",
            InvalidStatementDescriptorBusinessMismatch => {
                "invalid_statement_descriptor_business_mismatch"
            }
            InvalidStatementDescriptorDenylisted => "invalid_statement_descriptor_denylisted",
            InvalidStatementDescriptorLength => "invalid_statement_descriptor_length",
            InvalidStatementDescriptorPrefixDenylisted => {
                "invalid_statement_descriptor_prefix_denylisted"
            }
            InvalidStatementDescriptorPrefixMismatch => {
                "invalid_statement_descriptor_prefix_mismatch"
            }
            InvalidStreetAddress => "invalid_street_address",
            InvalidTaxId => "invalid_tax_id",
            InvalidTaxIdFormat => "invalid_tax_id_format",
            InvalidTosAcceptance => "invalid_tos_acceptance",
            InvalidUrlDenylisted => "invalid_url_denylisted",
            InvalidUrlFormat => "invalid_url_format",
            InvalidUrlLength => "invalid_url_length",
            InvalidUrlWebPresenceDetected => "invalid_url_web_presence_detected",
            InvalidUrlWebsiteBusinessInformationMismatch => {
                "invalid_url_website_business_information_mismatch"
            }
            InvalidUrlWebsiteEmpty => "invalid_url_website_empty",
            InvalidUrlWebsiteInaccessible => "invalid_url_website_inaccessible",
            InvalidUrlWebsiteInaccessibleGeoblocked => {
                "invalid_url_website_inaccessible_geoblocked"
            }
            InvalidUrlWebsiteInaccessiblePasswordProtected => {
                "invalid_url_website_inaccessible_password_protected"
            }
            InvalidUrlWebsiteIncomplete => "invalid_url_website_incomplete",
            InvalidUrlWebsiteIncompleteCancellationPolicy => {
                "invalid_url_website_incomplete_cancellation_policy"
            }
            InvalidUrlWebsiteIncompleteCustomerServiceDetails => {
                "invalid_url_website_incomplete_customer_service_details"
            }
            InvalidUrlWebsiteIncompleteLegalRestrictions => {
                "invalid_url_website_incomplete_legal_restrictions"
            }
            InvalidUrlWebsiteIncompleteRefundPolicy => {
                "invalid_url_website_incomplete_refund_policy"
            }
            InvalidUrlWebsiteIncompleteReturnPolicy => {
                "invalid_url_website_incomplete_return_policy"
            }
            InvalidUrlWebsiteIncompleteTermsAndConditions => {
                "invalid_url_website_incomplete_terms_and_conditions"
            }
            InvalidUrlWebsiteIncompleteUnderConstruction => {
                "invalid_url_website_incomplete_under_construction"
            }
            InvalidUrlWebsiteOther => "invalid_url_website_other",
            InvalidValueOther => "invalid_value_other",
            VerificationDirectorsMismatch => "verification_directors_mismatch",
            VerificationDocumentAddressMismatch => "verification_document_address_mismatch",
            VerificationDocumentAddressMissing => "verification_document_address_missing",
            VerificationDocumentCorrupt => "verification_document_corrupt",
            VerificationDocumentCountryNotSupported => {
                "verification_document_country_not_supported"
            }
            VerificationDocumentDirectorsMismatch => "verification_document_directors_mismatch",
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
            VerificationExtraneousDirectors => "verification_extraneous_directors",
            VerificationFailedAddressMatch => "verification_failed_address_match",
            VerificationFailedAuthorizerAuthority => "verification_failed_authorizer_authority",
            VerificationFailedBusinessIecNumber => "verification_failed_business_iec_number",
            VerificationFailedDocumentMatch => "verification_failed_document_match",
            VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            VerificationFailedNameMatch => "verification_failed_name_match",
            VerificationFailedOther => "verification_failed_other",
            VerificationFailedRepresentativeAuthority => {
                "verification_failed_representative_authority"
            }
            VerificationFailedResidentialAddress => "verification_failed_residential_address",
            VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
            VerificationLegalEntityStructureMismatch => {
                "verification_legal_entity_structure_mismatch"
            }
            VerificationMissingDirectors => "verification_missing_directors",
            VerificationMissingExecutives => "verification_missing_executives",
            VerificationMissingOwners => "verification_missing_owners",
            VerificationRejectedOwnershipExemptionReason => {
                "verification_rejected_ownership_exemption_reason"
            }
            VerificationRequiresAdditionalMemorandumOfAssociations => {
                "verification_requires_additional_memorandum_of_associations"
            }
            VerificationRequiresAdditionalProofOfRegistration => {
                "verification_requires_additional_proof_of_registration"
            }
            VerificationSupportability => "verification_supportability",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountRequirementsErrorCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountRequirementsErrorCode::*;
        match s {
            "information_missing" => Ok(InformationMissing),
            "invalid_address_city_state_postal_code" => Ok(InvalidAddressCityStatePostalCode),
            "invalid_address_highway_contract_box" => Ok(InvalidAddressHighwayContractBox),
            "invalid_address_private_mailbox" => Ok(InvalidAddressPrivateMailbox),
            "invalid_business_profile_name" => Ok(InvalidBusinessProfileName),
            "invalid_business_profile_name_denylisted" => Ok(InvalidBusinessProfileNameDenylisted),
            "invalid_company_name_denylisted" => Ok(InvalidCompanyNameDenylisted),
            "invalid_dob_age_over_maximum" => Ok(InvalidDobAgeOverMaximum),
            "invalid_dob_age_under_18" => Ok(InvalidDobAgeUnder18),
            "invalid_dob_age_under_minimum" => Ok(InvalidDobAgeUnderMinimum),
            "invalid_product_description_length" => Ok(InvalidProductDescriptionLength),
            "invalid_product_description_url_match" => Ok(InvalidProductDescriptionUrlMatch),
            "invalid_representative_country" => Ok(InvalidRepresentativeCountry),
            "invalid_signator" => Ok(InvalidSignator),
            "invalid_statement_descriptor_business_mismatch" => {
                Ok(InvalidStatementDescriptorBusinessMismatch)
            }
            "invalid_statement_descriptor_denylisted" => Ok(InvalidStatementDescriptorDenylisted),
            "invalid_statement_descriptor_length" => Ok(InvalidStatementDescriptorLength),
            "invalid_statement_descriptor_prefix_denylisted" => {
                Ok(InvalidStatementDescriptorPrefixDenylisted)
            }
            "invalid_statement_descriptor_prefix_mismatch" => {
                Ok(InvalidStatementDescriptorPrefixMismatch)
            }
            "invalid_street_address" => Ok(InvalidStreetAddress),
            "invalid_tax_id" => Ok(InvalidTaxId),
            "invalid_tax_id_format" => Ok(InvalidTaxIdFormat),
            "invalid_tos_acceptance" => Ok(InvalidTosAcceptance),
            "invalid_url_denylisted" => Ok(InvalidUrlDenylisted),
            "invalid_url_format" => Ok(InvalidUrlFormat),
            "invalid_url_length" => Ok(InvalidUrlLength),
            "invalid_url_web_presence_detected" => Ok(InvalidUrlWebPresenceDetected),
            "invalid_url_website_business_information_mismatch" => {
                Ok(InvalidUrlWebsiteBusinessInformationMismatch)
            }
            "invalid_url_website_empty" => Ok(InvalidUrlWebsiteEmpty),
            "invalid_url_website_inaccessible" => Ok(InvalidUrlWebsiteInaccessible),
            "invalid_url_website_inaccessible_geoblocked" => {
                Ok(InvalidUrlWebsiteInaccessibleGeoblocked)
            }
            "invalid_url_website_inaccessible_password_protected" => {
                Ok(InvalidUrlWebsiteInaccessiblePasswordProtected)
            }
            "invalid_url_website_incomplete" => Ok(InvalidUrlWebsiteIncomplete),
            "invalid_url_website_incomplete_cancellation_policy" => {
                Ok(InvalidUrlWebsiteIncompleteCancellationPolicy)
            }
            "invalid_url_website_incomplete_customer_service_details" => {
                Ok(InvalidUrlWebsiteIncompleteCustomerServiceDetails)
            }
            "invalid_url_website_incomplete_legal_restrictions" => {
                Ok(InvalidUrlWebsiteIncompleteLegalRestrictions)
            }
            "invalid_url_website_incomplete_refund_policy" => {
                Ok(InvalidUrlWebsiteIncompleteRefundPolicy)
            }
            "invalid_url_website_incomplete_return_policy" => {
                Ok(InvalidUrlWebsiteIncompleteReturnPolicy)
            }
            "invalid_url_website_incomplete_terms_and_conditions" => {
                Ok(InvalidUrlWebsiteIncompleteTermsAndConditions)
            }
            "invalid_url_website_incomplete_under_construction" => {
                Ok(InvalidUrlWebsiteIncompleteUnderConstruction)
            }
            "invalid_url_website_other" => Ok(InvalidUrlWebsiteOther),
            "invalid_value_other" => Ok(InvalidValueOther),
            "verification_directors_mismatch" => Ok(VerificationDirectorsMismatch),
            "verification_document_address_mismatch" => Ok(VerificationDocumentAddressMismatch),
            "verification_document_address_missing" => Ok(VerificationDocumentAddressMissing),
            "verification_document_corrupt" => Ok(VerificationDocumentCorrupt),
            "verification_document_country_not_supported" => {
                Ok(VerificationDocumentCountryNotSupported)
            }
            "verification_document_directors_mismatch" => Ok(VerificationDocumentDirectorsMismatch),
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
            "verification_extraneous_directors" => Ok(VerificationExtraneousDirectors),
            "verification_failed_address_match" => Ok(VerificationFailedAddressMatch),
            "verification_failed_authorizer_authority" => Ok(VerificationFailedAuthorizerAuthority),
            "verification_failed_business_iec_number" => Ok(VerificationFailedBusinessIecNumber),
            "verification_failed_document_match" => Ok(VerificationFailedDocumentMatch),
            "verification_failed_id_number_match" => Ok(VerificationFailedIdNumberMatch),
            "verification_failed_keyed_identity" => Ok(VerificationFailedKeyedIdentity),
            "verification_failed_keyed_match" => Ok(VerificationFailedKeyedMatch),
            "verification_failed_name_match" => Ok(VerificationFailedNameMatch),
            "verification_failed_other" => Ok(VerificationFailedOther),
            "verification_failed_representative_authority" => {
                Ok(VerificationFailedRepresentativeAuthority)
            }
            "verification_failed_residential_address" => Ok(VerificationFailedResidentialAddress),
            "verification_failed_tax_id_match" => Ok(VerificationFailedTaxIdMatch),
            "verification_failed_tax_id_not_issued" => Ok(VerificationFailedTaxIdNotIssued),
            "verification_legal_entity_structure_mismatch" => {
                Ok(VerificationLegalEntityStructureMismatch)
            }
            "verification_missing_directors" => Ok(VerificationMissingDirectors),
            "verification_missing_executives" => Ok(VerificationMissingExecutives),
            "verification_missing_owners" => Ok(VerificationMissingOwners),
            "verification_rejected_ownership_exemption_reason" => {
                Ok(VerificationRejectedOwnershipExemptionReason)
            }
            "verification_requires_additional_memorandum_of_associations" => {
                Ok(VerificationRequiresAdditionalMemorandumOfAssociations)
            }
            "verification_requires_additional_proof_of_registration" => {
                Ok(VerificationRequiresAdditionalProofOfRegistration)
            }
            "verification_supportability" => Ok(VerificationSupportability),
            v => Ok(Unknown(v.to_owned())),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountRequirementsErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountRequirementsErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountRequirementsErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountRequirementsErrorCode::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountRequirementsErrorCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountRequirementsErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
