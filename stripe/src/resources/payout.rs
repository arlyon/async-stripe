use params::{Metadata, Timestamp};
use resources::Currency;

/// The resource representing a Stripe payout.
///
/// For more details see https://stripe.com/docs/api#payout_object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Payout {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub arrival_date: Timestamp,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: String,
    pub destination: Option<String>,
    pub failure_balance_transaction: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub method: String,      // (standard, instant)
    pub source_type: String, // (card, bank_account, bitcoin_receiver, alipay_account)
    pub statement_descriptor: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub payout_type: String, // (bank_account, card)
}
