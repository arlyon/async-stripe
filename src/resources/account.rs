use crate::params::{Identifiable, List, Metadata, Timestamp};
use crate::resources::BankAccount;
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
pub struct TOSAcceptanceDetails {
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

/// The resource representing a Stripe account.
///
/// For more details see https://stripe.com/docs/api#account.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub object: String,
    pub business_name: String,
    pub business_url: Option<String>,
    pub charges_enabed: bool,
    pub country: String,
    pub debit_negative_balances: Option<bool>,
    pub decline_charge_on: Option<DeclineChargeDetails>,
    pub default_currency: String,
    pub details_submitted: bool,
    pub display_name: String,
    pub email: String,
    pub external_accounts: List<BankAccount>,
    pub legal_entity: Option<serde_json::Value>,
    pub metadata: Metadata,
    pub payout_schedule: Option<PayoutScheduleDetails>,
    pub payout_statement_descriptor: Option<String>,
    pub payouts_enabled: bool,
    pub product_description: Option<String>,
    pub statement_descriptor: String,
    pub support_email: String,
    pub support_phone: String,
    pub timezone: String,
    pub tos_acceptance: Option<TOSAcceptanceDetails>, // (who accepted Stripe's terms of service)
    #[serde(rename = "type")]
    pub account_type: Option<String>, // (Stripe, Custom, or Express)
    pub verification: Option<serde_json::Value>,
}

impl Identifiable for Account {
    fn id(&self) -> &str {
        &self.id
    }
}
