use crate::ids::AccountId;
use crate::params::{List, Metadata, Object, Timestamp};
use crate::resources::{BankAccount, Currency};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeclineChargeDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayoutScheduleDetails {
    pub delay_days: u64,
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TosAcceptanceDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// The set of parameters that can be used when creating an account for users.
///
/// For more details see https://stripe.com/docs/api#create_account.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>, // (country the account holder resides in)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>, // (required if account type is standard)
    #[serde(rename = "type")]
    pub account_type: &'static str,
}

/// An enum representing the possible values of an `Accounts`'s `account_type` field.
///
/// For more details see [https://stripe.com/docs/api/accounts/object#account_object-type](https://stripe.com/docs/api/accounts/object#account_object-type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Standard,
    Express,
    Custom,
}

/// The resource representing a Stripe account.
///
/// For more details see https://stripe.com/docs/api#account.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    /// Unique identifier for the object.
    pub id: AccountId,
    /// The Stripe account type.
    #[serde(rename = "type")]
    pub type_: AccountType,

    /// Whether the account can create live charges.
    pub charges_enabled: bool,
    /// The account's country.
    pub country: String,
    ///  Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: Timestamp,
    /// Three-letter ISO currency code representing the default currency for the account. This must be
    /// a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    pub default_currency: Currency,
    /// Always true for a deleted object.
    #[serde(default)]
    pub deleted: bool,
    /// Whether account details have been submitted. Standard accounts cannot receive payouts before
    /// this is true.
    pub details_submitted: bool,
    /// The primary user's email address.
    pub email: String,
    /// External accounts (bank accounts and debit cards) currently attached to this account.
    pub external_accounts: List<BankAccount>,
    /// Set of key-value pairs that you can attach to an object. This can be useful for storing
    /// additional information about the object in a structured format.
    pub metadata: Metadata,
    /// Whether Stripe can send payouts to this account.
    pub payouts_enabled: bool,
    /// Details on the acceptance of the Stripe Services Agreement.
    pub tos_acceptance: Option<TosAcceptanceDetails>,

    // FIXME: These were moved into `business_profile`
    pub business_name: Option<String>,
    pub business_url: Option<String>,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub product_description: Option<String>,

    // FIXME: This was split into `individual` and `company`
    pub legal_entity: Option<serde_json::Value>,

    // FIXME: These were moved into `settings`
    pub debit_negative_balances: Option<bool>,
    pub decline_charge_on: Option<DeclineChargeDetails>,
    pub statement_descriptor: Option<String>,
    pub payout_schedule: Option<PayoutScheduleDetails>,
    pub payout_statement_descriptor: Option<String>,
    pub display_name: Option<String>,
    pub timezone: Option<String>,

    // FIXME: This may have been removed
    pub verification: Option<serde_json::Value>,
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
