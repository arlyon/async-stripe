// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::PayoutId;
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, BankAccount, Card, Currency};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Payout".
///
/// For more details see [https://stripe.com/docs/api/payouts/object](https://stripe.com/docs/api/payouts/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Payout {
    /// Unique identifier for the object.
    pub id: PayoutId,

    /// Amount (in %s) to be transferred to your bank account or debit card.
    pub amount: i64,

    /// Date the payout is expected to arrive in the bank.
    ///
    /// This factors in delays like weekends or bank holidays.
    pub arrival_date: Timestamp,

    /// Returns `true` if the payout was created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule), and `false` if it was [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,

    /// ID of the balance transaction that describes the impact of this payout on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID of the bank account or card the payout was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Expandable<PayoutDestination>>,

    /// If the payout failed or was canceled, this will be the ID of the balance transaction that reversed the initial balance transaction, and puts the funds from the failed payout back in your balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Error code explaining reason for payout failure if available.
    ///
    /// See [Types of payout failures](https://stripe.com/docs/api#payout_failures) for a list of failure codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,

    /// Message to user further explaining reason for payout failure if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces](https://stripe.com/blog/instant-payouts-for-marketplaces) for more information.).
    pub method: String,

    /// The source balance this payout came from.
    ///
    /// One of `card`, `fpx`, or `bank_account`.
    pub source_type: String,

    /// Extra information about a payout to be displayed on the user's bank statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
    ///
    /// A payout is `pending` until it is submitted to the bank, when it becomes `in_transit`.
    /// The status then changes to `paid` if the transaction goes through, or to `failed` or `canceled` (within 5 business days).
    /// Some failed payouts may initially show as `paid` but then change to `failed`.
    pub status: String,

    /// Can be `bank_account` or `card`.
    #[serde(rename = "type")]
    pub type_: PayoutType,
}

impl Payout {
    /// Returns a list of existing payouts sent to third-party bank accounts or that Stripe has sent you.
    ///
    /// The payouts are returned in sorted order, with the most recently created payouts appearing first.
    pub fn list(client: &Client, params: ListPayouts<'_>) -> Response<List<Payout>> {
        client.get_query("/payouts", &params)
    }

    /// To send funds to your own bank account, you create a new payout object.
    ///
    /// Your [Stripe balance](https://stripe.com/docs/api#balance) must be able to cover the payout amount, or you’ll receive an “Insufficient Funds” error.  If your API key is in test mode, money won’t actually be sent, though everything else will occur as if in live mode.  If you are creating a manual payout on a Stripe account that uses multiple payment source types, you’ll need to specify the source type balance that the payout should draw from.
    /// The [balance object](https://stripe.com/docs/api#balance_object) details available and pending amounts by source type.
    pub fn create(client: &Client, params: CreatePayout<'_>) -> Response<Payout> {
        client.post_form("/payouts", &params)
    }

    /// Retrieves the details of an existing payout.
    ///
    /// Supply the unique payout ID from either a payout creation request or the payout list, and Stripe will return the corresponding payout information.
    pub fn retrieve(client: &Client, id: &PayoutId, expand: &[&str]) -> Response<Payout> {
        client.get_query(&format!("/payouts/{}", id), &Expand { expand })
    }

    /// Updates the specified payout by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    /// This request accepts only the metadata as arguments.
    pub fn update(client: &Client, id: &PayoutId, params: UpdatePayout<'_>) -> Response<Payout> {
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

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces for more information](https://stripe.com/blog/instant-payouts-for-marketplaces).).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PayoutMethod>,

    /// The balance type of your Stripe balance to draw this payout from.
    ///
    /// Balances for different payment sources are kept separately.
    /// You can find the amounts with the balances API.
    /// One of `bank_account`, `card`, or `fpx`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PayoutSourceType>,

    /// A string to be displayed on the recipient's bank or card statement.
    ///
    /// This may be at most 22 characters.
    /// Attempting to use a `statement_descriptor` longer than 22 characters will return an error.
    /// Note: Most banks will truncate this information and/or display it inconsistently.
    /// Some may not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

impl<'a> CreatePayout<'a> {
    pub fn new(amount: i64, currency: Currency) -> Self {
        CreatePayout {
            amount,
            currency,
            description: Default::default(),
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
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}

/// The parameters for `Payout::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum PayoutDestination {
    BankAccount(BankAccount),
    Card(Card),
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
