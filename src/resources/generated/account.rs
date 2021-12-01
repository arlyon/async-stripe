// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::AccountId;
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Address, BankAccount, Card, Currency, DelayDays, File, Person, PersonVerificationParams,
    VerificationDocumentParams,
};

/// The resource representing a Stripe "Account".
///
/// For more details see <https://stripe.com/docs/api/accounts/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    /// Unique identifier for the object.
    pub id: AccountId,

    /// Business information about the account.
    pub business_profile: Box<Option<BusinessProfile>>,

    /// The business type.
    pub business_type: Box<Option<AccountBusinessType>>,

    pub capabilities: Box<Option<AccountCapabilities>>,

    /// Whether the account can create live charges.
    pub charges_enabled: Box<Option<bool>>,

    pub company: Box<Option<Company>>,

    pub controller: Box<Option<AccountUnificationAccountController>>,

    /// The account's country.
    pub country: Box<Option<String>>,

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
    pub details_submitted: Box<Option<bool>>,

    /// An email address associated with the account.
    ///
    /// You can treat this as metadata: it is not used for authentication or messaging account holders.
    pub email: Box<Option<String>>,

    /// External accounts (bank accounts and debit cards) currently attached to this account.
    #[serde(default)]
    pub external_accounts: List<ExternalAccount>,

    pub future_requirements: Box<Option<AccountFutureRequirements>>,

    pub individual: Box<Option<Person>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Whether Stripe can send payouts to this account.
    pub payouts_enabled: Box<Option<bool>>,

    pub requirements: Box<Option<AccountRequirements>>,

    /// Options for customizing how the account functions within Stripe.
    pub settings: Box<Option<AccountSettings>>,

    pub tos_acceptance: Box<Option<TosAcceptance>>,

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

    /// Updates a [connected account](https://stripe.com/docs/connect/accounts) by setting the values of the parameters passed.
    ///
    /// Any parameters not provided are left unchanged.
    /// Most parameters can be changed only for Custom accounts.
    /// (These are marked **Custom Only** below.) Parameters marked **Custom and Express** are not supported for Standard accounts.  To update your own account, use the [Dashboard](https://dashboard.stripe.com/account).
    /// Refer to our [Connect](https://stripe.com/docs/connect/updating-accounts) documentation to learn more about updating accounts.
    pub fn update(client: &Client, id: &AccountId, params: UpdateAccount<'_>) -> Response<Account> {
        client.post_form(&format!("/accounts/{}", id), &params)
    }

    /// With [Connect](https://stripe.com/docs/connect), you can delete accounts you manage.
    ///
    /// Accounts created using test-mode keys can be deleted at any time.
    ///
    /// Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.  If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/account) instead.
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
    pub mcc: Box<Option<String>>,

    /// The customer-facing business name.
    pub name: Box<Option<String>>,

    /// Internal-only description of the product sold or service provided by the business.
    ///
    /// It's used by Stripe for risk and underwriting purposes.
    pub product_description: Box<Option<String>>,

    /// A publicly available mailing address for sending support issues to.
    pub support_address: Box<Option<Address>>,

    /// A publicly available email address for sending support issues to.
    pub support_email: Box<Option<String>>,

    /// A publicly available phone number to call with support issues.
    pub support_phone: Box<Option<String>>,

    /// A publicly available website for handling support issues.
    pub support_url: Box<Option<String>>,

    /// The business's publicly available website.
    pub url: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    pub acss_debit_payments: Box<Option<AccountCapabilitiesAcssDebitPayments>>,

    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    pub afterpay_clearpay_payments: Box<Option<AccountCapabilitiesAfterpayClearpayPayments>>,

    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    pub au_becs_debit_payments: Box<Option<CapabilityStatus>>,

    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    pub bacs_debit_payments: Box<Option<AccountCapabilitiesBacsDebitPayments>>,

    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    pub bancontact_payments: Box<Option<AccountCapabilitiesBancontactPayments>>,

    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    pub boleto_payments: Box<Option<AccountCapabilitiesBoletoPayments>>,

    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    pub card_issuing: Box<Option<CapabilityStatus>>,

    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    pub card_payments: Box<Option<CapabilityStatus>>,

    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    pub cartes_bancaires_payments: Box<Option<AccountCapabilitiesCartesBancairesPayments>>,

    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    pub eps_payments: Box<Option<AccountCapabilitiesEpsPayments>>,

    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    pub fpx_payments: Box<Option<AccountCapabilitiesFpxPayments>>,

    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    pub giropay_payments: Box<Option<AccountCapabilitiesGiropayPayments>>,

    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    pub grabpay_payments: Box<Option<AccountCapabilitiesGrabpayPayments>>,

    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    pub ideal_payments: Box<Option<AccountCapabilitiesIdealPayments>>,

    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    pub jcb_payments: Box<Option<CapabilityStatus>>,

    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    pub klarna_payments: Box<Option<AccountCapabilitiesKlarnaPayments>>,

    /// The status of the legacy payments capability of the account.
    pub legacy_payments: Box<Option<CapabilityStatus>>,

    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    pub oxxo_payments: Box<Option<AccountCapabilitiesOxxoPayments>>,

    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    pub p24_payments: Box<Option<AccountCapabilitiesP24Payments>>,

    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    pub sepa_debit_payments: Box<Option<AccountCapabilitiesSepaDebitPayments>>,

    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    pub sofort_payments: Box<Option<AccountCapabilitiesSofortPayments>>,

    /// The status of the tax reporting 1099-K (US) capability of the account.
    pub tax_reporting_us_1099_k: Box<Option<CapabilityStatus>>,

    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    pub tax_reporting_us_1099_misc: Box<Option<CapabilityStatus>>,

    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    pub transfers: Box<Option<CapabilityStatus>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountFutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Box<Option<Vec<AccountRequirementsAlternative>>>,

    /// Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty.
    ///
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning.
    pub current_deadline: Box<Option<Timestamp>>,

    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Box<Option<Vec<String>>>,

    /// This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Box<Option<String>>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Box<Option<Vec<AccountRequirementsError>>>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Box<Option<Vec<String>>>,

    /// Fields that weren't collected by `requirements.current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Box<Option<Vec<String>>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Box<Option<Vec<AccountRequirementsAlternative>>>,

    /// Date by which the fields in `currently_due` must be collected to keep the account enabled.
    ///
    /// These fields may disable the account sooner if the next threshold is reached before they are collected.
    pub current_deadline: Box<Option<Timestamp>>,

    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Box<Option<Vec<String>>>,

    /// If the account is disabled, this string describes why.
    ///
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.
    pub disabled_reason: Box<Option<String>>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Box<Option<Vec<AccountRequirementsError>>>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Box<Option<Vec<String>>>,

    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the account.
    pub past_due: Box<Option<Vec<String>>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirementsAlternative {
    /// Fields that can be provided to satisfy all fields in `original_fields_due`.
    pub alternative_fields_due: Vec<String>,

    /// Fields that are due and can be satisfied by providing all fields in `alternative_fields_due`.
    pub original_fields_due: Vec<String>,
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
    pub bacs_debit_payments: Box<Option<AccountBacsDebitPaymentsSettings>>,

    pub branding: BrandingSettings,

    pub card_issuing: Box<Option<AccountCardIssuingSettings>>,

    pub card_payments: CardPaymentsSettings,

    pub dashboard: DashboardSettings,

    pub payments: PaymentsSettings,

    pub payouts: Box<Option<PayoutSettings>>,

    pub sepa_debit_payments: Box<Option<AccountSepaDebitPaymentsSettings>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountBacsDebitPaymentsSettings {
    /// The Bacs Direct Debit Display Name for this account.
    ///
    /// For payments made with Bacs Direct Debit, this will appear on the mandate, and as the statement descriptor.
    pub display_name: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    pub icon: Box<Option<Expandable<File>>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    pub logo: Box<Option<Expandable<File>>>,

    /// A CSS hex color value representing the primary branding color for this account.
    pub primary_color: Box<Option<String>>,

    /// A CSS hex color value representing the secondary branding color for this account.
    pub secondary_color: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCardIssuingSettings {
    pub tos_acceptance: Box<Option<CardIssuingAccountTermsOfService>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettings {
    pub decline_on: Box<Option<DeclineChargeOn>>,

    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DashboardSettings {
    /// The display name for this account.
    ///
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    pub display_name: Box<Option<String>>,

    /// The timezone used in the Stripe Dashboard for this account.
    ///
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    pub timezone: Box<Option<String>>,
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
    pub statement_descriptor: Box<Option<String>>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kana: Box<Option<String>>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kanji: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettings {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details.
    /// Default value is `false` for Custom accounts, otherwise `true`.
    pub debit_negative_balances: bool,

    pub schedule: TransferSchedule,

    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSepaDebitPaymentsSettings {
    /// SEPA creditor identifier that identifies the company making the payment.
    pub creditor_id: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TosAcceptance {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    pub date: Box<Option<Timestamp>>,

    /// The IP address from which the account representative accepted their service agreement.
    pub ip: Box<Option<String>>,

    /// The user's service agreement type.
    pub service_agreement: Box<Option<String>>,

    /// The user agent of the browser from which the account representative accepted their service agreement.
    pub user_agent: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountUnificationAccountController {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    ///
    /// Otherwise, this field is null.
    pub is_controller: Box<Option<bool>>,

    /// The controller type.
    ///
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[serde(rename = "type")]
    pub type_: AccountUnificationAccountControllerType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardIssuingAccountTermsOfService {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    pub date: Box<Option<Timestamp>>,

    /// The IP address from which the account representative accepted the service agreement.
    pub ip: Box<Option<String>>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    pub user_agent: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Company {
    pub address: Box<Option<Address>>,

    /// The Kana variation of the company's primary address (Japan only).
    pub address_kana: Box<Option<Address>>,

    /// The Kanji variation of the company's primary address (Japan only).
    pub address_kanji: Box<Option<Address>>,

    /// Whether the company's directors have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    pub directors_provided: Box<Option<bool>>,

    /// Whether the company's executives have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided.
    pub executives_provided: Box<Option<bool>>,

    /// The company's legal name.
    pub name: Box<Option<String>>,

    /// The Kana variation of the company's legal name (Japan only).
    pub name_kana: Box<Option<String>>,

    /// The Kanji variation of the company's legal name (Japan only).
    pub name_kanji: Box<Option<String>>,

    /// Whether the company's owners have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    pub owners_provided: Box<Option<bool>>,

    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    pub ownership_declaration: Box<Option<LegalEntityUboDeclaration>>,

    /// The company's phone number (used for verification).
    pub phone: Box<Option<String>>,

    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    pub structure: Box<Option<CompanyStructure>>,

    /// Whether the company's business ID number was provided.
    pub tax_id_provided: Box<Option<bool>>,

    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    pub tax_id_registrar: Box<Option<String>>,

    /// Whether the company's business VAT number was provided.
    pub vat_id_provided: Box<Option<bool>>,

    /// Information on the verification state of the company.
    pub verification: Box<Option<CompanyVerification>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerification {
    pub document: CompanyVerificationDocument,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub back: Box<Option<Expandable<File>>>,

    /// A user-displayable string describing the verification state of this document.
    pub details: Box<Option<String>>,

    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    ///
    /// A machine-readable code specifying the verification state for this document.
    pub details_code: Box<Option<String>>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub front: Box<Option<Expandable<File>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LegalEntityUboDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    pub date: Box<Option<Timestamp>>,

    /// The IP address from which the beneficial owner attestation was made.
    pub ip: Box<Option<String>>,

    /// The user-agent string from the browser where the beneficial owner attestation was made.
    pub user_agent: Box<Option<String>>,
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
    pub monthly_anchor: Box<Option<u8>>,

    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    ///
    /// Only shown if `interval` is weekly.
    pub weekly_anchor: Box<Option<String>>,
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
    pub business_type: Option<AccountBusinessType>,

    /// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
    ///
    /// whether it has been requested or not).
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Box<Option<CreateAccountCapabilities>>,

    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Box<Option<CompanyParams>>,

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

    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Box<Option<CreateAccountDocuments>>,

    /// The email address of the account holder.
    ///
    /// This is only to make the account easier to identify to you.
    /// Stripe only emails Custom accounts with your consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/stripe-js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the bank account or card creation API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Box<Option<PersonParams>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Box<Option<AccountSettingsParams>>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Box<Option<AcceptTos>>,

    /// The type of Stripe account to create.
    ///
    /// May be one of `custom`, `express` or `standard`.
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
            capabilities: Default::default(),
            company: Default::default(),
            country: Default::default(),
            default_currency: Default::default(),
            documents: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            external_account: Default::default(),
            individual: Default::default(),
            metadata: Default::default(),
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
    pub business_type: Option<AccountBusinessType>,

    /// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
    ///
    /// whether it has been requested or not).
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Box<Option<UpdateAccountCapabilities>>,

    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Box<Option<CompanyParams>>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Box<Option<UpdateAccountDocuments>>,

    /// The email address of the account holder.
    ///
    /// This is only to make the account easier to identify to you.
    /// Stripe only emails Custom accounts with your consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/stripe-js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the bank account or card creation API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Box<Option<PersonParams>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Box<Option<AccountSettingsParams>>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Box<Option<AcceptTos>>,
}

impl<'a> UpdateAccount<'a> {
    pub fn new() -> Self {
        UpdateAccount {
            account_token: Default::default(),
            business_profile: Default::default(),
            business_type: Default::default(),
            capabilities: Default::default(),
            company: Default::default(),
            default_currency: Default::default(),
            documents: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            external_account: Default::default(),
            individual: Default::default(),
            metadata: Default::default(),
            settings: Default::default(),
            tos_acceptance: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AcceptTos {
    pub date: Box<Option<Timestamp>>,

    pub ip: Box<Option<String>>,

    pub service_agreement: Box<Option<String>>,

    pub user_agent: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParams {
    pub branding: Box<Option<BrandingSettingsParams>>,

    pub card_issuing: Box<Option<AccountSettingsParamsCardIssuing>>,

    pub card_payments: Box<Option<CardPaymentsSettingsParams>>,

    pub payments: Box<Option<PaymentsSettingsParams>>,

    pub payouts: Box<Option<PayoutSettingsParams>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    pub directors_provided: Box<Option<bool>>,

    pub executives_provided: Box<Option<bool>>,

    pub name: Box<Option<String>>,

    pub name_kana: Box<Option<String>>,

    pub name_kanji: Box<Option<String>>,

    pub owners_provided: Box<Option<bool>>,

    pub ownership_declaration: Box<Option<CompanyParamsOwnershipDeclaration>>,

    pub phone: Box<Option<String>>,

    pub registration_number: Box<Option<String>>,

    pub structure: Box<Option<CompanyParamsStructure>>,

    pub tax_id: Box<Option<String>>,

    pub tax_id_registrar: Box<Option<String>>,

    pub vat_id: Box<Option<String>>,

    pub verification: Box<Option<CompanyVerificationParams>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilities {
    pub acss_debit_payments: Box<Option<CreateAccountCapabilitiesAcssDebitPayments>>,

    pub afterpay_clearpay_payments: Box<Option<CreateAccountCapabilitiesAfterpayClearpayPayments>>,

    pub au_becs_debit_payments: Box<Option<CreateAccountCapabilitiesAuBecsDebitPayments>>,

    pub bacs_debit_payments: Box<Option<CreateAccountCapabilitiesBacsDebitPayments>>,

    pub bancontact_payments: Box<Option<CreateAccountCapabilitiesBancontactPayments>>,

    pub boleto_payments: Box<Option<CreateAccountCapabilitiesBoletoPayments>>,

    pub card_issuing: Box<Option<CreateAccountCapabilitiesCardIssuing>>,

    pub card_payments: Box<Option<CreateAccountCapabilitiesCardPayments>>,

    pub cartes_bancaires_payments: Box<Option<CreateAccountCapabilitiesCartesBancairesPayments>>,

    pub eps_payments: Box<Option<CreateAccountCapabilitiesEpsPayments>>,

    pub fpx_payments: Box<Option<CreateAccountCapabilitiesFpxPayments>>,

    pub giropay_payments: Box<Option<CreateAccountCapabilitiesGiropayPayments>>,

    pub grabpay_payments: Box<Option<CreateAccountCapabilitiesGrabpayPayments>>,

    pub ideal_payments: Box<Option<CreateAccountCapabilitiesIdealPayments>>,

    pub jcb_payments: Box<Option<CreateAccountCapabilitiesJcbPayments>>,

    pub klarna_payments: Box<Option<CreateAccountCapabilitiesKlarnaPayments>>,

    pub legacy_payments: Box<Option<CreateAccountCapabilitiesLegacyPayments>>,

    pub oxxo_payments: Box<Option<CreateAccountCapabilitiesOxxoPayments>>,

    pub p24_payments: Box<Option<CreateAccountCapabilitiesP24Payments>>,

    pub sepa_debit_payments: Box<Option<CreateAccountCapabilitiesSepaDebitPayments>>,

    pub sofort_payments: Box<Option<CreateAccountCapabilitiesSofortPayments>>,

    pub tax_reporting_us_1099_k: Box<Option<CreateAccountCapabilitiesTaxReportingUs1099K>>,

    pub tax_reporting_us_1099_misc: Box<Option<CreateAccountCapabilitiesTaxReportingUs1099Misc>>,

    pub transfers: Box<Option<CreateAccountCapabilitiesTransfers>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocuments {
    pub bank_account_ownership_verification:
        Box<Option<CreateAccountDocumentsBankAccountOwnershipVerification>>,

    pub company_license: Box<Option<CreateAccountDocumentsCompanyLicense>>,

    pub company_memorandum_of_association:
        Box<Option<CreateAccountDocumentsCompanyMemorandumOfAssociation>>,

    pub company_ministerial_decree: Box<Option<CreateAccountDocumentsCompanyMinisterialDecree>>,

    pub company_registration_verification:
        Box<Option<CreateAccountDocumentsCompanyRegistrationVerification>>,

    pub company_tax_id_verification: Box<Option<CreateAccountDocumentsCompanyTaxIdVerification>>,

    pub proof_of_registration: Box<Option<CreateAccountDocumentsProofOfRegistration>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    pub dob: Box<Option<PersonParamsDob>>,

    pub email: Box<Option<String>>,

    pub first_name: Box<Option<String>>,

    pub first_name_kana: Box<Option<String>>,

    pub first_name_kanji: Box<Option<String>>,

    pub full_name_aliases: Box<Option<Vec<String>>>,

    pub gender: Box<Option<String>>,

    pub id_number: Box<Option<String>>,

    pub last_name: Box<Option<String>>,

    pub last_name_kana: Box<Option<String>>,

    pub last_name_kanji: Box<Option<String>>,

    pub maiden_name: Box<Option<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    pub phone: Box<Option<String>>,

    pub political_exposure: Box<Option<PersonParamsPoliticalExposure>>,

    pub ssn_last_4: Box<Option<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilities {
    pub acss_debit_payments: Box<Option<UpdateAccountCapabilitiesAcssDebitPayments>>,

    pub afterpay_clearpay_payments: Box<Option<UpdateAccountCapabilitiesAfterpayClearpayPayments>>,

    pub au_becs_debit_payments: Box<Option<UpdateAccountCapabilitiesAuBecsDebitPayments>>,

    pub bacs_debit_payments: Box<Option<UpdateAccountCapabilitiesBacsDebitPayments>>,

    pub bancontact_payments: Box<Option<UpdateAccountCapabilitiesBancontactPayments>>,

    pub boleto_payments: Box<Option<UpdateAccountCapabilitiesBoletoPayments>>,

    pub card_issuing: Box<Option<UpdateAccountCapabilitiesCardIssuing>>,

    pub card_payments: Box<Option<UpdateAccountCapabilitiesCardPayments>>,

    pub cartes_bancaires_payments: Box<Option<UpdateAccountCapabilitiesCartesBancairesPayments>>,

    pub eps_payments: Box<Option<UpdateAccountCapabilitiesEpsPayments>>,

    pub fpx_payments: Box<Option<UpdateAccountCapabilitiesFpxPayments>>,

    pub giropay_payments: Box<Option<UpdateAccountCapabilitiesGiropayPayments>>,

    pub grabpay_payments: Box<Option<UpdateAccountCapabilitiesGrabpayPayments>>,

    pub ideal_payments: Box<Option<UpdateAccountCapabilitiesIdealPayments>>,

    pub jcb_payments: Box<Option<UpdateAccountCapabilitiesJcbPayments>>,

    pub klarna_payments: Box<Option<UpdateAccountCapabilitiesKlarnaPayments>>,

    pub legacy_payments: Box<Option<UpdateAccountCapabilitiesLegacyPayments>>,

    pub oxxo_payments: Box<Option<UpdateAccountCapabilitiesOxxoPayments>>,

    pub p24_payments: Box<Option<UpdateAccountCapabilitiesP24Payments>>,

    pub sepa_debit_payments: Box<Option<UpdateAccountCapabilitiesSepaDebitPayments>>,

    pub sofort_payments: Box<Option<UpdateAccountCapabilitiesSofortPayments>>,

    pub tax_reporting_us_1099_k: Box<Option<UpdateAccountCapabilitiesTaxReportingUs1099K>>,

    pub tax_reporting_us_1099_misc: Box<Option<UpdateAccountCapabilitiesTaxReportingUs1099Misc>>,

    pub transfers: Box<Option<UpdateAccountCapabilitiesTransfers>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocuments {
    pub bank_account_ownership_verification:
        Box<Option<UpdateAccountDocumentsBankAccountOwnershipVerification>>,

    pub company_license: Box<Option<UpdateAccountDocumentsCompanyLicense>>,

    pub company_memorandum_of_association:
        Box<Option<UpdateAccountDocumentsCompanyMemorandumOfAssociation>>,

    pub company_ministerial_decree: Box<Option<UpdateAccountDocumentsCompanyMinisterialDecree>>,

    pub company_registration_verification:
        Box<Option<UpdateAccountDocumentsCompanyRegistrationVerification>>,

    pub company_tax_id_verification: Box<Option<UpdateAccountDocumentsCompanyTaxIdVerification>>,

    pub proof_of_registration: Box<Option<UpdateAccountDocumentsProofOfRegistration>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuing {
    pub tos_acceptance: Box<Option<AccountSettingsParamsCardIssuingTosAcceptance>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettingsParams {
    pub icon: Box<Option<String>>,

    pub logo: Box<Option<String>>,

    pub primary_color: Box<Option<String>>,

    pub secondary_color: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettingsParams {
    pub decline_on: Box<Option<DeclineChargeOnParams>>,

    pub statement_descriptor_prefix: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyParamsOwnershipDeclaration {
    pub date: Box<Option<Timestamp>>,

    pub ip: Box<Option<String>>,

    pub user_agent: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAcssDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAfterpayClearpayPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAuBecsDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBacsDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBancontactPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBoletoPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardIssuing {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCartesBancairesPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesEpsPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesFpxPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGiropayPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGrabpayPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesIdealPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesJcbPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesKlarnaPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesLegacyPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesOxxoPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesP24Payments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSepaDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSofortPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099K {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099Misc {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTransfers {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsBankAccountOwnershipVerification {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyLicense {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMemorandumOfAssociation {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMinisterialDecree {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyRegistrationVerification {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyTaxIdVerification {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsProofOfRegistration {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettingsParams {
    pub statement_descriptor: Box<Option<String>>,

    pub statement_descriptor_kana: Box<Option<String>>,

    pub statement_descriptor_kanji: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettingsParams {
    pub debit_negative_balances: Box<Option<bool>>,

    pub schedule: Box<Option<TransferScheduleParams>>,

    pub statement_descriptor: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonParamsDob {
    pub day: i64,

    pub month: i64,

    pub year: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAcssDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAfterpayClearpayPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAuBecsDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBacsDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBancontactPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBoletoPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardIssuing {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCartesBancairesPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesEpsPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesFpxPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGiropayPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGrabpayPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesIdealPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesJcbPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesKlarnaPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesLegacyPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesOxxoPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesP24Payments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSepaDebitPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSofortPayments {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099K {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTransfers {
    pub requested: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsBankAccountOwnershipVerification {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyLicense {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMemorandumOfAssociation {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMinisterialDecree {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyRegistrationVerification {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyTaxIdVerification {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsProofOfRegistration {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuingTosAcceptance {
    pub date: Box<Option<Timestamp>>,

    pub ip: Box<Option<String>>,

    pub user_agent: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOnParams {
    pub avs_failure: Box<Option<bool>>,

    pub cvc_failure: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<DelayDays>,

    pub interval: Box<Option<TransferScheduleInterval>>,

    pub monthly_anchor: Box<Option<u8>>,

    pub weekly_anchor: Box<Option<TransferScheduleParamsWeeklyAnchor>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(BankAccount),
    Card(Card),
}

/// An enum representing the possible values of an `Account`'s `business_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl AccountBusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountBusinessType::Company => "company",
            AccountBusinessType::GovernmentEntity => "government_entity",
            AccountBusinessType::Individual => "individual",
            AccountBusinessType::NonProfit => "non_profit",
        }
    }
}

impl AsRef<str> for AccountBusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `acss_debit_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesAcssDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesAcssDebitPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesAcssDebitPayments::Active => "active",
            AccountCapabilitiesAcssDebitPayments::Inactive => "inactive",
            AccountCapabilitiesAcssDebitPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesAcssDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesAcssDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `afterpay_clearpay_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesAfterpayClearpayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesAfterpayClearpayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesAfterpayClearpayPayments::Active => "active",
            AccountCapabilitiesAfterpayClearpayPayments::Inactive => "inactive",
            AccountCapabilitiesAfterpayClearpayPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesAfterpayClearpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesAfterpayClearpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `bacs_debit_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBacsDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBacsDebitPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesBacsDebitPayments::Active => "active",
            AccountCapabilitiesBacsDebitPayments::Inactive => "inactive",
            AccountCapabilitiesBacsDebitPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBacsDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBacsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `bancontact_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBancontactPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBancontactPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesBancontactPayments::Active => "active",
            AccountCapabilitiesBancontactPayments::Inactive => "inactive",
            AccountCapabilitiesBancontactPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBancontactPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBancontactPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `boleto_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBoletoPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBoletoPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesBoletoPayments::Active => "active",
            AccountCapabilitiesBoletoPayments::Inactive => "inactive",
            AccountCapabilitiesBoletoPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBoletoPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBoletoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `cartes_bancaires_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesCartesBancairesPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesCartesBancairesPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesCartesBancairesPayments::Active => "active",
            AccountCapabilitiesCartesBancairesPayments::Inactive => "inactive",
            AccountCapabilitiesCartesBancairesPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesCartesBancairesPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesCartesBancairesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `eps_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesEpsPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesEpsPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesEpsPayments::Active => "active",
            AccountCapabilitiesEpsPayments::Inactive => "inactive",
            AccountCapabilitiesEpsPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesEpsPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesEpsPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `fpx_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesFpxPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesFpxPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesFpxPayments::Active => "active",
            AccountCapabilitiesFpxPayments::Inactive => "inactive",
            AccountCapabilitiesFpxPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesFpxPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesFpxPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `giropay_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesGiropayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesGiropayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesGiropayPayments::Active => "active",
            AccountCapabilitiesGiropayPayments::Inactive => "inactive",
            AccountCapabilitiesGiropayPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesGiropayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesGiropayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `grabpay_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesGrabpayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesGrabpayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesGrabpayPayments::Active => "active",
            AccountCapabilitiesGrabpayPayments::Inactive => "inactive",
            AccountCapabilitiesGrabpayPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesGrabpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesGrabpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `ideal_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesIdealPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesIdealPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesIdealPayments::Active => "active",
            AccountCapabilitiesIdealPayments::Inactive => "inactive",
            AccountCapabilitiesIdealPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesIdealPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesIdealPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `klarna_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesKlarnaPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesKlarnaPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesKlarnaPayments::Active => "active",
            AccountCapabilitiesKlarnaPayments::Inactive => "inactive",
            AccountCapabilitiesKlarnaPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesKlarnaPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesKlarnaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `oxxo_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesOxxoPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesOxxoPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesOxxoPayments::Active => "active",
            AccountCapabilitiesOxxoPayments::Inactive => "inactive",
            AccountCapabilitiesOxxoPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesOxxoPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesOxxoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `p24_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesP24Payments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesP24Payments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesP24Payments::Active => "active",
            AccountCapabilitiesP24Payments::Inactive => "inactive",
            AccountCapabilitiesP24Payments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesP24Payments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesP24Payments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `sepa_debit_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesSepaDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesSepaDebitPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesSepaDebitPayments::Active => "active",
            AccountCapabilitiesSepaDebitPayments::Inactive => "inactive",
            AccountCapabilitiesSepaDebitPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesSepaDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesSepaDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `sofort_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesSofortPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesSofortPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesSofortPayments::Active => "active",
            AccountCapabilitiesSofortPayments::Inactive => "inactive",
            AccountCapabilitiesSofortPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesSofortPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesSofortPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
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
    VerificationFailedTaxIdMatch,
    VerificationFailedTaxIdNotIssued,
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRequiresAdditionalMemorandumOfAssociations,
}

impl AccountRequirementsErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountRequirementsErrorCode::InvalidAddressCityStatePostalCode => "invalid_address_city_state_postal_code",
            AccountRequirementsErrorCode::InvalidStreetAddress => "invalid_street_address",
            AccountRequirementsErrorCode::InvalidValueOther => "invalid_value_other",
            AccountRequirementsErrorCode::VerificationDocumentAddressMismatch => "verification_document_address_mismatch",
            AccountRequirementsErrorCode::VerificationDocumentAddressMissing => "verification_document_address_missing",
            AccountRequirementsErrorCode::VerificationDocumentCorrupt => "verification_document_corrupt",
            AccountRequirementsErrorCode::VerificationDocumentCountryNotSupported => "verification_document_country_not_supported",
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
            AccountRequirementsErrorCode::VerificationFailedAddressMatch => "verification_failed_address_match",
            AccountRequirementsErrorCode::VerificationFailedBusinessIecNumber => "verification_failed_business_iec_number",
            AccountRequirementsErrorCode::VerificationFailedDocumentMatch => "verification_failed_document_match",
            AccountRequirementsErrorCode::VerificationFailedIdNumberMatch => "verification_failed_id_number_match",
            AccountRequirementsErrorCode::VerificationFailedKeyedIdentity => "verification_failed_keyed_identity",
            AccountRequirementsErrorCode::VerificationFailedKeyedMatch => "verification_failed_keyed_match",
            AccountRequirementsErrorCode::VerificationFailedNameMatch => "verification_failed_name_match",
            AccountRequirementsErrorCode::VerificationFailedOther => "verification_failed_other",
            AccountRequirementsErrorCode::VerificationFailedTaxIdMatch => "verification_failed_tax_id_match",
            AccountRequirementsErrorCode::VerificationFailedTaxIdNotIssued => "verification_failed_tax_id_not_issued",
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

/// An enum representing the possible values of an `AccountUnificationAccountController`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
}

impl AccountUnificationAccountControllerType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountUnificationAccountControllerType::Account => "account",
            AccountUnificationAccountControllerType::Application => "application",
        }
    }
}

impl AsRef<str> for AccountUnificationAccountControllerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountUnificationAccountControllerType {
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
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
}

impl CompanyParamsStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            CompanyParamsStructure::FreeZoneEstablishment => "free_zone_establishment",
            CompanyParamsStructure::FreeZoneLlc => "free_zone_llc",
            CompanyParamsStructure::GovernmentInstrumentality => "government_instrumentality",
            CompanyParamsStructure::GovernmentalUnit => "governmental_unit",
            CompanyParamsStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CompanyParamsStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CompanyParamsStructure::Llc => "llc",
            CompanyParamsStructure::MultiMemberLlc => "multi_member_llc",
            CompanyParamsStructure::PrivateCompany => "private_company",
            CompanyParamsStructure::PrivateCorporation => "private_corporation",
            CompanyParamsStructure::PrivatePartnership => "private_partnership",
            CompanyParamsStructure::PublicCompany => "public_company",
            CompanyParamsStructure::PublicCorporation => "public_corporation",
            CompanyParamsStructure::PublicPartnership => "public_partnership",
            CompanyParamsStructure::SingleMemberLlc => "single_member_llc",
            CompanyParamsStructure::SoleEstablishment => "sole_establishment",
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
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
}

impl CompanyStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            CompanyStructure::FreeZoneEstablishment => "free_zone_establishment",
            CompanyStructure::FreeZoneLlc => "free_zone_llc",
            CompanyStructure::GovernmentInstrumentality => "government_instrumentality",
            CompanyStructure::GovernmentalUnit => "governmental_unit",
            CompanyStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CompanyStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CompanyStructure::Llc => "llc",
            CompanyStructure::MultiMemberLlc => "multi_member_llc",
            CompanyStructure::PrivateCompany => "private_company",
            CompanyStructure::PrivateCorporation => "private_corporation",
            CompanyStructure::PrivatePartnership => "private_partnership",
            CompanyStructure::PublicCompany => "public_company",
            CompanyStructure::PublicCorporation => "public_corporation",
            CompanyStructure::PublicPartnership => "public_partnership",
            CompanyStructure::SingleMemberLlc => "single_member_llc",
            CompanyStructure::SoleEstablishment => "sole_establishment",
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

/// An enum representing the possible values of an `PersonParams`'s `political_exposure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PersonParamsPoliticalExposure {
    Existing,
    None,
}

impl PersonParamsPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        match self {
            PersonParamsPoliticalExposure::Existing => "existing",
            PersonParamsPoliticalExposure::None => "none",
        }
    }
}

impl AsRef<str> for PersonParamsPoliticalExposure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PersonParamsPoliticalExposure {
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

/// An enum representing the possible values of an `TransferScheduleParams`'s `weekly_anchor` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransferScheduleParamsWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}

impl TransferScheduleParamsWeeklyAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            TransferScheduleParamsWeeklyAnchor::Friday => "friday",
            TransferScheduleParamsWeeklyAnchor::Monday => "monday",
            TransferScheduleParamsWeeklyAnchor::Saturday => "saturday",
            TransferScheduleParamsWeeklyAnchor::Sunday => "sunday",
            TransferScheduleParamsWeeklyAnchor::Thursday => "thursday",
            TransferScheduleParamsWeeklyAnchor::Tuesday => "tuesday",
            TransferScheduleParamsWeeklyAnchor::Wednesday => "wednesday",
        }
    }
}

impl AsRef<str> for TransferScheduleParamsWeeklyAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransferScheduleParamsWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
