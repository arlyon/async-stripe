use client::Client;
use error::Error;
use params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use resources::Currency;
use serde_qs as qs;

/// The resource representing a Stripe refund.
///
/// For more details see https://stripe.com/docs/api#refunds.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Refund {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: String,
    pub charge: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub failure_balance_transaction: Option<String>,
    pub failure_reason: Option<RefundFailureReason>,
    pub metadata: Metadata,
    pub reason: Option<RefundReason>,
    pub receipt_number: Option<String>,
    pub status: RefundStatus,
}

impl Identifiable for Refund {
    fn id(&self) -> &str {
        &self.id
    }
}

/// An enum representing the possible values of a `Refund`'s `reason` field.
///
/// For more details see [https://stripe.com/docs/api/refunds/object#refund_object-reason](https://stripe.com/docs/api/refunds/object#refund_object-reason)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Copy, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    #[serde(other)]
    Other,
}

/// An enum representing the possible values of a `Refund`'s `status` field.
///
/// For more details see [https://stripe.com/docs/api/refunds/object#refund_object-status](https://stripe.com/docs/api/refunds/object#refund_object-status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
    Canceled,
    #[serde(other)]
    Other,
}

/// An enum representing the possible values of a `Refund`'s `failure_reason` field.
///
/// For more details see [https://stripe.com/docs/api/refunds/object#refund_object-failure_reason](https://stripe.com/docs/api/refunds/object#refund_object-failure_reason)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RefundFailureReason {
    LostOrStolenCard,
    ExpiredOrCanceledCard,
    Unknown,
    #[serde(other)]
    Other,
}

/// The set of parameters that can be used when creating refund object.
///
/// For more details see [https://stripe.com/docs/api/refunds/create](https://stripe.com/docs/api/refunds/create).
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RefundParams {
    pub charge: String,
    pub amount: Option<u64>,
    pub metadata: Metadata,
    pub reason: Option<RefundReason>,
    pub refund_application_fee: Option<bool>,
    pub reverse_transfer: Option<bool>,
}

/// The set of parameters that can be used when listing refund.
///
/// For more details see [https://stripe.com/docs/api/refunds/list](https://stripe.com/docs/api/refunds/list)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RefundListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl Refund {
    /// Creates a new refund.
    ///
    /// For more details see [https://stripe.com/docs/api/refunds/create](https://stripe.com/docs/api/refunds/create).
    pub fn create(client: &Client, params: RefundParams) -> Result<Refund, Error> {
        client.post("/refunds", params)
    }

    /// Retrieves the details of a refund.
    ///
    /// For more details see [https://stripe.com/docs/api/refunds/retrieve](https://stripe.com/docs/api/refunds/retrieve).
    pub fn retrieve(client: &Client, refund_id: &str) -> Result<Refund, Error> {
        client.get(&format!("/refunds/{}", refund_id))
    }

    /// Updates a refund's properties.
    ///
    /// For more details see [https://stripe.com/docs/api/refunds/update](https://stripe.com/docs/api/refunds/update).
    pub fn update(
        client: &Client,
        refund_id: &str,
        metadata: Option<Metadata>,
    ) -> Result<Refund, Error> {
        client.post(&format!("/refunds/{}", refund_id), metadata)
    }

    /// List all refunds.
    ///
    /// For more details see [https://stripe.com/docs/api#list_refunds](https://stripe.com/docs/api#list_refunds).
    pub fn list(client: &Client, params: RefundListParams) -> Result<List<Refund>, Error> {
        client.get(&format!("/refunds?{}", qs::to_string(&params)?))
    }
}
