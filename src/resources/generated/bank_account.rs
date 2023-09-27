// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::BankAccountId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Account, BankAccountStatus, Currency, Customer};

/// The resource representing a Stripe "BankAccount".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankAccount {
    /// Unique identifier for the object.
    pub id: BankAccountId,

    /// The ID of the account that the bank account is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Expandable<Account>>,

    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,

    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<String>,

    /// The bank account type.
    ///
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,

    /// A set of available payout methods for this bank account.
    ///
    /// Only values from this set should be passed as the `method` when creating a payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<BankAccountAvailablePayoutMethods>>,

    /// Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: Currency,

    /// The ID of the customer that the bank account is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// Whether this bank account is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Information about the [upcoming new requirements for the bank account](https://stripe.com/docs/connect/custom-accounts/future-requirements), including what information needs to be collected, and by when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<ExternalAccountRequirements>,

    /// The last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Information about the requirements for the bank account, including what information needs to be collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<ExternalAccountRequirements>,

    /// The routing transit number for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,

    /// For bank accounts, possible values are `new`, `validated`, `verified`, `verification_failed`, or `errored`.
    ///
    /// A bank account that hasn't had any activity or validation performed is `new`.
    /// If Stripe can determine that the bank account exists, its status will be `validated`.
    /// Note that there often isn’t enough information to know (e.g., for smaller credit unions), and the validation is not always run.
    /// If customer bank account verification has succeeded, the bank account status will be `verified`.
    /// If the verification failed for any reason, such as microdeposit failure, the status will be `verification_failed`.
    /// If a transfer sent to this bank account fails, we'll set the status to `errored` and will not continue to send transfers until the bank details are updated.  For external accounts, possible values are `new`, `errored` and `verification_failed`.
    /// If a transfer fails, the status is set to `errored` and transfers are stopped until account details are updated.
    /// In India, if we can't [verify the owner of the bank account](https://support.stripe.com/questions/bank-account-ownership-verification), we'll set the status to `verification_failed`.
    /// Other validations aren't run against external accounts because they're only used for payouts.
    /// This means the other statuses don't apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BankAccountStatus>,
}

impl Object for BankAccount {
    type Id = BankAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "bank_account"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExternalAccountRequirements {
    /// Fields that need to be collected to keep the external account enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Option<Vec<String>>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<AccountRequirementsError>>,

    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the external account.
    pub past_due: Option<Vec<String>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Option<Vec<String>>,
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

/// An enum representing the possible values of an `AccountRequirementsError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountRequirementsErrorCode {
    InvalidAddressCityStatePostalCode,
    #[serde(rename = "invalid_dob_age_under_18")]
    InvalidDobAgeUnder18,
    InvalidRepresentativeCountry,
    InvalidStreetAddress,
    InvalidTosAcceptance,
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
            AccountRequirementsErrorCode::InvalidDobAgeUnder18 => "invalid_dob_age_under_18",
            AccountRequirementsErrorCode::InvalidRepresentativeCountry => "invalid_representative_country",
            AccountRequirementsErrorCode::InvalidStreetAddress => "invalid_street_address",
            AccountRequirementsErrorCode::InvalidTosAcceptance => "invalid_tos_acceptance",
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

/// An enum representing the possible values of an `BankAccount`'s `available_payout_methods` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountAvailablePayoutMethods {
    Instant,
    Standard,
}

impl BankAccountAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        match self {
            BankAccountAvailablePayoutMethods::Instant => "instant",
            BankAccountAvailablePayoutMethods::Standard => "standard",
        }
    }
}

impl AsRef<str> for BankAccountAvailablePayoutMethods {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankAccountAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BankAccountAvailablePayoutMethods {
    fn default() -> Self {
        Self::Instant
    }
}
