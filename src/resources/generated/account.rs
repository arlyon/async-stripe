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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<Box<BusinessProfile>>,

    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<Box<AccountBusinessType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<AccountCapabilities>>,

    /// Whether the account can create live charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Box<Company>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<Box<AccountUnificationAccountController>>,

    /// The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

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
    pub details_submitted: Option<Box<bool>>,

    /// An email address associated with the account.
    ///
    /// You can treat this as metadata: it is not used for authentication or messaging account holders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    /// External accounts (bank accounts and debit cards) currently attached to this account.
    #[serde(default)]
    pub external_accounts: List<ExternalAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<Box<AccountFutureRequirements>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Box<Person>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Whether Stripe can send payouts to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Box<AccountRequirements>>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<AccountSettings>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<Box<TosAcceptance>>,

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
    /// Standard accounts created using live-mode keys cannot be deleted.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Box<String>>,

    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    /// Internal-only description of the product sold or service provided by the business.
    ///
    /// It's used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<Box<String>>,

    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<Box<Address>>,

    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<Box<String>>,

    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<Box<String>>,

    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<Box<String>>,

    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<Box<AccountCapabilitiesAcssDebitPayments>>,

    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<Box<AccountCapabilitiesAfterpayClearpayPayments>>,

    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<Box<CapabilityStatus>>,

    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<Box<AccountCapabilitiesBacsDebitPayments>>,

    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<Box<AccountCapabilitiesBancontactPayments>>,

    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<Box<AccountCapabilitiesBoletoPayments>>,

    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<Box<CapabilityStatus>>,

    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<Box<CapabilityStatus>>,

    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<Box<AccountCapabilitiesCartesBancairesPayments>>,

    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<Box<AccountCapabilitiesEpsPayments>>,

    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<Box<AccountCapabilitiesFpxPayments>>,

    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<Box<AccountCapabilitiesGiropayPayments>>,

    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<Box<AccountCapabilitiesGrabpayPayments>>,

    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<Box<AccountCapabilitiesIdealPayments>>,

    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<Box<CapabilityStatus>>,

    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<Box<AccountCapabilitiesKlarnaPayments>>,

    /// The status of the legacy payments capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<Box<CapabilityStatus>>,

    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<Box<AccountCapabilitiesOxxoPayments>>,

    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<Box<AccountCapabilitiesP24Payments>>,

    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<Box<AccountCapabilitiesSepaDebitPayments>>,

    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<Box<AccountCapabilitiesSofortPayments>>,

    /// The status of the tax reporting 1099-K (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<Box<CapabilityStatus>>,

    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<Box<CapabilityStatus>>,

    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Box<CapabilityStatus>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountFutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Box<Vec<AccountRequirementsAlternative>>>,

    /// Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty.
    ///
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<Box<Timestamp>>,

    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Box<Vec<String>>>,

    /// This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<Box<String>>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<Vec<AccountRequirementsError>>>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventually_due: Option<Box<Vec<String>>>,

    /// Fields that weren't collected by `requirements.current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Box<Vec<String>>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_verification: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Box<Vec<AccountRequirementsAlternative>>>,

    /// Date by which the fields in `currently_due` must be collected to keep the account enabled.
    ///
    /// These fields may disable the account sooner if the next threshold is reached before they are collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<Box<Timestamp>>,

    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Box<Vec<String>>>,

    /// If the account is disabled, this string describes why.
    ///
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<Box<String>>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<Vec<AccountRequirementsError>>>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventually_due: Option<Box<Vec<String>>>,

    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Box<Vec<String>>>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_verification: Option<Box<Vec<String>>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<Box<AccountBacsDebitPaymentsSettings>>,

    pub branding: BrandingSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<Box<AccountCardIssuingSettings>>,

    pub card_payments: CardPaymentsSettings,

    pub dashboard: DashboardSettings,

    pub payments: PaymentsSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<Box<PayoutSettings>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<Box<AccountSepaDebitPaymentsSettings>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountBacsDebitPaymentsSettings {
    /// The Bacs Direct Debit Display Name for this account.
    ///
    /// For payments made with Bacs Direct Debit, this will appear on the mandate, and as the statement descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<Expandable<File>>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Box<Expandable<File>>>,

    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<Box<String>>,

    /// A CSS hex color value representing the secondary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCardIssuingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<Box<CardIssuingAccountTermsOfService>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<Box<DeclineChargeOn>>,

    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DashboardSettings {
    /// The display name for this account.
    ///
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Box<String>>,

    /// The timezone used in the Stripe Dashboard for this account.
    ///
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Box<String>>,
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
    pub statement_descriptor: Option<Box<String>>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<Box<String>>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<Box<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSepaDebitPaymentsSettings {
    /// SEPA creditor identifier that identifies the company making the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_id: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TosAcceptance {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    /// The IP address from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    /// The user's service agreement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<Box<String>>,

    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountUnificationAccountController {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    ///
    /// Otherwise, this field is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_controller: Option<Box<bool>>,

    /// The controller type.
    ///
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[serde(rename = "type")]
    pub type_: AccountUnificationAccountControllerType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardIssuingAccountTermsOfService {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Company {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<Address>>,

    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Box<Address>>,

    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Box<Address>>,

    /// Whether the company's directors have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<Box<bool>>,

    /// Whether the company's executives have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<Box<bool>>,

    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<Box<String>>,

    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<Box<String>>,

    /// Whether the company's owners have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<Box<bool>>,

    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<Box<LegalEntityUboDeclaration>>,

    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Box<String>>,

    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Box<CompanyStructure>>,

    /// Whether the company's business ID number was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_provided: Option<Box<bool>>,

    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<Box<String>>,

    /// Whether the company's business VAT number was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id_provided: Option<Box<bool>>,

    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<CompanyVerification>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerification {
    pub document: CompanyVerificationDocument,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<Box<Expandable<File>>>,

    /// A user-displayable string describing the verification state of this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<String>>,

    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    ///
    /// A machine-readable code specifying the verification state for this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<Box<String>>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<Box<Expandable<File>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LegalEntityUboDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    /// The user-agent string from the browser where the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
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
    pub monthly_anchor: Option<Box<u8>>,

    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    ///
    /// Only shown if `interval` is weekly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<Box<String>>,
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
    pub capabilities: Option<Box<CreateAccountCapabilities>>,

    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Box<CompanyParams>>,

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
    pub documents: Option<Box<CreateAccountDocuments>>,

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
    /// To add additional external accounts without replacing the existing default for the currency, use the bank account or card creation API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Box<PersonParams>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<AccountSettingsParams>>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<Box<AcceptTos>>,

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
    pub capabilities: Option<Box<UpdateAccountCapabilities>>,

    /// Information about the company or business.
    ///
    /// This field is available for any `business_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Box<CompanyParams>>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Box<UpdateAccountDocuments>>,

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
    /// To add additional external accounts without replacing the existing default for the currency, use the bank account or card creation API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Box<PersonParams>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<AccountSettingsParams>>,

    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<Box<AcceptTos>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<Box<BrandingSettingsParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<Box<AccountSettingsParamsCardIssuing>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<Box<CardPaymentsSettingsParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Box<PaymentsSettingsParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<Box<PayoutSettingsParams>>,
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
    pub directors_provided: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<Box<CompanyParamsOwnershipDeclaration>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Box<CompanyParamsStructure>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<CompanyVerificationParams>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<Box<CreateAccountCapabilitiesAcssDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<Box<CreateAccountCapabilitiesAfterpayClearpayPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<Box<CreateAccountCapabilitiesAuBecsDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<Box<CreateAccountCapabilitiesBacsDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<Box<CreateAccountCapabilitiesBancontactPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<Box<CreateAccountCapabilitiesBoletoPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<Box<CreateAccountCapabilitiesCardIssuing>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<Box<CreateAccountCapabilitiesCardPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<Box<CreateAccountCapabilitiesCartesBancairesPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<Box<CreateAccountCapabilitiesEpsPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<Box<CreateAccountCapabilitiesFpxPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<Box<CreateAccountCapabilitiesGiropayPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<Box<CreateAccountCapabilitiesGrabpayPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<Box<CreateAccountCapabilitiesIdealPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<Box<CreateAccountCapabilitiesJcbPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<Box<CreateAccountCapabilitiesKlarnaPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<Box<CreateAccountCapabilitiesLegacyPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<Box<CreateAccountCapabilitiesOxxoPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<Box<CreateAccountCapabilitiesP24Payments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<Box<CreateAccountCapabilitiesSepaDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<Box<CreateAccountCapabilitiesSofortPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<Box<CreateAccountCapabilitiesTaxReportingUs1099K>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<Box<CreateAccountCapabilitiesTaxReportingUs1099Misc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Box<CreateAccountCapabilitiesTransfers>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocuments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<Box<CreateAccountDocumentsBankAccountOwnershipVerification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<Box<CreateAccountDocumentsCompanyLicense>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association:
        Option<Box<CreateAccountDocumentsCompanyMemorandumOfAssociation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<Box<CreateAccountDocumentsCompanyMinisterialDecree>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification:
        Option<Box<CreateAccountDocumentsCompanyRegistrationVerification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<Box<CreateAccountDocumentsCompanyTaxIdVerification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<Box<CreateAccountDocumentsProofOfRegistration>>,
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
    pub dob: Option<Box<PersonParamsDob>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Box<Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<Box<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<Box<PersonParamsPoliticalExposure>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<Box<UpdateAccountCapabilitiesAcssDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<Box<UpdateAccountCapabilitiesAfterpayClearpayPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<Box<UpdateAccountCapabilitiesAuBecsDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<Box<UpdateAccountCapabilitiesBacsDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<Box<UpdateAccountCapabilitiesBancontactPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<Box<UpdateAccountCapabilitiesBoletoPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<Box<UpdateAccountCapabilitiesCardIssuing>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<Box<UpdateAccountCapabilitiesCardPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<Box<UpdateAccountCapabilitiesCartesBancairesPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<Box<UpdateAccountCapabilitiesEpsPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<Box<UpdateAccountCapabilitiesFpxPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<Box<UpdateAccountCapabilitiesGiropayPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<Box<UpdateAccountCapabilitiesGrabpayPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<Box<UpdateAccountCapabilitiesIdealPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<Box<UpdateAccountCapabilitiesJcbPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<Box<UpdateAccountCapabilitiesKlarnaPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<Box<UpdateAccountCapabilitiesLegacyPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<Box<UpdateAccountCapabilitiesOxxoPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<Box<UpdateAccountCapabilitiesP24Payments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<Box<UpdateAccountCapabilitiesSepaDebitPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<Box<UpdateAccountCapabilitiesSofortPayments>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<Box<UpdateAccountCapabilitiesTaxReportingUs1099K>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<Box<UpdateAccountCapabilitiesTaxReportingUs1099Misc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Box<UpdateAccountCapabilitiesTransfers>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocuments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<Box<UpdateAccountDocumentsBankAccountOwnershipVerification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<Box<UpdateAccountDocumentsCompanyLicense>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association:
        Option<Box<UpdateAccountDocumentsCompanyMemorandumOfAssociation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<Box<UpdateAccountDocumentsCompanyMinisterialDecree>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification:
        Option<Box<UpdateAccountDocumentsCompanyRegistrationVerification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<Box<UpdateAccountDocumentsCompanyTaxIdVerification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<Box<UpdateAccountDocumentsProofOfRegistration>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<Box<AccountSettingsParamsCardIssuingTosAcceptance>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<Box<DeclineChargeOnParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyParamsOwnershipDeclaration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAcssDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAfterpayClearpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAuBecsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBacsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBancontactPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBoletoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCartesBancairesPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesEpsPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesFpxPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGiropayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGrabpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesIdealPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesJcbPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesKlarnaPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesLegacyPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesOxxoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesP24Payments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSepaDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSofortPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099K {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099Misc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsBankAccountOwnershipVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyLicense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMemorandumOfAssociation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMinisterialDecree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyRegistrationVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyTaxIdVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsProofOfRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<TransferScheduleParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonParamsDob {
    pub day: i64,

    pub month: i64,

    pub year: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAcssDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAfterpayClearpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAuBecsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBacsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBancontactPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBoletoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCartesBancairesPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesEpsPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesFpxPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGiropayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGrabpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesIdealPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesJcbPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesKlarnaPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesLegacyPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesOxxoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesP24Payments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSepaDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSofortPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099K {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsBankAccountOwnershipVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyLicense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMemorandumOfAssociation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMinisterialDecree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyRegistrationVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyTaxIdVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsProofOfRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuingTosAcceptance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOnParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<Box<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<DelayDays>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Box<TransferScheduleInterval>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<Box<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<Box<TransferScheduleParamsWeeklyAnchor>>,
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
