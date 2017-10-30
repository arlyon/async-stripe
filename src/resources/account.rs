#[derive(Serialize)]
pub struct AccountParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>, // (country the account holder resides in)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>, // (required if account type is standard)
    #[serde(rename = "type")]
    pub account_type: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub object: String,
    pub business_name: String,
    pub business_url: Option<String>,
    pub charges_enabed: bool,
    pub country: String,
    pub debit_negative_balances: Option<bool>,
    pub decline_charge_on: Option<bool>, // hash
    pub default_currency: String,
    pub details_submitted: bool,
    pub display_name: String,
    pub email: String,
    pub external_accounts: Option<bool>, // list
    pub legal_entity: Option<bool>, // hash
    pub metadata: bool, // hash
    pub payout_schedule: Option<bool>, // hash
    pub payout_statement_descriptor: Option<String>,
    pub payouts_enabled: bool,
    pub product_description: Option<String>,
    pub statement_descriptor: String,
    pub support_email: String,
    pub support_phone: String,
    pub timezone: String,
    pub tos_acceptance: Option<String>, // hash
    #[serde(rename = "type")]
    pub account_type: String, // Stripe, custom, or express
    pub verification: Option<String>, // hash
}
