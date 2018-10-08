use params::{Identifiable, Metadata};
use resources::Currency;

/// The resource representing a Stripe bank account.
///
/// For more details see https://stripe.com/docs/api#customer_bank_account_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub metadata: Metadata,
    pub routing_number: String,
    pub status: String, // (new, validated, verified, verification_failed, errored)
}

impl Identifiable for BankAccount {
    fn id(&self) -> &str {
        &self.id
    }
}
