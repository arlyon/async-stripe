use crate::config::{Client, Response};
use crate::params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use crate::resources::Currency;
use serde_derive::{Deserialize, Serialize};

/// The set of parameters that can be used when creating a payout object.
///
/// For more details see [https://stripe.com/docs/api/payouts/create](https://stripe.com/docs/api/payouts/create)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PayoutParams<'a> {
    pub amount: u64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PayoutMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PayoutSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

/// The set of parameters that can be used when listing payouts.
///
/// For more details see [https://stripe.com/docs/api/payouts/list](https://stripe.com/docs/api/payouts/list)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PayoutListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PayoutStatus>,
}

/// An enum representing the possible values of a `PayOut`'s `failure_code` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/failures](https://stripe.com/docs/api/payouts/failures)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutFailureCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    CouldNotProcess,
    DebitNotAuthorized,
    Declined,
    InsufficientFunds,
    InvalidAccountNumber,
    IncorrectAccountHolderName,
    InvalidCurrency,
    NoAccount,
    UnsupportedCard,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}
/// An enum representing the possible values of a `PayOut`'s `method` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-method](https://stripe.com/docs/api/payouts/object#payout_object-method)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethod {
    Standard,
    Instant,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}
/// An enum representing the possible values of a `PayOut`'s `source_type` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-source_type](https://stripe.com/docs/api/payouts/object#payout_object-source_type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutSourceType {
    Card,
    BankAccount,
    AlipayAccount,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}
/// An enum representing the possible values of a `PayOut`'s `status` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-status](https://stripe.com/docs/api/payouts/object#payout_object-status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Paid,
    Pending,
    InTransit,
    Canceled,
    Failed,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `PayOut`'s `payout_type` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-type](https://stripe.com/docs/api/payouts/object#payout_object-type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// The resource representing a Stripe payout.
///
/// For more details see https://stripe.com/docs/api#payout_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub failure_code: Option<PayoutFailureCode>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub method: PayoutMethod,
    pub source_type: PayoutSourceType,
    pub statement_descriptor: Option<String>,
    pub status: PayoutStatus,
    #[serde(rename = "type")]
    pub payout_type: PayoutType,
}

impl Payout {
    /// Creates a new payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/create](https://stripe.com/docs/api/payouts/create).
    pub fn create(client: &Client, params: PayoutParams<'_>) -> Response<Payout> {
        client.post_form("/payouts", params)
    }

    /// Retrieves the details of a payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/retrieve](https://stripe.com/docs/api/payouts/retrieve).
    pub fn retrieve(client: &Client, payout_id: &str) -> Response<Payout> {
        client.get(&format!("/payouts/{}", payout_id))
    }

    /// Updates a payout's properties.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/update](https://stripe.com/docs/api/payouts/update).
    pub fn update(
        client: &Client,
        payout_id: &str,
        metadata: Option<Metadata>,
    ) -> Response<Payout> {
        client.post_form(&format!("/payouts/{}", payout_id), metadata)
    }

    /// List all payouts.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/list](https://stripe.com/docs/api/payouts/list).
    pub fn list(client: &Client, params: PayoutListParams<'_>) -> Response<List<Payout>> {
        client.get_query("/payouts", &params)
    }

    /// Cancels the payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/cancel](https://stripe.com/docs/api/payouts/cancel).
    pub fn cancel(client: &Client, payout_id: &str) -> Response<Payout> {
        client.post(&format!("/payouts/{}/cancel", payout_id))
    }
}

impl Identifiable for Payout {
    fn id(&self) -> &str {
        &self.id
    }
}
