use crate::ids::AccountId;
use crate::params::{Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{
    Address, BankAccount, Card, Currency, File, LegalEntityJapanAddress, Person,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Account".
///
/// For more details see [https://stripe.com/docs/api/accounts/object](https://stripe.com/docs/api/accounts/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    /// Unique identifier for the object.
    id: AccountId,

    /// Optional information related to the business.
    business_profile: Option<BusinessProfile>,

    /// The business type.
    ///
    /// Can be `individual` or `company`.
    business_type: Option<String>,

    capabilities: AccountCapabilities,

    /// Whether the account can create live charges.
    charges_enabled: bool,

    company: LegalEntityCompany,

    /// The account's country.
    country: String,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    created: Timestamp,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    default_currency: Currency,

    /// Whether account details have been submitted.
    ///
    /// Standard accounts cannot receive payouts before this is true.
    details_submitted: bool,

    /// The primary user's email address.
    email: Option<String>,

    /// External accounts (bank accounts and debit cards) currently attached to this account.
    external_accounts: List<ExternalAccount>,

    individual: Person,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    metadata: Metadata,

    /// Whether Stripe can send payouts to this account.
    payouts_enabled: bool,

    requirements: AccountRequirements,

    /// Account options for customizing how the account functions within Stripe.
    settings: Option<AccountSettings>,

    tos_acceptance: TosAcceptance,

    /// The Stripe account type.
    ///
    /// Can be `standard`, `express`, or `custom`.
    #[serde(rename = "type")]
    type_: String,
}

impl Object for Account {
    type Id = AccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn object(&self) -> &'static str {
        "account"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BusinessProfile {
    /// The merchant category code for the account.
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    mcc: Option<String>,

    /// The customer-facing business name.
    name: Option<String>,

    /// Internal-only description of the product sold or service provided by the business.
    ///
    /// It's used by Stripe for risk and underwriting purposes.
    product_description: Option<String>,

    /// A publicly available mailing address for sending support issues to.
    support_address: Option<Address>,

    /// A publicly available email address for sending support issues to.
    support_email: Option<String>,

    /// A publicly available phone number to call with support issues.
    support_phone: Option<String>,

    /// A publicly available website for handling support issues.
    support_url: Option<String>,

    /// The business's publicly available website.
    url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCapabilities {
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    card_payments: AccountCapabilitiesCardPayments,

    /// The status of the legacy payments capability of the account.
    legacy_payments: AccountCapabilitiesLegacyPayments,

    /// The status of the platform payments capability of the account, or whether your platform can process charges on behalf of the account.
    platform_payments: AccountCapabilitiesPlatformPayments,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirements {
    /// The date the fields in `currently_due` must be collected by to keep payouts enabled for the account.
    ///
    /// These fields might block payouts sooner if the next threshold is reached before these fields are collected.
    current_deadline: Option<Timestamp>,

    /// The fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by the `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    currently_due: Option<Vec<String>>,

    /// If the account is disabled, this string describes why the account can’t create charges or receive payouts.
    ///
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `rejected.fraud`, `rejected.terms_of_service`, `rejected.listed`, `rejected.other`, `listed`, `under_review`, or `other`.
    disabled_reason: Option<String>,

    /// The fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, these fields appear in `currently_due` as well, and the `current_deadline` is set.
    eventually_due: Option<Vec<String>>,

    /// The fields that weren't collected by the `current_deadline`.
    ///
    /// These fields need to be collected to re-enable the account.
    past_due: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettings {
    branding: BrandingSettings,

    card_payments: CardPaymentsSettings,

    dashboard: DashboardSettings,

    payments: PaymentsSettings,

    payouts: PayoutSettings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    icon: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    logo: Option<Expandable<File>>,

    /// A CSS hex color value representing the primary branding color for this account.
    primary_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettings {
    decline_on: DeclineChargeOn,

    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    statement_descriptor_prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DashboardSettings {
    /// The display name for this account.
    ///
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    display_name: Option<String>,

    /// The timezone used in the Stripe Dashboard for this account.
    ///
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    timezone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOn {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    ///
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    avs_failure: bool,

    /// Whether Stripe automatically declines charges with an incorrect CVC.
    ///
    /// This setting only applies when a CVC is provided and it fails bank verification.
    cvc_failure: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettings {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    statement_descriptor: Option<String>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    statement_descriptor_kana: Option<String>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    statement_descriptor_kanji: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettings {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details.
    /// Default value is `true` for Express accounts and `false` for Custom accounts.
    debit_negative_balances: bool,

    schedule: TransferSchedule,

    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TosAcceptance {
    /// The Unix timestamp marking when the Stripe Services Agreement was accepted by the account representative.
    date: Option<Timestamp>,

    /// The IP address from which the Stripe Services Agreement was accepted by the account representative.
    ip: Option<String>,

    /// The user agent of the browser from which the Stripe Services Agreement was accepted by the account representative.
    user_agent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LegalEntityCompany {
    address: Address,

    /// The Kana variation of the company's primary address (Japan only).
    address_kana: Option<LegalEntityJapanAddress>,

    /// The Kanji variation of the company's primary address (Japan only).
    address_kanji: Option<LegalEntityJapanAddress>,

    /// Whether the company's directors have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    directors_provided: bool,

    /// The company's legal name.
    name: Option<String>,

    /// The Kana variation of the company's legal name (Japan only).
    name_kana: Option<String>,

    /// The Kanji variation of the company's legal name (Japan only).
    name_kanji: Option<String>,

    /// Whether the company's owners have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that all owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    owners_provided: bool,

    /// The company's phone number (used for verification).
    phone: Option<String>,

    /// Whether the company's business ID number was provided.
    tax_id_provided: bool,

    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    tax_id_registrar: String,

    /// Whether the company's business VAT number was provided.
    vat_id_provided: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferSchedule {
    /// The number of days charges for the account will be held before being paid out.
    delay_days: u32,

    /// How frequently funds will be paid out.
    ///
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    interval: String,

    /// The day of the month funds will be paid out.
    ///
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    monthly_anchor: Option<u8>,

    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    ///
    /// Only shown if `interval` is weekly.
    weekly_anchor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(BankAccount),
    Card(Card),
}

/// An enum representing the possible values of an `AccountCapabilities`'s `card_payments` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesCardPayments {
    Active,
    Inactive,
    Pending,
}

/// An enum representing the possible values of an `AccountCapabilities`'s `legacy_payments` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesLegacyPayments {
    Active,
    Inactive,
    Pending,
}

/// An enum representing the possible values of an `AccountCapabilities`'s `platform_payments` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesPlatformPayments {
    Active,
    Inactive,
    Pending,
}
