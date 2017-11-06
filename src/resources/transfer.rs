use params::{List, Metadata, Timestamp};
use resources::Currency;

/// The resource representing a Stripe transfer reversal.
///
/// For more details see https://stripe.com/docs/api/dotnet#transfer_reversal_object.
#[derive(Debug, Deserialize)]
pub struct TransferReversal {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub metadata: Metadata,
    pub transfer: String,
}

/// The resource representing a Stripe transfer.
///
/// For more details see https://stripe.com/docs/api/dotnet#transfer_object.
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub amount_reversed: u64,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: Option<String>,
    pub destination: String,
    pub destination_payment: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub reversals: List<TransferReversal>,
    pub reversed: bool,
    pub source_transaction: String,
    pub source_type: String,
    pub transfer_group: String,
}
