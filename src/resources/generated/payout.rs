// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::PayoutId;
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, Currency, PayoutDestinationUnion};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Payout".
///
/// For more details see <https://stripe.com/docs/api/payouts/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Payout {
    /// Unique identifier for the object.
    pub id: PayoutId,

    /// The amount (in cents (or local equivalent)) that transfers to your bank account or debit card.
    pub amount: i64,

    /// Date that you can expect the payout to arrive in the bank.
    ///
    /// This factors in delays to account for weekends or bank holidays.
    pub arrival_date: Timestamp,

    /// Returns `true` if the payout is created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule) and `false` if it's [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,

    /// ID of the balance transaction that describes the impact of this payout on your account balance.
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// ID of the bank account or card the payout is sent to.
    pub destination: Option<Expandable<PayoutDestinationUnion>>,

    /// If the payout fails or cancels, this is the ID of the balance transaction that reverses the initial balance transaction and returns the funds from the failed payout back in your balance.
    pub failure_balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Error code that provides a reason for a payout failure, if available.
    ///
    /// View our [list of failure codes](https://stripe.com/docs/api#payout_failures).
    pub failure_code: Option<String>,

    /// Message that provides the reason for a payout failure, if available.
    pub failure_message: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is supported for payouts to debit cards and bank accounts in certain countries.
    /// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
    pub method: String,

    /// If the payout reverses another, this is the ID of the original payout.
    pub original_payout: Option<Expandable<Payout>>,

    /// If `completed`, you can use the [Balance Transactions API](https://stripe.com/docs/api/balance_transactions/list#balance_transaction_list-payout) to list all balance transactions that are paid out in this payout.
    pub reconciliation_status: PayoutReconciliationStatus,

    /// If the payout reverses, this is the ID of the payout that reverses this payout.
    pub reversed_by: Option<Expandable<Payout>>,

    /// The source balance this payout came from, which can be one of the following: `card`, `fpx`, or `bank_account`.
    pub source_type: String,

    /// Extra information about a payout that displays on the user's bank statement.
    pub statement_descriptor: Option<String>,

    /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
    ///
    /// A payout is `pending` until it's submitted to the bank, when it becomes `in_transit`.
    /// The status changes to `paid` if the transaction succeeds, or to `failed` or `canceled` (within 5 business days).
    /// Some payouts that fail might initially show as `paid`, then change to `failed`.
    pub status: String,

    /// Can be `bank_account` or `card`.
    #[serde(rename = "type")]
    pub type_: PayoutType,
}

impl Payout {
    /// Returns a list of existing payouts sent to third-party bank accounts or payouts that Stripe sent to you.
    ///
    /// The payouts return in sorted order, with the most recently created payouts appearing first.
    pub fn list(client: &Client, params: &ListPayouts<'_>) -> Response<List<Payout>> {
        client.get_query("/payouts", params)
    }

    /// To send funds to your own bank account, create a new payout object.
    ///
    /// Your [Stripe balance](https://stripe.com/docs/api#balance) must cover the payout amount.
    /// If it doesn’t, you receive an “Insufficient Funds” error.  If your API key is in test mode, money won’t actually be sent, though every other action occurs as if you’re in live mode.  If you create a manual payout on a Stripe account that uses multiple payment source types, you need to specify the source type balance that the payout draws from.
    /// The [balance object](https://stripe.com/docs/api#balance_object) details available and pending amounts by source type.
    pub fn create(client: &Client, params: CreatePayout<'_>) -> Response<Payout> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/payouts", &params)
    }

    /// Retrieves the details of an existing payout.
    ///
    /// Supply the unique payout ID from either a payout creation request or the payout list.
    /// Stripe returns the corresponding payout information.
    pub fn retrieve(client: &Client, id: &PayoutId, expand: &[&str]) -> Response<Payout> {
        client.get_query(&format!("/payouts/{}", id), Expand { expand })
    }

    /// Updates the specified payout by setting the values of the parameters you pass.
    ///
    /// We don’t change parameters that you don’t provide.
    /// This request only accepts the metadata as arguments.
    pub fn update(client: &Client, id: &PayoutId, params: UpdatePayout<'_>) -> Response<Payout> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/payouts/{}", id), &params)
    }
}

impl Object for Payout {
    type Id = PayoutId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payout"
    }
}

/// The parameters for `Payout::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreatePayout<'a> {
    /// A positive integer in cents representing how much to payout.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// The ID of a bank account or a card to send the payout to.
    ///
    /// If you don't provide a destination, we use the default external account for the specified currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

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

    /// The method used to send this payout, which is `standard` or `instant`.
    ///
    /// We support `instant` for payouts to debit cards and bank accounts in certain countries.
    /// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PayoutMethod>,

    /// The balance type of your Stripe balance to draw this payout from.
    ///
    /// Balances for different payment sources are kept separately.
    /// You can find the amounts with the Balances API.
    /// One of `bank_account`, `card`, or `fpx`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PayoutSourceType>,

    /// A string that displays on the recipient's bank or card statement (up to 22 characters).
    ///
    /// A `statement_descriptor` that's longer than 22 characters return an error.
    /// Most banks truncate this information and display it inconsistently.
    /// Some banks might not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

impl<'a> CreatePayout<'a> {
    pub fn new(amount: i64, currency: Currency) -> Self {
        CreatePayout {
            amount,
            currency,
            description: Default::default(),
            destination: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            method: Default::default(),
            source_type: Default::default(),
            statement_descriptor: Default::default(),
        }
    }
}

/// The parameters for `Payout::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPayouts<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// The ID of an external account - only return payouts sent to this external account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PayoutId>,

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
    pub starting_after: Option<PayoutId>,

    /// Only return payouts that have the given status: `pending`, `paid`, `failed`, or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

impl<'a> ListPayouts<'a> {
    pub fn new() -> Self {
        ListPayouts {
            arrival_date: Default::default(),
            created: Default::default(),
            destination: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
impl Paginable for ListPayouts<'_> {
    type O = Payout;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `Payout::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePayout<'a> {
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

impl<'a> UpdatePayout<'a> {
    pub fn new() -> Self {
        UpdatePayout { expand: Default::default(), metadata: Default::default() }
    }
}

/// An enum representing the possible values of an `CreatePayout`'s `method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethod {
    Instant,
    Standard,
}

impl PayoutMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PayoutMethod::Instant => "instant",
            PayoutMethod::Standard => "standard",
        }
    }
}

impl AsRef<str> for PayoutMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayoutMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PayoutMethod {
    fn default() -> Self {
        Self::Instant
    }
}

/// An enum representing the possible values of an `Payout`'s `reconciliation_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutReconciliationStatus {
    Completed,
    InProgress,
    NotApplicable,
}

impl PayoutReconciliationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PayoutReconciliationStatus::Completed => "completed",
            PayoutReconciliationStatus::InProgress => "in_progress",
            PayoutReconciliationStatus::NotApplicable => "not_applicable",
        }
    }
}

impl AsRef<str> for PayoutReconciliationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayoutReconciliationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PayoutReconciliationStatus {
    fn default() -> Self {
        Self::Completed
    }
}

/// An enum representing the possible values of an `CreatePayout`'s `source_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutSourceType {
    BankAccount,
    Card,
    Fpx,
}

impl PayoutSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            PayoutSourceType::BankAccount => "bank_account",
            PayoutSourceType::Card => "card",
            PayoutSourceType::Fpx => "fpx",
        }
    }
}

impl AsRef<str> for PayoutSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayoutSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PayoutSourceType {
    fn default() -> Self {
        Self::BankAccount
    }
}

/// An enum representing the possible values of an `Payout`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,
}

impl PayoutType {
    pub fn as_str(self) -> &'static str {
        match self {
            PayoutType::BankAccount => "bank_account",
            PayoutType::Card => "card",
        }
    }
}

impl AsRef<str> for PayoutType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PayoutType {
    fn default() -> Self {
        Self::BankAccount
    }
}
