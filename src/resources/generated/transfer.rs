// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ChargeId, TransferId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{Account, BalanceTransaction, Charge, Currency, TransferReversal};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Transfer".
///
/// For more details see <https://stripe.com/docs/api/transfers/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Transfer {
    /// Unique identifier for the object.
    pub id: TransferId,

    /// Amount in cents (or local equivalent) to be transferred.
    pub amount: i64,

    /// Amount in cents (or local equivalent) reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,

    /// Balance transaction that describes the impact of this transfer on your account balance.
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time that this record of the transfer was first created.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// ID of the Stripe account the transfer was sent to.
    pub destination: Option<Expandable<Account>>,

    /// If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<Expandable<Charge>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// A list of reversals that have been applied to the transfer.
    pub reversals: List<TransferReversal>,

    /// Whether the transfer has been fully reversed.
    ///
    /// If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,

    /// ID of the charge or payment that was used to fund the transfer.
    ///
    /// If null, the transfer was funded from the available balance.
    pub source_transaction: Option<Expandable<Charge>>,

    /// The source balance this transfer came from.
    ///
    /// One of `card`, `fpx`, or `bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<TransferSourceType>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}

impl Transfer {
    /// Returns a list of existing transfers sent to connected accounts.
    ///
    /// The transfers are returned in sorted order, with the most recently created transfers appearing first.
    pub fn list(client: &Client, params: &ListTransfers<'_>) -> Response<List<Transfer>> {
        client.get_query("/transfers", params)
    }

    /// To send funds from your Stripe account to a connected account, you create a new transfer object.
    ///
    /// Your [Stripe balance](https://stripe.com/docs/api#balance) must be able to cover the transfer amount, or you’ll receive an “Insufficient Funds” error.
    pub fn create(client: &Client, params: CreateTransfer<'_>) -> Response<Transfer> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/transfers", &params)
    }

    /// Retrieves the details of an existing transfer.
    ///
    /// Supply the unique transfer ID from either a transfer creation request or the transfer list, and Stripe will return the corresponding transfer information.
    pub fn retrieve(client: &Client, id: &TransferId, expand: &[&str]) -> Response<Transfer> {
        client.get_query(&format!("/transfers/{}", id), Expand { expand })
    }

    /// Updates the specified transfer by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request accepts only metadata as an argument.
    pub fn update(
        client: &Client,
        id: &TransferId,
        params: UpdateTransfer<'_>,
    ) -> Response<Transfer> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/transfers/{}", id), &params)
    }
}

impl Object for Transfer {
    type Id = TransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "transfer"
    }
}

/// The parameters for `Transfer::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateTransfer<'a> {
    /// A positive integer in cents (or local equivalent) representing how much to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// 3-letter [ISO code for currency](https://stripe.com/docs/payouts).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// The ID of a connected Stripe account.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub destination: String,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// You can use this parameter to transfer funds from a charge before they are added to your available balance.
    ///
    /// A pending balance will transfer immediately but the funds will not become available until the original charge becomes available.
    /// [See the Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-availability) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<ChargeId>,

    /// The source balance to use for this transfer.
    ///
    /// One of `bank_account`, `card`, or `fpx`.
    /// For most users, this will default to `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<TransferSourceType>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

impl<'a> CreateTransfer<'a> {
    pub fn new(currency: Currency, destination: String) -> Self {
        CreateTransfer {
            amount: Default::default(),
            currency,
            description: Default::default(),
            destination,
            expand: Default::default(),
            metadata: Default::default(),
            source_transaction: Default::default(),
            source_type: Default::default(),
            transfer_group: Default::default(),
        }
    }
}

/// The parameters for `Transfer::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTransfers<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return transfers for the destination specified by this account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TransferId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<TransferId>,

    /// Only return transfers with the specified transfer group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

impl<'a> ListTransfers<'a> {
    pub fn new() -> Self {
        ListTransfers {
            created: Default::default(),
            destination: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            transfer_group: Default::default(),
        }
    }
}
impl Paginable for ListTransfers<'_> {
    type O = Transfer;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `Transfer::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateTransfer<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> UpdateTransfer<'a> {
    pub fn new() -> Self {
        UpdateTransfer {
            description: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `CreateTransfer`'s `source_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransferSourceType {
    BankAccount,
    Card,
    Fpx,
}

impl TransferSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            TransferSourceType::BankAccount => "bank_account",
            TransferSourceType::Card => "card",
            TransferSourceType::Fpx => "fpx",
        }
    }
}

impl AsRef<str> for TransferSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransferSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TransferSourceType {
    fn default() -> Self {
        Self::BankAccount
    }
}
