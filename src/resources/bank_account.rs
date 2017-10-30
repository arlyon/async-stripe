use params::Metadata;
use resources::Currency;

/// The resource representing a Stripe bank account.
///
/// For more details see https://stripe.com/docs/api#bank_accounts.
#[derive(Debug, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub object: String,
    pub account: String,
    pub account_holder_name: String,
    pub account_holder_type: String, // (individual or company)
    pub bank_name: String,
    pub country: String,
    pub currency: Currency,
    pub customer: String,
    pub default_for_currency: bool,
    pub fingerprint: String,
    pub last4: String,
    pub metadata: Option<Metadata>,
    pub routing_number: String,
    pub status: String, // (new, validated, verified, verification_failed, errored)
}
