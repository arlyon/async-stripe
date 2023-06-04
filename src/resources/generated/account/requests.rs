use crate::{Client, Response};

impl crate::account::Account {
    /// Retrieves the details of an account.
    pub fn retrieve_many(
        client: &Client,
        params: RetrieveManyAccount,
    ) -> Response<crate::account::Account> {
        client.get_query("/account", params)
    }
    /// Retrieves the details of an account.
    pub fn retrieve(
        client: &Client,
        account: &crate::account::AccountId,
        params: RetrieveAccount,
    ) -> Response<crate::account::Account> {
        client.get_query(&format!("/accounts/{account}", account = account), params)
    }
    /// Updates a [connected account](https://stripe.com/docs/connect/accounts) by setting the values of the parameters passed.
    ///
    /// Any parameters not provided are left unchanged.
    /// Most parameters can be changed only for Custom accounts.
    /// (These are marked **Custom Only** below.) Parameters marked **Custom and Express** are not supported for Standard accounts.  To update your own account, use the [Dashboard](https://dashboard.stripe.com/account).
    /// Refer to our [Connect](https://stripe.com/docs/connect/updating-accounts) documentation to learn more about updating accounts.
    pub fn update(
        client: &Client,
        account: &crate::account::AccountId,
        params: UpdateAccount,
    ) -> Response<crate::account::Account> {
        client.send_form(
            &format!("/accounts/{account}", account = account),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of accounts connected to your platform via [Connect](https://stripe.com/docs/connect).
    ///
    /// If you’re not a platform, the list is empty.
    pub fn list(
        client: &Client,
        params: ListAccount,
    ) -> Response<crate::List<crate::account::Account>> {
        client.get_query("/accounts", params)
    }
    /// With [Connect](https://stripe.com/docs/connect), you can create Stripe accounts for your users.
    /// To do this, you’ll first need to [register your platform](https://dashboard.stripe.com/account/applications/settings).
    pub fn create(client: &Client, params: CreateAccount) -> Response<crate::account::Account> {
        client.send_form("/accounts", params, http_types::Method::Post)
    }
    /// With [Connect](https://stripe.com/docs/connect), you can delete accounts you manage.
    ///
    /// Accounts created using test-mode keys can be deleted at any time.
    ///
    /// Standard accounts created using live-mode keys cannot be deleted.
    /// Custom or Express accounts created using live-mode keys can only be deleted once all balances are zero.  If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/account) instead.
    pub fn delete(
        client: &Client,
        account: &crate::account::AccountId,
    ) -> Response<crate::account::DeletedAccount> {
        client.send(&format!("/accounts/{account}", account = account), http_types::Method::Delete)
    }
    /// With [Connect](https://stripe.com/docs/connect), you may flag accounts as suspicious.
    ///
    /// Test-mode Custom and Express accounts can be rejected at any time.
    ///
    /// Accounts created using live-mode keys may only be rejected once all balances are zero.
    pub fn reject(
        client: &Client,
        account: &crate::account::AccountId,
        params: RejectAccount,
    ) -> Response<crate::account::Account> {
        client.send_form(
            &format!("/accounts/{account}/reject", account = account),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of people associated with the account’s legal entity.
    ///
    /// The people are returned sorted by creation date, with the most recent people appearing first.
    pub fn persons(
        client: &Client,
        account: &crate::account::AccountId,
        params: PersonsAccount,
    ) -> Response<crate::List<crate::person::Person>> {
        client.get_query(&format!("/accounts/{account}/persons", account = account), params)
    }
    /// Returns a list of capabilities associated with the account.
    ///
    /// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
    pub fn capabilities(
        client: &Client,
        account: &crate::account::AccountId,
        params: CapabilitiesAccount,
    ) -> Response<crate::List<crate::capability::Capability>> {
        client.get_query(&format!("/accounts/{account}/capabilities", account = account), params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveManyAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveManyAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccount<'a> {
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,
    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<UpdateAccountBusinessProfile<'a>>,
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<UpdateAccountBusinessType>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<UpdateAccountCompany<'a>>,
    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<crate::Currency>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<UpdateAccountDocuments<'a>>,
    /// The email address of the account holder.
    ///
    /// This is only to make the account easier to identify to you.
    /// Stripe only emails Custom accounts with your consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,
    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<UpdateAccountIndividual<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateAccountSettings<'a>>,
    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<UpdateAccountTosAcceptance<'a>>,
}
impl<'a> UpdateAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Business information about the account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountBusinessProfile<'a> {
    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<&'a str>,
    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Internal-only description of the product sold by, or service provided by, the business.
    ///
    /// Used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<UpdateAccountBusinessProfileSupportAddress<'a>>,
    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<&'a str>,
    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<&'a str>,
    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<&'a str>,
    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> UpdateAccountBusinessProfile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A publicly available mailing address for sending support issues to.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountBusinessProfileSupportAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateAccountBusinessProfileSupportAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The business type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl UpdateAccountBusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::GovernmentEntity => "government_entity",
            Self::Individual => "individual",
            Self::NonProfit => "non_profit",
        }
    }
}

impl AsRef<str> for UpdateAccountBusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateAccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
///
/// whether it has been requested or not).
/// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
/// An account may have some of its requested capabilities be active and some be inactive.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<UpdateAccountCapabilitiesSepaDebitPayments>,
    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<UpdateAccountCapabilitiesSofortPayments>,
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
}
impl UpdateAccountCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The acss_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesAcssDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesAcssDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The affirm_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesAffirmPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesAffirmPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The afterpay_clearpay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesAfterpayClearpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesAfterpayClearpayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The au_becs_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesAuBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesAuBecsDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The bacs_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesBacsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesBacsDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The bancontact_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesBancontactPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesBancontactPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The bank_transfer_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesBankTransferPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The blik_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesBlikPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesBlikPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The boleto_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesBoletoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesBoletoPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The card_issuing capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesCardIssuing {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesCardIssuing {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The card_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesCardPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The cartes_bancaires_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesCartesBancairesPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesCartesBancairesPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The eps_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesEpsPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesEpsPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The fpx_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesFpxPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesFpxPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The giropay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesGiropayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesGiropayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The grabpay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesGrabpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesGrabpayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The ideal_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesIdealPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesIdealPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The jcb_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesJcbPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesJcbPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The klarna_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesKlarnaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesKlarnaPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The konbini_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesKonbiniPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesKonbiniPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The legacy_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesLegacyPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesLegacyPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The link_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesLinkPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesLinkPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The oxxo_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesOxxoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesOxxoPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The p24_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesP24Payments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesP24Payments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The paynow_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesPaynowPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesPaynowPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The promptpay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesPromptpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesPromptpayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The sepa_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesSepaDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesSepaDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The sofort_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesSofortPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesSofortPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The tax_reporting_us_1099_k capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099K {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesTaxReportingUs1099K {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The tax_reporting_us_1099_misc capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The transfers capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesTransfers {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The treasury capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesTreasury {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesTreasury {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The us_bank_account_ach_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapabilitiesUsBankAccountAchPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl UpdateAccountCapabilitiesUsBankAccountAchPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the company or business.
///
/// This field is available for any `business_type`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompany<'a> {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateAccountCompanyAddress<'a>>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<UpdateAccountCompanyAddressKana<'a>>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<UpdateAccountCompanyAddressKanji<'a>>,
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
    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<&'a str>,
    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<&'a str>,
    /// Whether the company's owners have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<UpdateAccountCompanyOwnershipDeclaration<'a>>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    ///
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<&'a str>,
    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<UpdateAccountCompanyStructure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    ///
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<&'a str>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<&'a str>,
    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<&'a str>,
    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<UpdateAccountCompanyVerification<'a>>,
}
impl<'a> UpdateAccountCompany<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The company's primary address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateAccountCompanyAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> UpdateAccountCompanyAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> UpdateAccountCompanyAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyOwnershipDeclaration<'a> {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> UpdateAccountCompanyOwnershipDeclaration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The category identifying the legal structure of the company or legal entity.
///
/// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountCompanyStructure {
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

impl UpdateAccountCompanyStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FreeZoneEstablishment => "free_zone_establishment",
            Self::FreeZoneLlc => "free_zone_llc",
            Self::GovernmentInstrumentality => "government_instrumentality",
            Self::GovernmentalUnit => "governmental_unit",
            Self::IncorporatedNonProfit => "incorporated_non_profit",
            Self::LimitedLiabilityPartnership => "limited_liability_partnership",
            Self::Llc => "llc",
            Self::MultiMemberLlc => "multi_member_llc",
            Self::PrivateCompany => "private_company",
            Self::PrivateCorporation => "private_corporation",
            Self::PrivatePartnership => "private_partnership",
            Self::PublicCompany => "public_company",
            Self::PublicCorporation => "public_corporation",
            Self::PublicPartnership => "public_partnership",
            Self::SingleMemberLlc => "single_member_llc",
            Self::SoleEstablishment => "sole_establishment",
            Self::SoleProprietorship => "sole_proprietorship",
            Self::TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            Self::UnincorporatedAssociation => "unincorporated_association",
            Self::UnincorporatedNonProfit => "unincorporated_non_profit",
        }
    }
}

impl AsRef<str> for UpdateAccountCompanyStructure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Information on the verification state of the company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyVerification<'a> {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdateAccountCompanyVerificationDocument<'a>>,
}
impl<'a> UpdateAccountCompanyVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A document verifying the business.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCompanyVerificationDocument<'a> {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> UpdateAccountCompanyVerificationDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocuments<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    ///
    /// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateAccountDocumentsBankAccountOwnershipVerification<'a>>,
    /// One or more documents that demonstrate proof of a company's license to operate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<UpdateAccountDocumentsCompanyLicense<'a>>,
    /// One or more documents showing the company's Memorandum of Association.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association:
        Option<UpdateAccountDocumentsCompanyMemorandumOfAssociation<'a>>,
    /// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<UpdateAccountDocumentsCompanyMinisterialDecree<'a>>,
    /// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification:
        Option<UpdateAccountDocumentsCompanyRegistrationVerification<'a>>,
    /// One or more documents that demonstrate proof of a company's tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<UpdateAccountDocumentsCompanyTaxIdVerification<'a>>,
    /// One or more documents showing the company’s proof of registration with the national business registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<UpdateAccountDocumentsProofOfRegistration<'a>>,
}
impl<'a> UpdateAccountDocuments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
///
/// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a voided check.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocumentsBankAccountOwnershipVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountDocumentsBankAccountOwnershipVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof of a company's license to operate.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocumentsCompanyLicense<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountDocumentsCompanyLicense<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the company's Memorandum of Association.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocumentsCompanyMemorandumOfAssociation<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountDocumentsCompanyMemorandumOfAssociation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocumentsCompanyMinisterialDecree<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountDocumentsCompanyMinisterialDecree<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocumentsCompanyRegistrationVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountDocumentsCompanyRegistrationVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof of a company's tax ID.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocumentsCompanyTaxIdVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountDocumentsCompanyTaxIdVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the company’s proof of registration with the national business registry.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountDocumentsProofOfRegistration<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountDocumentsProofOfRegistration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the person represented by the account.
///
/// This field is null unless `business_type` is set to `individual`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividual<'a> {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateAccountIndividualAddress<'a>>,
    /// The Kana variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<UpdateAccountIndividualAddressKana<'a>>,
    /// The Kanji variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<UpdateAccountIndividualAddressKanji<'a>>,
    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<UpdateAccountIndividualDob>,
    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// The Kana variation of the the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<&'a str>,
    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<&'a str>,
    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<&'a [&'a str]>,
    /// The individual's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<&'a str>,
    /// The government-issued ID number of the individual, as appropriate for the representative’s country.
    ///
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<&'a str>,
    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<&'a str>,
    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<&'a str>,
    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<UpdateAccountIndividualPoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<UpdateAccountIndividualRegisteredAddress<'a>>,
    /// The last four digits of the individual's Social Security Number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<UpdateAccountIndividualVerification<'a>>,
}
impl<'a> UpdateAccountIndividual<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The individual's primary address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateAccountIndividualAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> UpdateAccountIndividualAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> UpdateAccountIndividualAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The individual's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateAccountIndividualDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl UpdateAccountIndividualDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountIndividualPoliticalExposure {
    Existing,
    None,
}

impl UpdateAccountIndividualPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Existing => "existing",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdateAccountIndividualPoliticalExposure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The individual's registered address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualRegisteredAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateAccountIndividualRegisteredAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The individual's verification document information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualVerification<'a> {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<UpdateAccountIndividualVerificationAdditionalDocument<'a>>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<UpdateAccountIndividualVerificationDocument<'a>>,
}
impl<'a> UpdateAccountIndividualVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualVerificationAdditionalDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> UpdateAccountIndividualVerificationAdditionalDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An identifying document, either a passport or local ID card.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountIndividualVerificationDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> UpdateAccountIndividualVerificationDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options for customizing how the account functions within Stripe.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettings<'a> {
    /// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<UpdateAccountSettingsBranding<'a>>,
    /// Settings specific to the account's use of the Card Issuing product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateAccountSettingsCardIssuing<'a>>,
    /// Settings specific to card charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<UpdateAccountSettingsCardPayments<'a>>,
    /// Settings that apply across payment methods for charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<UpdateAccountSettingsPayments<'a>>,
    /// Settings specific to the account's payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<UpdateAccountSettingsPayouts<'a>>,
    /// Settings specific to the account's Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<UpdateAccountSettingsTreasury<'a>>,
}
impl<'a> UpdateAccountSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsBranding<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<&'a str>,
    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<&'a str>,
    /// A CSS hex color value representing the secondary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<&'a str>,
}
impl<'a> UpdateAccountSettingsBranding<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings specific to the account's use of the Card Issuing product.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsCardIssuing<'a> {
    /// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](https://stripe.com/docs/issuing/connect/tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<UpdateAccountSettingsCardIssuingTosAcceptance<'a>>,
}
impl<'a> UpdateAccountSettingsCardIssuing<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](https://stripe.com/docs/issuing/connect/tos_acceptance).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsCardIssuingTosAcceptance<'a> {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> UpdateAccountSettingsCardIssuingTosAcceptance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings specific to card charging on the account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsCardPayments<'a> {
    /// Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<UpdateAccountSettingsCardPaymentsDeclineOn>,
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<&'a str>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kana: Option<&'a str>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kanji: Option<&'a str>,
}
impl<'a> UpdateAccountSettingsCardPayments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsCardPaymentsDeclineOn {
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
impl UpdateAccountSettingsCardPaymentsDeclineOn {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings that apply across payment methods for charging on the account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsPayments<'a> {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<&'a str>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<&'a str>,
}
impl<'a> UpdateAccountSettingsPayments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings specific to the account's payouts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsPayouts<'a> {
    /// A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// For details, see [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    ///
    /// For details, see our [Setting Bank and Debit Card Payouts](https://stripe.com/docs/connect/bank-transfers#payout-information) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<UpdateAccountSettingsPayoutsSchedule>,
    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> UpdateAccountSettingsPayouts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on when funds from charges are available, and when they are paid out to an external account.
///
/// For details, see our [Setting Bank and Debit Card Payouts](https://stripe.com/docs/connect/bank-transfers#payout-information) documentation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsPayoutsSchedule {
    /// The number of days charge funds are held before being paid out.
    ///
    /// May also be set to `minimum`, representing the lowest available value for the account country.
    /// Default is `minimum`.
    /// The `delay_days` parameter does not apply when the `interval` is `manual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<UpdateAccountSettingsPayoutsScheduleDelayDays>,
    /// How frequently available funds are paid out.
    ///
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<UpdateAccountSettingsPayoutsScheduleInterval>,
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
    pub weekly_anchor: Option<UpdateAccountSettingsPayoutsScheduleWeeklyAnchor>,
}
impl UpdateAccountSettingsPayoutsSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The number of days charge funds are held before being paid out.
///
/// May also be set to `minimum`, representing the lowest available value for the account country.
/// Default is `minimum`.
/// The `delay_days` parameter does not apply when the `interval` is `manual`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateAccountSettingsPayoutsScheduleDelayDays {
    Minimum,
    U32(u32),
}
/// How frequently available funds are paid out.
///
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountSettingsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
}

impl UpdateAccountSettingsPayoutsScheduleInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Manual => "manual",
            Self::Monthly => "monthly",
            Self::Weekly => "weekly",
        }
    }
}

impl AsRef<str> for UpdateAccountSettingsPayoutsScheduleInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
///
/// (required and applicable only if `interval` is `weekly`.).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}

impl UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Friday => "friday",
            Self::Monday => "monday",
            Self::Saturday => "saturday",
            Self::Sunday => "sunday",
            Self::Thursday => "thursday",
            Self::Tuesday => "tuesday",
            Self::Wednesday => "wednesday",
        }
    }
}

impl AsRef<str> for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Settings specific to the account's Treasury FinancialAccounts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsTreasury<'a> {
    /// Details on the account's acceptance of the Stripe Treasury Services Agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<UpdateAccountSettingsTreasuryTosAcceptance<'a>>,
}
impl<'a> UpdateAccountSettingsTreasury<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on the account's acceptance of the Stripe Treasury Services Agreement.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountSettingsTreasuryTosAcceptance<'a> {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> UpdateAccountSettingsTreasuryTosAcceptance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountTosAcceptance<'a> {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user's service agreement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> UpdateAccountTosAcceptance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListAccount<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccount<'a> {
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,
    /// Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<CreateAccountBusinessProfile<'a>>,
    /// The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<CreateAccountBusinessType>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CreateAccountCompany<'a>>,
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
    pub default_currency: Option<crate::Currency>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateAccountDocuments<'a>>,
    /// The email address of the account holder.
    ///
    /// This is only to make the account easier to identify to you.
    /// Stripe only emails Custom accounts with your consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,
    /// Information about the person represented by the account.
    ///
    /// This field is null unless `business_type` is set to `individual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<CreateAccountIndividual<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateAccountSettings<'a>>,
    /// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<CreateAccountTosAcceptance<'a>>,
    /// The type of Stripe account to create.
    ///
    /// May be one of `custom`, `express` or `standard`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateAccountType>,
}
impl<'a> CreateAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Business information about the account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountBusinessProfile<'a> {
    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<&'a str>,
    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Internal-only description of the product sold by, or service provided by, the business.
    ///
    /// Used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<CreateAccountBusinessProfileSupportAddress<'a>>,
    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<&'a str>,
    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<&'a str>,
    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<&'a str>,
    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> CreateAccountBusinessProfile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A publicly available mailing address for sending support issues to.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountBusinessProfileSupportAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateAccountBusinessProfileSupportAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The business type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl CreateAccountBusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::GovernmentEntity => "government_entity",
            Self::Individual => "individual",
            Self::NonProfit => "non_profit",
        }
    }
}

impl AsRef<str> for CreateAccountBusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Each key of the dictionary represents a capability, and each capability maps to its settings (e.g.
///
/// whether it has been requested or not).
/// Each capability will be inactive until you have provided its specific requirements and Stripe has verified them.
/// An account may have some of its requested capabilities be active and some be inactive.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<CreateAccountCapabilitiesSepaDebitPayments>,
    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<CreateAccountCapabilitiesSofortPayments>,
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
}
impl CreateAccountCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The acss_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesAcssDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesAcssDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The affirm_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesAffirmPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesAffirmPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The afterpay_clearpay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesAfterpayClearpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesAfterpayClearpayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The au_becs_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesAuBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesAuBecsDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The bacs_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesBacsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesBacsDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The bancontact_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesBancontactPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesBancontactPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The bank_transfer_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesBankTransferPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The blik_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesBlikPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesBlikPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The boleto_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesBoletoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesBoletoPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The card_issuing capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesCardIssuing {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesCardIssuing {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The card_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesCardPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The cartes_bancaires_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesCartesBancairesPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesCartesBancairesPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The eps_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesEpsPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesEpsPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The fpx_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesFpxPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesFpxPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The giropay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesGiropayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesGiropayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The grabpay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesGrabpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesGrabpayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The ideal_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesIdealPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesIdealPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The jcb_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesJcbPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesJcbPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The klarna_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesKlarnaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesKlarnaPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The konbini_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesKonbiniPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesKonbiniPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The legacy_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesLegacyPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesLegacyPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The link_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesLinkPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesLinkPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The oxxo_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesOxxoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesOxxoPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The p24_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesP24Payments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesP24Payments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The paynow_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesPaynowPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesPaynowPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The promptpay_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesPromptpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesPromptpayPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The sepa_debit_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesSepaDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesSepaDebitPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The sofort_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesSofortPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesSofortPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The tax_reporting_us_1099_k capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099K {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesTaxReportingUs1099K {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The tax_reporting_us_1099_misc capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099Misc {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesTaxReportingUs1099Misc {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The transfers capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesTransfers {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesTransfers {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The treasury capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesTreasury {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesTreasury {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The us_bank_account_ach_payments capability.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCapabilitiesUsBankAccountAchPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CreateAccountCapabilitiesUsBankAccountAchPayments {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the company or business.
///
/// This field is available for any `business_type`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompany<'a> {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateAccountCompanyAddress<'a>>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateAccountCompanyAddressKana<'a>>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateAccountCompanyAddressKanji<'a>>,
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
    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<&'a str>,
    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<&'a str>,
    /// Whether the company's owners have been provided.
    ///
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](https://stripe.com/docs/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CreateAccountCompanyOwnershipDeclaration<'a>>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    ///
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<&'a str>,
    /// The category identifying the legal structure of the company or legal entity.
    ///
    /// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CreateAccountCompanyStructure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    ///
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<&'a str>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<&'a str>,
    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<&'a str>,
    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CreateAccountCompanyVerification<'a>>,
}
impl<'a> CreateAccountCompany<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The company's primary address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateAccountCompanyAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateAccountCompanyAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateAccountCompanyAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyOwnershipDeclaration<'a> {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> CreateAccountCompanyOwnershipDeclaration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The category identifying the legal structure of the company or legal entity.
///
/// See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountCompanyStructure {
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

impl CreateAccountCompanyStructure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FreeZoneEstablishment => "free_zone_establishment",
            Self::FreeZoneLlc => "free_zone_llc",
            Self::GovernmentInstrumentality => "government_instrumentality",
            Self::GovernmentalUnit => "governmental_unit",
            Self::IncorporatedNonProfit => "incorporated_non_profit",
            Self::LimitedLiabilityPartnership => "limited_liability_partnership",
            Self::Llc => "llc",
            Self::MultiMemberLlc => "multi_member_llc",
            Self::PrivateCompany => "private_company",
            Self::PrivateCorporation => "private_corporation",
            Self::PrivatePartnership => "private_partnership",
            Self::PublicCompany => "public_company",
            Self::PublicCorporation => "public_corporation",
            Self::PublicPartnership => "public_partnership",
            Self::SingleMemberLlc => "single_member_llc",
            Self::SoleEstablishment => "sole_establishment",
            Self::SoleProprietorship => "sole_proprietorship",
            Self::TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            Self::UnincorporatedAssociation => "unincorporated_association",
            Self::UnincorporatedNonProfit => "unincorporated_non_profit",
        }
    }
}

impl AsRef<str> for CreateAccountCompanyStructure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Information on the verification state of the company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyVerification<'a> {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateAccountCompanyVerificationDocument<'a>>,
}
impl<'a> CreateAccountCompanyVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A document verifying the business.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountCompanyVerificationDocument<'a> {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> CreateAccountCompanyVerificationDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocuments<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    ///
    /// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<CreateAccountDocumentsBankAccountOwnershipVerification<'a>>,
    /// One or more documents that demonstrate proof of a company's license to operate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<CreateAccountDocumentsCompanyLicense<'a>>,
    /// One or more documents showing the company's Memorandum of Association.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association:
        Option<CreateAccountDocumentsCompanyMemorandumOfAssociation<'a>>,
    /// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<CreateAccountDocumentsCompanyMinisterialDecree<'a>>,
    /// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification:
        Option<CreateAccountDocumentsCompanyRegistrationVerification<'a>>,
    /// One or more documents that demonstrate proof of a company's tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<CreateAccountDocumentsCompanyTaxIdVerification<'a>>,
    /// One or more documents showing the company’s proof of registration with the national business registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<CreateAccountDocumentsProofOfRegistration<'a>>,
}
impl<'a> CreateAccountDocuments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
///
/// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a voided check.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocumentsBankAccountOwnershipVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountDocumentsBankAccountOwnershipVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof of a company's license to operate.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocumentsCompanyLicense<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountDocumentsCompanyLicense<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the company's Memorandum of Association.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocumentsCompanyMemorandumOfAssociation<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountDocumentsCompanyMemorandumOfAssociation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocumentsCompanyMinisterialDecree<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountDocumentsCompanyMinisterialDecree<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocumentsCompanyRegistrationVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountDocumentsCompanyRegistrationVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that demonstrate proof of a company's tax ID.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocumentsCompanyTaxIdVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountDocumentsCompanyTaxIdVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents showing the company’s proof of registration with the national business registry.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountDocumentsProofOfRegistration<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountDocumentsProofOfRegistration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the person represented by the account.
///
/// This field is null unless `business_type` is set to `individual`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividual<'a> {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateAccountIndividualAddress<'a>>,
    /// The Kana variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateAccountIndividualAddressKana<'a>>,
    /// The Kanji variation of the the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateAccountIndividualAddressKanji<'a>>,
    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreateAccountIndividualDob>,
    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// The Kana variation of the the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<&'a str>,
    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<&'a str>,
    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<&'a [&'a str]>,
    /// The individual's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<&'a str>,
    /// The government-issued ID number of the individual, as appropriate for the representative’s country.
    ///
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    ///
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](https://stripe.com/docs/js/tokens_sources/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<&'a str>,
    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<&'a str>,
    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<&'a str>,
    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<CreateAccountIndividualPoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreateAccountIndividualRegisteredAddress<'a>>,
    /// The last four digits of the individual's Social Security Number (U.S.
    ///
    /// only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<CreateAccountIndividualVerification<'a>>,
}
impl<'a> CreateAccountIndividual<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The individual's primary address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateAccountIndividualAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kana variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualAddressKana<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateAccountIndividualAddressKana<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The Kanji variation of the the individual's primary address (Japan only).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualAddressKanji<'a> {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<&'a str>,
}
impl<'a> CreateAccountIndividualAddressKanji<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The individual's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountIndividualDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl CreateAccountIndividualDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountIndividualPoliticalExposure {
    Existing,
    None,
}

impl CreateAccountIndividualPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Existing => "existing",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateAccountIndividualPoliticalExposure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The individual's registered address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualRegisteredAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateAccountIndividualRegisteredAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The individual's verification document information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualVerification<'a> {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<CreateAccountIndividualVerificationAdditionalDocument<'a>>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<CreateAccountIndividualVerificationDocument<'a>>,
}
impl<'a> CreateAccountIndividualVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualVerificationAdditionalDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> CreateAccountIndividualVerificationAdditionalDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An identifying document, either a passport or local ID card.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountIndividualVerificationDocument<'a> {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<&'a str>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    ///
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<&'a str>,
}
impl<'a> CreateAccountIndividualVerificationDocument<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options for customizing how the account functions within Stripe.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettings<'a> {
    /// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<CreateAccountSettingsBranding<'a>>,
    /// Settings specific to the account's use of the Card Issuing product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CreateAccountSettingsCardIssuing<'a>>,
    /// Settings specific to card charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CreateAccountSettingsCardPayments<'a>>,
    /// Settings that apply across payment methods for charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountSettingsPayments<'a>>,
    /// Settings specific to the account's payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<CreateAccountSettingsPayouts<'a>>,
    /// Settings specific to the account's Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CreateAccountSettingsTreasury<'a>>,
}
impl<'a> CreateAccountSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsBranding<'a> {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<&'a str>,
    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<&'a str>,
    /// A CSS hex color value representing the secondary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<&'a str>,
}
impl<'a> CreateAccountSettingsBranding<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings specific to the account's use of the Card Issuing product.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsCardIssuing<'a> {
    /// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](https://stripe.com/docs/issuing/connect/tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<CreateAccountSettingsCardIssuingTosAcceptance<'a>>,
}
impl<'a> CreateAccountSettingsCardIssuing<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](https://stripe.com/docs/issuing/connect/tos_acceptance).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsCardIssuingTosAcceptance<'a> {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> CreateAccountSettingsCardIssuingTosAcceptance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings specific to card charging on the account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsCardPayments<'a> {
    /// Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<CreateAccountSettingsCardPaymentsDeclineOn>,
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<&'a str>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kana: Option<&'a str>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    ///
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kanji: Option<&'a str>,
}
impl<'a> CreateAccountSettingsCardPayments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsCardPaymentsDeclineOn {
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
impl CreateAccountSettingsCardPaymentsDeclineOn {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings that apply across payment methods for charging on the account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsPayments<'a> {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<&'a str>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<&'a str>,
}
impl<'a> CreateAccountSettingsPayments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings specific to the account's payouts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsPayouts<'a> {
    /// A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// For details, see [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    ///
    /// For details, see our [Setting Bank and Debit Card Payouts](https://stripe.com/docs/connect/bank-transfers#payout-information) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CreateAccountSettingsPayoutsSchedule>,
    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> CreateAccountSettingsPayouts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on when funds from charges are available, and when they are paid out to an external account.
///
/// For details, see our [Setting Bank and Debit Card Payouts](https://stripe.com/docs/connect/bank-transfers#payout-information) documentation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsPayoutsSchedule {
    /// The number of days charge funds are held before being paid out.
    ///
    /// May also be set to `minimum`, representing the lowest available value for the account country.
    /// Default is `minimum`.
    /// The `delay_days` parameter does not apply when the `interval` is `manual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<CreateAccountSettingsPayoutsScheduleDelayDays>,
    /// How frequently available funds are paid out.
    ///
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CreateAccountSettingsPayoutsScheduleInterval>,
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
    pub weekly_anchor: Option<CreateAccountSettingsPayoutsScheduleWeeklyAnchor>,
}
impl CreateAccountSettingsPayoutsSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The number of days charge funds are held before being paid out.
///
/// May also be set to `minimum`, representing the lowest available value for the account country.
/// Default is `minimum`.
/// The `delay_days` parameter does not apply when the `interval` is `manual`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreateAccountSettingsPayoutsScheduleDelayDays {
    Minimum,
    U32(u32),
}
/// How frequently available funds are paid out.
///
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountSettingsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
}

impl CreateAccountSettingsPayoutsScheduleInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Manual => "manual",
            Self::Monthly => "monthly",
            Self::Weekly => "weekly",
        }
    }
}

impl AsRef<str> for CreateAccountSettingsPayoutsScheduleInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
///
/// (required and applicable only if `interval` is `weekly`.).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}

impl CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Friday => "friday",
            Self::Monday => "monday",
            Self::Saturday => "saturday",
            Self::Sunday => "sunday",
            Self::Thursday => "thursday",
            Self::Tuesday => "tuesday",
            Self::Wednesday => "wednesday",
        }
    }
}

impl AsRef<str> for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Settings specific to the account's Treasury FinancialAccounts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsTreasury<'a> {
    /// Details on the account's acceptance of the Stripe Treasury Services Agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<CreateAccountSettingsTreasuryTosAcceptance<'a>>,
}
impl<'a> CreateAccountSettingsTreasury<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on the account's acceptance of the Stripe Treasury Services Agreement.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountSettingsTreasuryTosAcceptance<'a> {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> CreateAccountSettingsTreasuryTosAcceptance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details on the account's acceptance of the [Stripe Services Agreement](https://stripe.com/docs/connect/updating-accounts#tos-acceptance).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountTosAcceptance<'a> {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::Timestamp>,
    /// The IP address from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user's service agreement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<&'a str>,
    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> CreateAccountTosAcceptance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of Stripe account to create.
///
/// May be one of `custom`, `express` or `standard`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountType {
    Custom,
    Express,
    Standard,
}

impl CreateAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Express => "express",
            Self::Standard => "standard",
        }
    }
}

impl AsRef<str> for CreateAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RejectAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The reason for rejecting the account.
    ///
    /// Can be `fraud`, `terms_of_service`, or `other`.
    pub reason: &'a str,
}
impl<'a> RejectAccount<'a> {
    pub fn new(reason: &'a str) -> Self {
        Self { expand: Default::default(), reason }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PersonsAccount<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filters on the list of people returned based on the person's relationship to the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PersonsAccountRelationship>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> PersonsAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filters on the list of people returned based on the person's relationship to the account's company.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PersonsAccountRelationship {
    /// A filter on the list of people returned based on whether these people are directors of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// A filter on the list of people returned based on whether these people are executives of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// A filter on the list of people returned based on whether these people are owners of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// A filter on the list of people returned based on whether these people are the representative of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
}
impl PersonsAccountRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CapabilitiesAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CapabilitiesAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
