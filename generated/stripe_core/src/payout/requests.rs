use stripe::{Client, Response};

impl stripe_core::payout::Payout {
    /// Retrieves the details of an existing payout.
    ///
    /// Supply the unique payout ID from either a payout creation request or the payout list, and Stripe will return the corresponding payout information.
    pub fn retrieve(
        client: &Client,
        payout: &stripe_core::payout::PayoutId,
        params: RetrievePayout,
    ) -> Response<stripe_core::payout::Payout> {
        client.get_query(&format!("/payouts/{payout}", payout = payout), params)
    }
    /// Returns a list of existing payouts sent to third-party bank accounts or that Stripe has sent you.
    ///
    /// The payouts are returned in sorted order, with the most recently created payouts appearing first.
    pub fn list(
        client: &Client,
        params: ListPayout,
    ) -> Response<stripe_types::List<stripe_core::payout::Payout>> {
        client.get_query("/payouts", params)
    }
    /// To send funds to your own bank account, you create a new payout object.
    ///
    /// Your [Stripe balance](https://stripe.com/docs/api#balance) must be able to cover the payout amount, or you’ll receive an “Insufficient Funds” error.  If your API key is in test mode, money won’t actually be sent, though everything else will occur as if in live mode.  If you are creating a manual payout on a Stripe account that uses multiple payment source types, you’ll need to specify the source type balance that the payout should draw from.
    /// The [balance object](https://stripe.com/docs/api#balance_object) details available and pending amounts by source type.
    pub fn create(client: &Client, params: CreatePayout) -> Response<stripe_core::payout::Payout> {
        client.send_form("/payouts", params, http_types::Method::Post)
    }
    /// Updates the specified payout by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    /// This request accepts only the metadata as arguments.
    pub fn update(
        client: &Client,
        payout: &stripe_core::payout::PayoutId,
        params: UpdatePayout,
    ) -> Response<stripe_core::payout::Payout> {
        client.send_form(
            &format!("/payouts/{payout}", payout = payout),
            params,
            http_types::Method::Post,
        )
    }
    /// A previously created payout can be canceled if it has not yet been paid out.
    ///
    /// Funds will be refunded to your available balance.
    /// You may not cancel automatic Stripe payouts.
    pub fn cancel(
        client: &Client,
        payout: &stripe_core::payout::PayoutId,
        params: CancelPayout,
    ) -> Response<stripe_core::payout::Payout> {
        client.send_form(
            &format!("/payouts/{payout}/cancel", payout = payout),
            params,
            http_types::Method::Post,
        )
    }
    /// Reverses a payout by debiting the destination bank account.
    ///
    /// Only payouts for connected accounts to US bank accounts may be reversed at this time.
    /// If the payout is in the `pending` status, `/v1/payouts/:id/cancel` should be used instead.  By requesting a reversal via `/v1/payouts/:id/reverse`, you confirm that the authorized signatory of the selected bank account has authorized the debit on the bank account and that no other authorization is required.
    pub fn reverse(
        client: &Client,
        payout: &stripe_core::payout::PayoutId,
        params: ReversePayout,
    ) -> Response<stripe_core::payout::Payout> {
        client.send_form(
            &format!("/payouts/{payout}/reverse", payout = payout),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListPayout<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// The ID of an external account - only return payouts sent to this external account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return payouts that have the given status: `pending`, `paid`, `failed`, or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}
impl<'a> ListPayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePayout<'a> {
    /// A positive integer in cents representing how much to payout.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The ID of a bank account or a card to send the payout to.
    ///
    /// If no destination is supplied, the default external account for the specified currency will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces for more information](https://stripe.com/blog/instant-payouts-for-marketplaces).).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<CreatePayoutMethod>,
    /// The balance type of your Stripe balance to draw this payout from.
    ///
    /// Balances for different payment sources are kept separately.
    /// You can find the amounts with the balances API.
    /// One of `bank_account`, `card`, or `fpx`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<CreatePayoutSourceType>,
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
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self {
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
/// The method used to send this payout, which can be `standard` or `instant`.
///
/// `instant` is only supported for payouts to debit cards.
/// (See [Instant payouts for marketplaces for more information](https://stripe.com/blog/instant-payouts-for-marketplaces).).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePayoutMethod {
    Instant,
    Standard,
}

impl CreatePayoutMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "instant",
            Self::Standard => "standard",
        }
    }
}

impl AsRef<str> for CreatePayoutMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePayoutMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The balance type of your Stripe balance to draw this payout from.
///
/// Balances for different payment sources are kept separately.
/// You can find the amounts with the balances API.
/// One of `bank_account`, `card`, or `fpx`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePayoutSourceType {
    BankAccount,
    Card,
    Fpx,
}

impl CreatePayoutSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
            Self::Card => "card",
            Self::Fpx => "fpx",
        }
    }
}

impl AsRef<str> for CreatePayoutSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePayoutSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
}
impl<'a> UpdatePayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelPayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelPayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReversePayout<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
}
impl<'a> ReversePayout<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
