// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::AccountId;
use crate::params::{
    Deleted, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{
    Address, Currency, DelayDays, ExternalAccount, File, Person, PersonVerificationParams, TaxId,
    VerificationDocumentParams,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Account".
///
/// For more details see <https://stripe.com/docs/api/accounts/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Account {
    /// Unique identifier for the object.
    pub id: AccountId,

    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,

    /// The business type.
    ///
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property is only returned for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<AccountBusinessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AccountCapabilities>,

    /// Whether the account can create live charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<AccountUnificationAccountController>,

    /// The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Time at which the account was connected.
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

    /// An email address associated with the account.
    ///
    /// It's not used for authentication and Stripe doesn't market to this field without explicit approval from the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// External accounts (bank accounts and debit cards) currently attached to this account.
    ///
    /// External accounts are only returned for requests where `controller[is_controller]` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<List<ExternalAccount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<AccountFutureRequirements>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Person>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

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
    pub fn list(client: &Client, params: &ListAccounts<'_>) -> Response<List<Account>> {
        client.get_query("/accounts", params)
    }

    /// With [Connect](https://stripe.com/docs/connect), you can create Stripe accounts for your users.
    /// To do this, you’ll first need to [register your platform](https://dashboard.stripe.com/account/applications/settings).
    ///
    /// If you’ve already collected information for your connected accounts, you [can prefill that information](https://stripe.com/docs/connect/best-practices#onboarding) when
    /// creating the account.
    ///
    /// Connect Onboarding won’t ask for the prefilled information during account onboarding. You can prefill any information on the account.
    pub fn create(client: &Client, params: CreateAccount<'_>) -> Response<Account> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/accounts", &params)
    }

    /// Retrieves the details of an account.
    pub fn retrieve(client: &Client, id: &AccountId, expand: &[&str]) -> Response<Account> {
        client.get_query(&format!("/accounts/{}", id), Expand { expand })
    }

    /// Updates a [connected account](https://stripe.com/docs/connect/accounts) by setting the values of the parameters passed.
    ///
    /// Any parameters not provided are left unchanged.  For Custom accounts, you can update any information on the account.
    /// For other accounts, you can update all information until that account has started to go through Connect Onboarding.
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), some properties can only be changed or updated for Custom accounts.  To update your own account, use the [Dashboard](https://dashboard.stripe.com/settings/account).
    /// Refer to our [Connect](https://stripe.com/docs/connect/updating-accounts) documentation to learn more about updating accounts.
    pub fn update(client: &Client, id: &AccountId, params: UpdateAccount<'_>) -> Response<Account> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/accounts/{}", id), &params)
    }

    /// With [Connect](https://stripe.com/docs/connect), you can delete accounts you manage.
    ///
    /// Accounts created using test-mode keys can be deleted at any time.
    ///
    /// Standard accounts created using live-mode keys cannot be deleted.
    /// Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.  If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/settings/account) instead.
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BusinessProfile {
    /// The applicant's gross annual revenue for its preceding fiscal year.
    pub annual_revenue: Option<AccountAnnualRevenue>,

    /// An estimated upper bound of employees, contractors, vendors, etc.
    ///
    /// currently working for the business.
    pub estimated_worker_count: Option<u64>,

    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    pub mcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue: Option<AccountMonthlyEstimatedRevenue>,

    /// The customer-facing business name.
    pub name: Option<String>,

    /// Internal-only description of the product sold or service provided by the business.
    ///
    /// It's used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// A publicly available mailing address for sending support issues to.
    pub support_address: Option<Address>,

    /// A publicly available email address for sending support issues to.
    pub support_email: Option<String>,

    /// A publicly available phone number to call with support issues.
    pub support_phone: Option<String>,

    /// A publicly available website for handling support issues.
    pub support_url: Option<String>,

    /// The business's publicly available website.
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountAnnualRevenue {
    /// A non-negative integer representing the amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<Currency>,

    /// The close-out date of the preceding fiscal year in ISO 8601 format.
    ///
    /// E.g.
    /// 2023-12-31 for the 31st of December, 2023.
    pub fiscal_year_end: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<AccountCapabilitiesAcssDebitPayments>,

    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<AccountCapabilitiesAffirmPayments>,

    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<AccountCapabilitiesAfterpayClearpayPayments>,

    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<CapabilityStatus>,

    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<AccountCapabilitiesBacsDebitPayments>,

    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<AccountCapabilitiesBancontactPayments>,

    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<AccountCapabilitiesBankTransferPayments>,

    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<AccountCapabilitiesBlikPayments>,

    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<AccountCapabilitiesBoletoPayments>,

    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CapabilityStatus>,

    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CapabilityStatus>,

    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<AccountCapabilitiesCartesBancairesPayments>,

    /// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<AccountCapabilitiesCashappPayments>,

    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<AccountCapabilitiesEpsPayments>,

    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<AccountCapabilitiesFpxPayments>,

    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<AccountCapabilitiesGiropayPayments>,

    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<AccountCapabilitiesGrabpayPayments>,

    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<AccountCapabilitiesIdealPayments>,

    /// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<AccountCapabilitiesIndiaInternationalPayments>,

    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<CapabilityStatus>,

    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<AccountCapabilitiesKlarnaPayments>,

    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<AccountCapabilitiesKonbiniPayments>,

    /// The status of the legacy payments capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CapabilityStatus>,

    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<AccountCapabilitiesLinkPayments>,

    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<AccountCapabilitiesOxxoPayments>,

    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<AccountCapabilitiesP24Payments>,

    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<AccountCapabilitiesPaynowPayments>,

    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<AccountCapabilitiesPromptpayPayments>,

    /// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<AccountCapabilitiesRevolutPayPayments>,

    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<AccountCapabilitiesSepaDebitPayments>,

    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<AccountCapabilitiesSofortPayments>,

    /// The status of the Swish capability of the account, or whether the account can directly process Swish payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_payments: Option<AccountCapabilitiesSwishPayments>,

    /// The status of the tax reporting 1099-K (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<CapabilityStatus>,

    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<CapabilityStatus>,

    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CapabilityStatus>,

    /// The status of the banking capability, or whether the account can have bank accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<AccountCapabilitiesTreasury>,

    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<AccountCapabilitiesUsBankAccountAchPayments>,

    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<AccountCapabilitiesZipPayments>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountFutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,

    /// Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty.
    ///
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning.
    pub current_deadline: Option<Timestamp>,

    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Option<Vec<String>>,

    /// This is typed as a string for consistency with `requirements.disabled_reason`.
    pub disabled_reason: Option<String>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<AccountRequirementsError>>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Option<Vec<String>>,

    /// Fields that weren't collected by `requirements.current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Option<Vec<String>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountMonthlyEstimatedRevenue {
    /// A non-negative integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,

    /// Date by which the fields in `currently_due` must be collected to keep the account enabled.
    ///
    /// These fields may disable the account sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<Timestamp>,

    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Option<Vec<String>>,

    /// If the account is disabled, this string describes why.
    ///
    /// [Learn more about handling verification issues](https://stripe.com/docs/connect/handling-api-verification).
    /// Can be `action_required.requested_capabilities`, `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.incomplete_verification`, `rejected.listed`, `rejected.other`, `rejected.terms_of_service`, `under_review`, or `other`.
    pub disabled_reason: Option<String>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<AccountRequirementsError>>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Option<Vec<String>>,

    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the account.
    pub past_due: Option<Vec<String>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Option<Vec<String>>,
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
pub struct AccountSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<AccountBacsDebitPaymentsSettings>,

    pub branding: BrandingSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<AccountCardIssuingSettings>,

    pub card_payments: CardPaymentsSettings,

    pub dashboard: DashboardSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<AccountInvoicesSettings>,

    pub payments: PaymentsSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<AccountSepaDebitPaymentsSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<AccountTreasurySettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountBacsDebitPaymentsSettings {
    /// The Bacs Direct Debit display name for this account.
    ///
    /// For payments made with Bacs Direct Debit, this name appears on the mandate as the statement descriptor.
    /// Mobile banking apps display it as the name of the business.
    /// To use custom branding, set the Bacs Direct Debit Display Name during or right after creation.
    /// Custom branding incurs an additional monthly fee for the platform.
    /// The fee appears 5 business days after requesting Bacs.
    /// If you don't set the display name before requesting Bacs capability, it's automatically set as "Stripe" and the account is onboarded to Stripe branding, which is free.
    pub display_name: Option<String>,

    /// The Bacs Direct Debit Service user number for this account.
    ///
    /// For payments made with Bacs Direct Debit, this number is a unique identifier of the account with our banking partners.
    pub service_user_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    pub icon: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    pub logo: Option<Expandable<File>>,

    /// A CSS hex color value representing the primary branding color for this account.
    pub primary_color: Option<String>,

    /// A CSS hex color value representing the secondary branding color for this account.
    pub secondary_color: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountCardIssuingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<CardIssuingAccountTermsOfService>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardPaymentsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOn>,

    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix: Option<String>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kana: Option<String>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kanji: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DashboardSettings {
    /// The display name for this account.
    ///
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    pub display_name: Option<String>,

    /// The timezone used in the Stripe Dashboard for this account.
    ///
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    pub timezone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountInvoicesSettings {
    /// The list of default Account Tax IDs to automatically include on invoices.
    ///
    /// Account Tax IDs get added when an invoice is finalized.
    pub default_account_tax_ids: Option<Vec<Expandable<TaxId>>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentsSettings {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    pub statement_descriptor: Option<String>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kana: Option<String>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    pub statement_descriptor_kanji: Option<String>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kana: Option<String>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    pub statement_descriptor_prefix_kanji: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSepaDebitPaymentsSettings {
    /// SEPA creditor identifier that identifies the company making the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TosAcceptance {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user's service agreement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<String>,

    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountTreasurySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AccountTermsOfService>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountTermsOfService {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    pub date: Option<Timestamp>,

    /// The IP address from which the account representative accepted the service agreement.
    pub ip: Option<String>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountUnificationAccountController {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    ///
    /// Otherwise, this field is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_controller: Option<bool>,

    /// The controller type.
    ///
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[serde(rename = "type")]
    pub type_: AccountUnificationAccountControllerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardIssuingAccountTermsOfService {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    pub date: Option<Timestamp>,

    /// The IP address from which the account representative accepted the service agreement.
    pub ip: Option<String>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

    /// The export license ID number of the company, also referred as Import Export Code (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<String>,

    /// The purpose code to use for export transactions (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<String>,

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

    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<LegalEntityUboDeclaration>,

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompanyVerification {
    pub document: CompanyVerificationDocument,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub back: Option<Expandable<File>>,

    /// A user-displayable string describing the verification state of this document.
    pub details: Option<String>,

    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    ///
    /// A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub front: Option<Expandable<File>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LegalEntityUboDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    pub date: Option<Timestamp>,

    /// The IP address from which the beneficial owner attestation was made.
    pub ip: Option<String>,

    /// The user-agent string from the browser where the beneficial owner attestation was made.
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    pub weekly_anchor: Option<String>,
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
    ///
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<AccountBusinessType>,

    /// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
    ///
    /// whether it has been requested or not).
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CreateAccountCapabilities>,

    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    /// The country in which the account holder resides, or in which the business is legally established.
    ///
    /// This should be an ISO 3166-1 alpha-2 country code.
    /// For example, if you are in the United States and the business for which you're creating an account is legally represented in Canada, you would use `CA` as the country for the account being created.
    /// Available countries include [Stripe's global markets](https://stripe.com/global) as well as countries where [cross-border payouts](https://stripe.com/docs/connect/cross-border-payouts) are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateAccountDocuments>,

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
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs.
    /// After you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettingsParams>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance) This property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AcceptTos>,

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
impl Paginable for ListAccounts<'_> {
    type O = Account;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
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
    ///
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<AccountBusinessType>,

    /// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
    ///
    /// whether it has been requested or not).
    /// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
    /// An account may have some of its requested capabilities be active and some be inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<UpdateAccountCapabilities>,

    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<UpdateAccountDocuments>,

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
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs.
    /// After you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    /// Once you create an [Account Link](https://stripe.com/docs/api/account_links) or [Account Session](https://stripe.com/docs/api/account_sessions), this property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettingsParams>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance) This property can only be updated for Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AcceptTos>,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AcceptTos {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user's service agreement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<String>,

    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSettingsParams {
    /// Settings specific to Bacs Direct Debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<AccountSettingsParamsBacsDebitPayments>,

    /// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<BrandingSettingsParams>,

    /// Settings specific to the account's use of the Card Issuing product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<AccountSettingsParamsCardIssuing>,

    /// Settings specific to card charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CardPaymentsSettingsParams>,

    /// Settings that apply across payment methods for charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsSettingsParams>,

    /// Settings specific to the account's payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutSettingsParams>,

    /// Settings specific to the account's Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<AccountSettingsParamsTreasury>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompanyParams {
    /// The company's primary address.
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
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,

    /// Whether the company's executives have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,

    /// The export license ID number of the company, also referred as Import Export Code (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<String>,

    /// The purpose code to use for export transactions (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<String>,

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
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,

    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CompanyParamsOwnershipDeclaration>,

    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    ///
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,

    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    /// Pass an empty string to unset this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CompanyParamsStructure>,

    /// The business ID number of the company, as appropriate for the company’s country.
    ///
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,

    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,

    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CompanyVerificationParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilities {
    /// The acss_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<CreateAccountCapabilitiesAcssDebitPayments>,

    /// The affirm_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<CreateAccountCapabilitiesAffirmPayments>,

    /// The afterpay_clearpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<CreateAccountCapabilitiesAfterpayClearpayPayments>,

    /// The au_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<CreateAccountCapabilitiesAuBecsDebitPayments>,

    /// The bacs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<CreateAccountCapabilitiesBacsDebitPayments>,

    /// The bancontact_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<CreateAccountCapabilitiesBancontactPayments>,

    /// The bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<CreateAccountCapabilitiesBankTransferPayments>,

    /// The blik_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<CreateAccountCapabilitiesBlikPayments>,

    /// The boleto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<CreateAccountCapabilitiesBoletoPayments>,

    /// The card_issuing capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CreateAccountCapabilitiesCardIssuing>,

    /// The card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CreateAccountCapabilitiesCardPayments>,

    /// The cartes_bancaires_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<CreateAccountCapabilitiesCartesBancairesPayments>,

    /// The cashapp_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<CreateAccountCapabilitiesCashappPayments>,

    /// The eps_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<CreateAccountCapabilitiesEpsPayments>,

    /// The fpx_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<CreateAccountCapabilitiesFpxPayments>,

    /// The giropay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<CreateAccountCapabilitiesGiropayPayments>,

    /// The grabpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<CreateAccountCapabilitiesGrabpayPayments>,

    /// The ideal_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<CreateAccountCapabilitiesIdealPayments>,

    /// The india_international_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<CreateAccountCapabilitiesIndiaInternationalPayments>,

    /// The jcb_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<CreateAccountCapabilitiesJcbPayments>,

    /// The klarna_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<CreateAccountCapabilitiesKlarnaPayments>,

    /// The konbini_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<CreateAccountCapabilitiesKonbiniPayments>,

    /// The legacy_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CreateAccountCapabilitiesLegacyPayments>,

    /// The link_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<CreateAccountCapabilitiesLinkPayments>,

    /// The oxxo_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<CreateAccountCapabilitiesOxxoPayments>,

    /// The p24_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<CreateAccountCapabilitiesP24Payments>,

    /// The paynow_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<CreateAccountCapabilitiesPaynowPayments>,

    /// The promptpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<CreateAccountCapabilitiesPromptpayPayments>,

    /// The revolut_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<CreateAccountCapabilitiesRevolutPayPayments>,

    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<CreateAccountCapabilitiesSepaDebitPayments>,

    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<CreateAccountCapabilitiesSofortPayments>,

    /// The swish_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_payments: Option<CreateAccountCapabilitiesSwishPayments>,

    /// The tax_reporting_us_1099_k capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<CreateAccountCapabilitiesTaxReportingUs1099K>,

    /// The tax_reporting_us_1099_misc capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<CreateAccountCapabilitiesTaxReportingUs1099Misc>,

    /// The transfers capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CreateAccountCapabilitiesTransfers>,

    /// The treasury capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CreateAccountCapabilitiesTreasury>,

    /// The us_bank_account_ach_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<CreateAccountCapabilitiesUsBankAccountAchPayments>,

    /// The zip_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<CreateAccountCapabilitiesZipPayments>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocuments {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    ///
    /// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<CreateAccountDocumentsBankAccountOwnershipVerification>,

    /// One or more documents that demonstrate proof of a company's license to operate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<CreateAccountDocumentsCompanyLicense>,

    /// One or more documents showing the company's Memorandum of Association.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association:
        Option<CreateAccountDocumentsCompanyMemorandumOfAssociation>,

    /// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<CreateAccountDocumentsCompanyMinisterialDecree>,

    /// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification:
        Option<CreateAccountDocumentsCompanyRegistrationVerification>,

    /// One or more documents that demonstrate proof of a company's tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<CreateAccountDocumentsCompanyTaxIdVerification>,

    /// One or more documents showing the company’s proof of registration with the national business registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<CreateAccountDocumentsProofOfRegistration>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonParams {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The Kana variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    /// The Kanji variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<PersonParamsDob>,

    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The Kana variation of the the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,

    /// The individual's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    /// The government-issued ID number of the individual, as appropriate for the representative's country.
    ///
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,

    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<PersonParamsPoliticalExposure>,

    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<PersonParamsRegisteredAddress>,

    /// Describes the person’s relationship to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PersonParamsRelationship>,

    /// The last four digits of the individual's Social Security Number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,

    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilities {
    /// The acss_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<UpdateAccountCapabilitiesAcssDebitPayments>,

    /// The affirm_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<UpdateAccountCapabilitiesAffirmPayments>,

    /// The afterpay_clearpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<UpdateAccountCapabilitiesAfterpayClearpayPayments>,

    /// The au_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<UpdateAccountCapabilitiesAuBecsDebitPayments>,

    /// The bacs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<UpdateAccountCapabilitiesBacsDebitPayments>,

    /// The bancontact_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<UpdateAccountCapabilitiesBancontactPayments>,

    /// The bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<UpdateAccountCapabilitiesBankTransferPayments>,

    /// The blik_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<UpdateAccountCapabilitiesBlikPayments>,

    /// The boleto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<UpdateAccountCapabilitiesBoletoPayments>,

    /// The card_issuing capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateAccountCapabilitiesCardIssuing>,

    /// The card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<UpdateAccountCapabilitiesCardPayments>,

    /// The cartes_bancaires_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<UpdateAccountCapabilitiesCartesBancairesPayments>,

    /// The cashapp_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<UpdateAccountCapabilitiesCashappPayments>,

    /// The eps_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<UpdateAccountCapabilitiesEpsPayments>,

    /// The fpx_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<UpdateAccountCapabilitiesFpxPayments>,

    /// The giropay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<UpdateAccountCapabilitiesGiropayPayments>,

    /// The grabpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<UpdateAccountCapabilitiesGrabpayPayments>,

    /// The ideal_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<UpdateAccountCapabilitiesIdealPayments>,

    /// The india_international_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<UpdateAccountCapabilitiesIndiaInternationalPayments>,

    /// The jcb_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<UpdateAccountCapabilitiesJcbPayments>,

    /// The klarna_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<UpdateAccountCapabilitiesKlarnaPayments>,

    /// The konbini_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<UpdateAccountCapabilitiesKonbiniPayments>,

    /// The legacy_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<UpdateAccountCapabilitiesLegacyPayments>,

    /// The link_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<UpdateAccountCapabilitiesLinkPayments>,

    /// The oxxo_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<UpdateAccountCapabilitiesOxxoPayments>,

    /// The p24_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<UpdateAccountCapabilitiesP24Payments>,

    /// The paynow_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<UpdateAccountCapabilitiesPaynowPayments>,

    /// The promptpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<UpdateAccountCapabilitiesPromptpayPayments>,

    /// The revolut_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<UpdateAccountCapabilitiesRevolutPayPayments>,

    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<UpdateAccountCapabilitiesSepaDebitPayments>,

    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<UpdateAccountCapabilitiesSofortPayments>,

    /// The swish_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_payments: Option<UpdateAccountCapabilitiesSwishPayments>,

    /// The tax_reporting_us_1099_k capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<UpdateAccountCapabilitiesTaxReportingUs1099K>,

    /// The tax_reporting_us_1099_misc capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<UpdateAccountCapabilitiesTaxReportingUs1099Misc>,

    /// The transfers capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<UpdateAccountCapabilitiesTransfers>,

    /// The treasury capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<UpdateAccountCapabilitiesTreasury>,

    /// The us_bank_account_ach_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<UpdateAccountCapabilitiesUsBankAccountAchPayments>,

    /// The zip_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<UpdateAccountCapabilitiesZipPayments>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocuments {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    ///
    /// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateAccountDocumentsBankAccountOwnershipVerification>,

    /// One or more documents that demonstrate proof of a company's license to operate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<UpdateAccountDocumentsCompanyLicense>,

    /// One or more documents showing the company's Memorandum of Association.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association:
        Option<UpdateAccountDocumentsCompanyMemorandumOfAssociation>,

    /// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<UpdateAccountDocumentsCompanyMinisterialDecree>,

    /// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification:
        Option<UpdateAccountDocumentsCompanyRegistrationVerification>,

    /// One or more documents that demonstrate proof of a company's tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<UpdateAccountDocumentsCompanyTaxIdVerification>,

    /// One or more documents showing the company’s proof of registration with the national business registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<UpdateAccountDocumentsProofOfRegistration>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSettingsParamsBacsDebitPayments {
    /// The Bacs Direct Debit Display Name for this account.
    ///
    /// For payments made with Bacs Direct Debit, this name appears on the mandate as the statement descriptor.
    /// Mobile banking apps display it as the name of the business.
    /// To use custom branding, set the Bacs Direct Debit Display Name during or right after creation.
    /// Custom branding incurs an additional monthly fee for the platform.
    /// If you don't set the display name before requesting Bacs capability, it's automatically set as "Stripe" and the account is onboarded to Stripe branding, which is free.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuing {
    /// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](https://stripe.com/docs/issuing/connect/tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AccountSettingsParamsCardIssuingTosAcceptance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSettingsParamsTreasury {
    /// Details on the account's acceptance of the Stripe Treasury Services Agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AccountSettingsParamsTreasuryTosAcceptance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BrandingSettingsParams {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,

    /// A CSS hex color value representing the secondary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardPaymentsSettingsParams {
    /// Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOnParams>,

    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kana: Option<String>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kanji: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompanyParamsOwnershipDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompanyVerificationParams {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAcssDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAffirmPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAfterpayClearpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAuBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBacsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBancontactPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBlikPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBoletoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardIssuing {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCartesBancairesPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCashappPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesEpsPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesFpxPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGiropayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGrabpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesIdealPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesIndiaInternationalPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesJcbPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesKlarnaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesKonbiniPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesLegacyPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesLinkPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesOxxoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesP24Payments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesPaynowPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesPromptpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesRevolutPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSepaDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSofortPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSwishPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099K {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099Misc {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTransfers {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTreasury {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesUsBankAccountAchPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesZipPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocumentsBankAccountOwnershipVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyLicense {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMemorandumOfAssociation {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMinisterialDecree {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyRegistrationVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyTaxIdVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountDocumentsProofOfRegistration {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentsSettingsParams {
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayoutSettingsParams {
    /// A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// For details, see [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,

    /// Details on when funds from charges are available, and when they are paid out to an external account.
    ///
    /// For details, see our [Setting Bank and Debit Card Payouts](https://stripe.com/docs/connect/bank-transfers#payout-information) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TransferScheduleParams>,

    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonParamsDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonParamsRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonParamsRelationship {
    /// Whether the person is a director of the account's legal entity.
    ///
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,

    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,

    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,

    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,

    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAcssDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAffirmPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAfterpayClearpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAuBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBacsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBancontactPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBlikPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBoletoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardIssuing {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCartesBancairesPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCashappPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesEpsPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesFpxPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGiropayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGrabpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesIdealPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesIndiaInternationalPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesJcbPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesKlarnaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesKonbiniPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesLegacyPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesLinkPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesOxxoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesP24Payments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesPaynowPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesPromptpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesRevolutPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSepaDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSofortPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSwishPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099K {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTransfers {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTreasury {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesUsBankAccountAchPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesZipPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsBankAccountOwnershipVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyLicense {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMemorandumOfAssociation {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMinisterialDecree {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyRegistrationVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyTaxIdVerification {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsProofOfRegistration {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuingTosAcceptance {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSettingsParamsTreasuryTosAcceptance {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeclineChargeOnParams {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    ///
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,

    /// Whether Stripe automatically declines charges with an incorrect CVC.
    ///
    /// This setting only applies when a CVC is provided and it fails bank verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransferScheduleParams {
    /// The number of days charge funds are held before being paid out.
    ///
    /// May also be set to `minimum`, representing the lowest available value for the account country.
    /// Default is `minimum`.
    /// The `delay_days` parameter remains at the last configured value if `interval` is `manual`.
    /// [Learn more about controlling payout delay days](https://stripe.com/docs/connect/manage-payout-schedule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<DelayDays>,

    /// How frequently available funds are paid out.
    ///
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<TransferScheduleInterval>,

    /// The day of the month when available funds are paid out, specified as a number between 1--31.
    ///
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,

    /// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
    ///
    /// (required and applicable only if `interval` is `weekly`.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<TransferScheduleParamsWeeklyAnchor>,
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
impl std::default::Default for AccountBusinessType {
    fn default() -> Self {
        Self::Company
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
impl std::default::Default for AccountCapabilitiesAcssDebitPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `affirm_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesAffirmPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesAffirmPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesAffirmPayments::Active => "active",
            AccountCapabilitiesAffirmPayments::Inactive => "inactive",
            AccountCapabilitiesAffirmPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesAffirmPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesAffirmPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesAffirmPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesAfterpayClearpayPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesBacsDebitPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesBancontactPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `bank_transfer_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBankTransferPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBankTransferPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesBankTransferPayments::Active => "active",
            AccountCapabilitiesBankTransferPayments::Inactive => "inactive",
            AccountCapabilitiesBankTransferPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBankTransferPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesBankTransferPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `blik_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBlikPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBlikPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesBlikPayments::Active => "active",
            AccountCapabilitiesBlikPayments::Inactive => "inactive",
            AccountCapabilitiesBlikPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBlikPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBlikPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesBlikPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesBoletoPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesCartesBancairesPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `cashapp_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesCashappPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesCashappPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesCashappPayments::Active => "active",
            AccountCapabilitiesCashappPayments::Inactive => "inactive",
            AccountCapabilitiesCashappPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesCashappPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesCashappPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesCashappPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesEpsPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesFpxPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesGiropayPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesGrabpayPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesIdealPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `india_international_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesIndiaInternationalPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesIndiaInternationalPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesIndiaInternationalPayments::Active => "active",
            AccountCapabilitiesIndiaInternationalPayments::Inactive => "inactive",
            AccountCapabilitiesIndiaInternationalPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesIndiaInternationalPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesIndiaInternationalPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesIndiaInternationalPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesKlarnaPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `konbini_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesKonbiniPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesKonbiniPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesKonbiniPayments::Active => "active",
            AccountCapabilitiesKonbiniPayments::Inactive => "inactive",
            AccountCapabilitiesKonbiniPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesKonbiniPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesKonbiniPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesKonbiniPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `link_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesLinkPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesLinkPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesLinkPayments::Active => "active",
            AccountCapabilitiesLinkPayments::Inactive => "inactive",
            AccountCapabilitiesLinkPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesLinkPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesLinkPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesLinkPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesOxxoPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesP24Payments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `paynow_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesPaynowPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesPaynowPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesPaynowPayments::Active => "active",
            AccountCapabilitiesPaynowPayments::Inactive => "inactive",
            AccountCapabilitiesPaynowPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesPaynowPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesPaynowPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesPaynowPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `promptpay_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesPromptpayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesPromptpayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesPromptpayPayments::Active => "active",
            AccountCapabilitiesPromptpayPayments::Inactive => "inactive",
            AccountCapabilitiesPromptpayPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesPromptpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesPromptpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesPromptpayPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `revolut_pay_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesRevolutPayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesRevolutPayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesRevolutPayPayments::Active => "active",
            AccountCapabilitiesRevolutPayPayments::Inactive => "inactive",
            AccountCapabilitiesRevolutPayPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesRevolutPayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesRevolutPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesRevolutPayPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesSepaDebitPayments {
    fn default() -> Self {
        Self::Active
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
impl std::default::Default for AccountCapabilitiesSofortPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `swish_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesSwishPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesSwishPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesSwishPayments::Active => "active",
            AccountCapabilitiesSwishPayments::Inactive => "inactive",
            AccountCapabilitiesSwishPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesSwishPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesSwishPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesSwishPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `treasury` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesTreasury {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesTreasury {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesTreasury::Active => "active",
            AccountCapabilitiesTreasury::Inactive => "inactive",
            AccountCapabilitiesTreasury::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesTreasury {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesTreasury {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `us_bank_account_ach_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesUsBankAccountAchPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesUsBankAccountAchPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesUsBankAccountAchPayments::Active => "active",
            AccountCapabilitiesUsBankAccountAchPayments::Inactive => "inactive",
            AccountCapabilitiesUsBankAccountAchPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesUsBankAccountAchPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesUsBankAccountAchPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesUsBankAccountAchPayments {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `AccountCapabilities`'s `zip_payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesZipPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesZipPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountCapabilitiesZipPayments::Active => "active",
            AccountCapabilitiesZipPayments::Inactive => "inactive",
            AccountCapabilitiesZipPayments::Pending => "pending",
        }
    }
}

impl AsRef<str> for AccountCapabilitiesZipPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesZipPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountCapabilitiesZipPayments {
    fn default() -> Self {
        Self::Active
    }
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
impl std::default::Default for AccountType {
    fn default() -> Self {
        Self::Custom
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
impl std::default::Default for AccountUnificationAccountControllerType {
    fn default() -> Self {
        Self::Account
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
impl std::default::Default for CapabilityStatus {
    fn default() -> Self {
        Self::Active
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
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
}

impl CompanyParamsStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            CompanyParamsStructure::FreeZoneEstablishment => "free_zone_establishment",
            CompanyParamsStructure::FreeZoneLlc => "free_zone_llc",
            CompanyParamsStructure::GovernmentInstrumentality => "government_instrumentality",
            CompanyParamsStructure::GovernmentalUnit => "governmental_unit",
            CompanyParamsStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CompanyParamsStructure::IncorporatedPartnership => "incorporated_partnership",
            CompanyParamsStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CompanyParamsStructure::Llc => "llc",
            CompanyParamsStructure::MultiMemberLlc => "multi_member_llc",
            CompanyParamsStructure::PrivateCompany => "private_company",
            CompanyParamsStructure::PrivateCorporation => "private_corporation",
            CompanyParamsStructure::PrivatePartnership => "private_partnership",
            CompanyParamsStructure::PublicCompany => "public_company",
            CompanyParamsStructure::PublicCorporation => "public_corporation",
            CompanyParamsStructure::PublicPartnership => "public_partnership",
            CompanyParamsStructure::RegisteredCharity => "registered_charity",
            CompanyParamsStructure::SingleMemberLlc => "single_member_llc",
            CompanyParamsStructure::SoleEstablishment => "sole_establishment",
            CompanyParamsStructure::SoleProprietorship => "sole_proprietorship",
            CompanyParamsStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            CompanyParamsStructure::UnincorporatedAssociation => "unincorporated_association",
            CompanyParamsStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
            CompanyParamsStructure::UnincorporatedPartnership => "unincorporated_partnership",
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
impl std::default::Default for CompanyParamsStructure {
    fn default() -> Self {
        Self::FreeZoneEstablishment
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
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
}

impl CompanyStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            CompanyStructure::FreeZoneEstablishment => "free_zone_establishment",
            CompanyStructure::FreeZoneLlc => "free_zone_llc",
            CompanyStructure::GovernmentInstrumentality => "government_instrumentality",
            CompanyStructure::GovernmentalUnit => "governmental_unit",
            CompanyStructure::IncorporatedNonProfit => "incorporated_non_profit",
            CompanyStructure::IncorporatedPartnership => "incorporated_partnership",
            CompanyStructure::LimitedLiabilityPartnership => "limited_liability_partnership",
            CompanyStructure::Llc => "llc",
            CompanyStructure::MultiMemberLlc => "multi_member_llc",
            CompanyStructure::PrivateCompany => "private_company",
            CompanyStructure::PrivateCorporation => "private_corporation",
            CompanyStructure::PrivatePartnership => "private_partnership",
            CompanyStructure::PublicCompany => "public_company",
            CompanyStructure::PublicCorporation => "public_corporation",
            CompanyStructure::PublicPartnership => "public_partnership",
            CompanyStructure::RegisteredCharity => "registered_charity",
            CompanyStructure::SingleMemberLlc => "single_member_llc",
            CompanyStructure::SoleEstablishment => "sole_establishment",
            CompanyStructure::SoleProprietorship => "sole_proprietorship",
            CompanyStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            CompanyStructure::UnincorporatedAssociation => "unincorporated_association",
            CompanyStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
            CompanyStructure::UnincorporatedPartnership => "unincorporated_partnership",
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
impl std::default::Default for CompanyStructure {
    fn default() -> Self {
        Self::FreeZoneEstablishment
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
impl std::default::Default for PersonParamsPoliticalExposure {
    fn default() -> Self {
        Self::Existing
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
impl std::default::Default for TransferScheduleInterval {
    fn default() -> Self {
        Self::Daily
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
impl std::default::Default for TransferScheduleParamsWeeklyAnchor {
    fn default() -> Self {
        Self::Friday
    }
}
