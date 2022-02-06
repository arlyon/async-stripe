// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::params::{Deleted, Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Address, BusinessProfile, Currency, DelayDays, PersonVerificationParams,
    VerificationDocumentParams,
};
use crate::Account;

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

// written at 597
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

// written at 597
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

// written at 597
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

// written at 1030
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

// written at 1030
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

// written at 1030
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

// written at 1030
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

// written at 1030
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

// written at 1030
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

// written at 1030
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

// written at 1030
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

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<Box<AccountSettingsParamsCardIssuingTosAcceptance>>,
}

// written at 1030
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

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<Box<DeclineChargeOnParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyParamsOwnershipDeclaration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentParams>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAcssDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAfterpayClearpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesAuBecsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBacsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBancontactPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesBoletoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCardPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesCartesBancairesPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesEpsPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesFpxPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGiropayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesGrabpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesIdealPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesJcbPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesKlarnaPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesLegacyPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesOxxoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesP24Payments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSepaDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesSofortPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099K {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099Misc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountCapabilitiesTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsBankAccountOwnershipVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyLicense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMemorandumOfAssociation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyMinisterialDecree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyRegistrationVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsCompanyTaxIdVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountDocumentsProofOfRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<TransferScheduleParams>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonParamsDob {
    pub day: i64,

    pub month: i64,

    pub year: i64,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAcssDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAfterpayClearpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesAuBecsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBacsDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBancontactPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesBoletoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCardPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesCartesBancairesPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesEpsPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesFpxPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGiropayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesGrabpayPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesIdealPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesJcbPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesKlarnaPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesLegacyPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesOxxoPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesP24Payments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSepaDebitPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesSofortPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099K {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountCapabilitiesTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Box<bool>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsBankAccountOwnershipVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyLicense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMemorandumOfAssociation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyMinisterialDecree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyRegistrationVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsCompanyTaxIdVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccountDocumentsProofOfRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Box<Vec<String>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettingsParamsCardIssuingTosAcceptance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOnParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<Box<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<Box<bool>>,
}

// written at 1030
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

/// An enum representing the possible values of an `CreateAccount`'s `business_type` field.
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
