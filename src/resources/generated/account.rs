// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::AccountId;
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Address, BankAccount, BusinessType, Card, Currency, DelayDays, Dob, File, Person,
    PersonVerificationParams, VerificationDocumentParams, Weekday,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Account".
///
/// For more details see [https://stripe.com/docs/api/accounts/object](https://stripe.com/docs/api/accounts/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    /// Unique identifier for the object.
    pub id: AccountId,

    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,

    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AccountCapabilities>,

    /// Whether the account can create live charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,

    /// The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Whether account details have been submitted.
    ///
    /// Standard accounts cannot receive payouts before this is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,

    /// The primary user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// External accounts (bank accounts and debit cards) currently attached to this account.
    #[serde(default)]
    pub external_accounts: List<ExternalAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Person>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Whether Stripe can send payouts to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountRequirements>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TosAcceptance>,

    /// The Stripe account type.
    ///
    /// Can be `standard`, `express`, or `custom`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<AccountType>,
}

impl Account {
    /// Returns a list of accounts connected to your platform via [Connect](https://stripe.com/docs/connect).
    ///
    /// If you’re not a platform, the list is empty.
    pub fn list(client: &Client, params: ListAccounts<'_>) -> Response<List<Account>> {
        client.get_query("/accounts", &params)
    }

    /// With [Connect](https://stripe.com/docs/connect), you can create Stripe accounts for your users.
    /// To do this, you’ll first need to [register your platform](https://dashboard.stripe.com/account/applications/settings).
    pub fn create(client: &Client, params: CreateAccount<'_>) -> Response<Account> {
        client.post_form("/accounts", &params)
    }

    /// Retrieves the details of an account.
    pub fn retrieve(client: &Client, id: &AccountId, expand: &[&str]) -> Response<Account> {
        client.get_query(&format!("/accounts/{}", id), &Expand { expand })
    }

    /// Updates a connected [Express or Custom account](https://stripe.com/docs/connect/accounts) by setting the values of the parameters passed.
    ///
    /// Any parameters not provided are left unchanged.
    /// Most parameters can be changed only for Custom accounts.
    /// (These are marked **Custom Only** below.) Parameters marked **Custom and Express** are supported by both account types.  To update your own account, use the [Dashboard](https://dashboard.stripe.com/account).
    /// Refer to our [Connect](https://stripe.com/docs/connect/updating-accounts) documentation to learn more about updating accounts.
    pub fn update(client: &Client, id: &AccountId, params: UpdateAccount<'_>) -> Response<Account> {
        client.post_form(&format!("/accounts/{}", id), &params)
    }

    /// With [Connect](https://stripe.com/docs/connect), you can delete Custom or Express accounts you manage.
    ///
    /// Accounts created using test-mode keys can be deleted at any time.
    ///
    /// Accounts created using live-mode keys can only be deleted once all balances are zero.  If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/account) instead.
    pub fn delete(client: &Client, id: &AccountId) -> Response<Deleted<AccountId>> {
        client.delete(&format!("/accounts/{}", id))
    }
}

impl Object for Account {
    type Id = AccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "account"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BusinessProfile {
    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,

    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Internal-only description of the product sold or service provided by the business.
    ///
    /// It's used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<Address>,

    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,

    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,

    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,

    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCapabilities {
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<CapabilityStatus>,

    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CapabilityStatus>,

    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CapabilityStatus>,

    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<CapabilityStatus>,

    /// The status of the legacy payments capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CapabilityStatus>,

    /// The status of the tax reporting 1099-K (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<CapabilityStatus>,

    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<CapabilityStatus>,

    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CapabilityStatus>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirements {
    /// The date the fields in `currently_due` must be collected by to keep payouts enabled for the account.
    ///
    /// These fields might block payouts sooner if the next threshold is reached before these fields are collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<Timestamp>,

    /// The fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by the `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Vec<String>>,

    /// If the account is disabled, this string describes why the account can’t create charges or receive payouts.
    ///
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `rejected.fraud`, `rejected.terms_of_service`, `rejected.listed`, `rejected.other`, `listed`, `under_review`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,

    /// The fields that need to be collected again because validation or verification failed for some reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AccountRequirementsError>>,

    /// The fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, these fields appear in `currently_due` as well, and the `current_deadline` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventually_due: Option<Vec<String>>,

    /// The fields that weren't collected by the `current_deadline`.
    ///
    /// These fields need to be collected to re-enable the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<String>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// An empty array unless an asynchronous verification is pending.
    /// If verification fails, the fields in this array become required and move to `currently_due` or `past_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_verification: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirementsError {
    /// The code for the type of error.
    pub code: AccountRequirementsErrorCode,

    /// An informative message that indicates the error type and provides additional details about the error.
    pub reason: String,

    /// The specific user onboarding requirement field (in the requirements hash) that needs to be resolved.
    pub requirement: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettings {
    pub branding: BrandingSettings,

    pub card_payments: CardPaymentsSettings,

    pub dashboard: DashboardSettings,

    pub payments: PaymentsSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutSettings>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Expandable<File>>,

    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,

    /// A CSS hex color value representing the secondary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOn>,

    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DashboardSettings {
    /// The display name for this account.
    ///
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The timezone used in the Stripe Dashboard for this account.
    ///
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOn {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    ///
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    pub avs_failure: bool,

    /// Whether Stripe automatically declines charges with an incorrect CVC.
    ///
    /// This setting only applies when a CVC is provided and it fails bank verification.
    pub cvc_failure: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettings {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<String>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettings {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details.
    /// Default value is `true` for Express accounts and `false` for Custom accounts.
    pub debit_negative_balances: bool,

    pub schedule: TransferSchedule,

    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TosAcceptance {
    /// The Unix timestamp marking when the Stripe Services Agreement was accepted by the account representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the Stripe Services Agreement was accepted by the account representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the Stripe Services Agreement was accepted by the account representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Company {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    /// Whether the company's directors have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,

    /// Whether the company's executives have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,

    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,

    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,

    /// Whether the company's owners have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,

    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CompanyStructure>,

    /// Whether the company's business ID number was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_provided: Option<bool>,

    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,

    /// Whether the company's business VAT number was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id_provided: Option<bool>,

    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CompanyVerification>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerification {
    pub document: CompanyVerificationDocument,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<Expandable<File>>,

    /// A user-displayable string describing the verification state of this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    ///
    /// A machine-readable code specifying the verification state for this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<Expandable<File>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferSchedule {
    /// The number of days charges for the account will be held before being paid out.
    pub delay_days: u32,

    /// How frequently funds will be paid out.
    ///
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: String,

    /// The day of the month funds will be paid out.
    ///
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,

    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    ///
    /// Only shown if `interval` is weekly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<Weekday>,
}

/// The parameters for `Account::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateAccount<'a> {
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,

    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,

    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,

    /// Information about the company or business.
    ///
    /// This field is null unless `business_type` is set to `company`, `government_entity`, or `non_profit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    /// The country in which the account holder resides, or in which the business is legally established.
    ///
    /// This should be an ISO 3166-1 alpha-2 country code.
    /// For example, if you are in the United States and the business for which you're creating an account is legally represented in Canada, you would use `CA` as the country for the account being created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    /// The email address of the account holder.
    ///
    /// For Custom accounts, this is only to make the account easier to identify to you: Stripe will never directly email your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A card or bank account to attach to the account.
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/stripe.js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the bank account or card creation API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The set of capabilities you want to unlock for this account.
    ///
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_capabilities: Option<Vec<RequestedCapability>>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettingsParams>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AcceptTos>,

    /// The type of Stripe account to create.
    ///
    /// Currently must be `custom`, as only [Custom accounts](https://stripe.com/docs/connect/custom-accounts) may be created via the API.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<AccountType>,
}

impl<'a> CreateAccount<'a> {
    pub fn new() -> Self {
        CreateAccount {
            account_token: Default::default(),
            business_profile: Default::default(),
            business_type: Default::default(),
            company: Default::default(),
            country: Default::default(),
            default_currency: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            external_account: Default::default(),
            individual: Default::default(),
            metadata: Default::default(),
            requested_capabilities: Default::default(),
            settings: Default::default(),
            tos_acceptance: Default::default(),
            type_: Default::default(),
        }
    }
}

/// The parameters for `Account::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListAccounts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<AccountId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<AccountId>,
}

impl<'a> ListAccounts<'a> {
    pub fn new() -> Self {
        ListAccounts {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `Account::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateAccount<'a> {
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,

    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,

    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,

    /// Information about the company or business.
    ///
    /// This field is null unless `business_type` is set to `company`, `government_entity`, or `non_profit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    /// Email address of the account representative.
    ///
    /// For Standard accounts, this is used to ask them to claim their Stripe account.
    /// For Custom accounts, this only makes the account easier to identify to platforms; Stripe does not email the account representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A card or bank account to attach to the account.
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/stripe.js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the bank account or card creation API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The set of capabilities you want to unlock for this account.
    ///
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_capabilities: Option<Vec<RequestedCapability>>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettingsParams>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AcceptTos>,
}

impl<'a> UpdateAccount<'a> {
    pub fn new() -> Self {
        UpdateAccount {
            account_token: Default::default(),
            business_profile: Default::default(),
            business_type: Default::default(),
            company: Default::default(),
            default_currency: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            external_account: Default::default(),
            individual: Default::default(),
            metadata: Default::default(),
            requested_capabilities: Default::default(),
            settings: Default::default(),
            tos_acceptance: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AcceptTos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<BrandingSettingsParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CardPaymentsSettingsParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsSettingsParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutSettingsParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CompanyParamsStructure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CompanyVerificationParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<Dob>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOnParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TransferScheduleParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOnParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<DelayDays>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<TransferScheduleInterval>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<Weekday>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(BankAccount),
    Card(Card),
}

/// An enum representing the possible values of an `AccountRequirementsError`'s `code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountRequirementsErrorCode {
    InvalidAddressCityStatePostalCode,
    InvalidStreetAddress,
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
    VerificationDocumentManipulated,
    VerificationDocumentMissingBack,
    VerificationDocumentMissingFront,
    VerificationDocumentNameMismatch,
    VerificationDocumentNameMissing,
    VerificationDocumentNationalityMismatch,
    VerificationDocumentNotReadable,
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
}

impl AccountRequirementsErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountRequirementsErrorCode::InvalidAddressCityStatePostalCode => {
                "invalid_address_city_state_postal_code"
            }
            AccountRequirementsErrorCode::InvalidStreetAddress => "invalid_street_address",
            AccountRequirementsErrorCode::InvalidValueOther => "invalid_value_other",
            AccountRequirementsErrorCode::VerificationDocumentAddressMismatch => {
                "verification_document_address_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentAddressMissing => {
                "verification_document_address_missing"
            }
            AccountRequirementsErrorCode::VerificationDocumentCorrupt => {
                "verification_document_corrupt"
            }
            AccountRequirementsErrorCode::VerificationDocumentCountryNotSupported => {
                "verification_document_country_not_supported"
            }
            AccountRequirementsErrorCode::VerificationDocumentDobMismatch => {
                "verification_document_dob_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentDuplicateType => {
                "verification_document_duplicate_type"
            }
            AccountRequirementsErrorCode::VerificationDocumentExpired => {
                "verification_document_expired"
            }
            AccountRequirementsErrorCode::VerificationDocumentFailedCopy => {
                "verification_document_failed_copy"
            }
            AccountRequirementsErrorCode::VerificationDocumentFailedGreyscale => {
                "verification_document_failed_greyscale"
            }
            AccountRequirementsErrorCode::VerificationDocumentFailedOther => {
                "verification_document_failed_other"
            }
            AccountRequirementsErrorCode::VerificationDocumentFailedTestMode => {
                "verification_document_failed_test_mode"
            }
            AccountRequirementsErrorCode::VerificationDocumentFraudulent => {
                "verification_document_fraudulent"
            }
            AccountRequirementsErrorCode::VerificationDocumentIdNumberMismatch => {
                "verification_document_id_number_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentIdNumberMissing => {
                "verification_document_id_number_missing"
            }
            AccountRequirementsErrorCode::VerificationDocumentIncomplete => {
                "verification_document_incomplete"
            }
            AccountRequirementsErrorCode::VerificationDocumentInvalid => {
                "verification_document_invalid"
            }
            AccountRequirementsErrorCode::VerificationDocumentManipulated => {
                "verification_document_manipulated"
            }
            AccountRequirementsErrorCode::VerificationDocumentMissingBack => {
                "verification_document_missing_back"
            }
            AccountRequirementsErrorCode::VerificationDocumentMissingFront => {
                "verification_document_missing_front"
            }
            AccountRequirementsErrorCode::VerificationDocumentNameMismatch => {
                "verification_document_name_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentNameMissing => {
                "verification_document_name_missing"
            }
            AccountRequirementsErrorCode::VerificationDocumentNationalityMismatch => {
                "verification_document_nationality_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentNotReadable => {
                "verification_document_not_readable"
            }
            AccountRequirementsErrorCode::VerificationDocumentNotUploaded => {
                "verification_document_not_uploaded"
            }
            AccountRequirementsErrorCode::VerificationDocumentPhotoMismatch => {
                "verification_document_photo_mismatch"
            }
            AccountRequirementsErrorCode::VerificationDocumentTooLarge => {
                "verification_document_too_large"
            }
            AccountRequirementsErrorCode::VerificationDocumentTypeNotSupported => {
                "verification_document_type_not_supported"
            }
            AccountRequirementsErrorCode::VerificationFailedAddressMatch => {
                "verification_failed_address_match"
            }
            AccountRequirementsErrorCode::VerificationFailedBusinessIecNumber => {
                "verification_failed_business_iec_number"
            }
            AccountRequirementsErrorCode::VerificationFailedDocumentMatch => {
                "verification_failed_document_match"
            }
            AccountRequirementsErrorCode::VerificationFailedIdNumberMatch => {
                "verification_failed_id_number_match"
            }
            AccountRequirementsErrorCode::VerificationFailedKeyedIdentity => {
                "verification_failed_keyed_identity"
            }
            AccountRequirementsErrorCode::VerificationFailedKeyedMatch => {
                "verification_failed_keyed_match"
            }
            AccountRequirementsErrorCode::VerificationFailedNameMatch => {
                "verification_failed_name_match"
            }
            AccountRequirementsErrorCode::VerificationFailedOther => "verification_failed_other",
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

/// An enum representing the possible values of an `CreateAccount`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Custom,
    Express,
    Standard,
}

impl AccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountType::Custom => "custom",
            AccountType::Express => "express",
            AccountType::Standard => "standard",
        }
    }
}

impl AsRef<str> for AccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `au_becs_debit_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Active,
    Inactive,
    Pending,
}

impl CapabilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CapabilityStatus::Active => "active",
            CapabilityStatus::Inactive => "inactive",
            CapabilityStatus::Pending => "pending",
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

/// An enum representing the possible values of an `CompanyParams`'s `structure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompanyParamsStructure {
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    LimitedLiabilityPartnership,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
}

impl CompanyParamsStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            CompanyParamsStructure::GovernmentInstrumentality => "government_instrumentality",
            CompanyParamsStructure::GovernmentalUnit => "governmental_unit",
            CompanyParamsStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CompanyParamsStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CompanyParamsStructure::MultiMemberLlc => "multi_member_llc",
            CompanyParamsStructure::PrivateCompany => "private_company",
            CompanyParamsStructure::PrivateCorporation => "private_corporation",
            CompanyParamsStructure::PrivatePartnership => "private_partnership",
            CompanyParamsStructure::PublicCompany => "public_company",
            CompanyParamsStructure::PublicCorporation => "public_corporation",
            CompanyParamsStructure::PublicPartnership => "public_partnership",
            CompanyParamsStructure::SoleProprietorship => "sole_proprietorship",
            CompanyParamsStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            CompanyParamsStructure::UnincorporatedAssociation => "unincorporated_association",
            CompanyParamsStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
        }
    }
}

impl AsRef<str> for CompanyParamsStructure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CompanyParamsStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Company`'s `structure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompanyStructure {
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    LimitedLiabilityPartnership,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
}

impl CompanyStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            CompanyStructure::GovernmentInstrumentality => "government_instrumentality",
            CompanyStructure::GovernmentalUnit => "governmental_unit",
            CompanyStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CompanyStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CompanyStructure::MultiMemberLlc => "multi_member_llc",
            CompanyStructure::PrivateCompany => "private_company",
            CompanyStructure::PrivateCorporation => "private_corporation",
            CompanyStructure::PrivatePartnership => "private_partnership",
            CompanyStructure::PublicCompany => "public_company",
            CompanyStructure::PublicCorporation => "public_corporation",
            CompanyStructure::PublicPartnership => "public_partnership",
            CompanyStructure::SoleProprietorship => "sole_proprietorship",
            CompanyStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            CompanyStructure::UnincorporatedAssociation => "unincorporated_association",
            CompanyStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
        }
    }
}

impl AsRef<str> for CompanyStructure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateAccount`'s `requested_capabilities` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RequestedCapability {
    AuBecsDebitPayments,
    CardIssuing,
    CardPayments,
    JcbPayments,
    LegacyPayments,
    #[serde(rename = "tax_reporting_us_1099_k")]
    TaxReportingUs1099K,
    #[serde(rename = "tax_reporting_us_1099_misc")]
    TaxReportingUs1099Misc,
    Transfers,
}

impl RequestedCapability {
    pub fn as_str(self) -> &'static str {
        match self {
            RequestedCapability::AuBecsDebitPayments => "au_becs_debit_payments",
            RequestedCapability::CardIssuing => "card_issuing",
            RequestedCapability::CardPayments => "card_payments",
            RequestedCapability::JcbPayments => "jcb_payments",
            RequestedCapability::LegacyPayments => "legacy_payments",
            RequestedCapability::TaxReportingUs1099K => "tax_reporting_us_1099_k",
            RequestedCapability::TaxReportingUs1099Misc => "tax_reporting_us_1099_misc",
            RequestedCapability::Transfers => "transfers",
        }
    }
}

impl AsRef<str> for RequestedCapability {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RequestedCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `TransferScheduleParams`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransferScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
}

impl TransferScheduleInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            TransferScheduleInterval::Daily => "daily",
            TransferScheduleInterval::Manual => "manual",
            TransferScheduleInterval::Monthly => "monthly",
            TransferScheduleInterval::Weekly => "weekly",
        }
    }
}

impl AsRef<str> for TransferScheduleInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransferScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
