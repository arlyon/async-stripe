use serde::Deserialize;
use serde::Serialize;

use crate::client::{Client, Response};
use crate::params::Metadata;
use crate::resources::TransferReversal;
use crate::TransferId;

/// The set of parameters that can be used when doing transfer reversal.
///
/// For more details see <https://stripe.com/docs/api/transfer_reversals/create>.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTransferReversal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
}

impl TransferReversal {
    /// Create Transfer Reversal
    ///
    /// For more details see <https://stripe.com/docs/api/transfer_reversals/create>.
    pub fn create(
        client: &Client,
        transfer: &TransferId,
        params: CreateTransferReversal,
    ) -> Response<TransferReversal> {
        client.post_form(&format!("/transfers/{}/reversals", transfer), params)
    }
}
