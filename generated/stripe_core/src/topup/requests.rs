use stripe::{Client, Response};

impl stripe_core::topup::Topup {
    /// Top up the balance of an account.
    pub fn create(client: &Client, params: CreateTopup) -> Response<stripe_core::topup::Topup> {
        client.send_form("/topups", params, http_types::Method::Post)
    }
    /// Returns a list of top-ups.
    pub fn list(
        client: &Client,
        params: ListTopup,
    ) -> Response<stripe_types::List<stripe_core::topup::Topup>> {
        client.get_query("/topups", params)
    }
    /// Retrieves the details of a top-up that has previously been created.
    ///
    /// Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.
    pub fn retrieve(
        client: &Client,
        topup: &stripe_core::topup::TopupId,
        params: RetrieveTopup,
    ) -> Response<stripe_core::topup::Topup> {
        client.get_query(&format!("/topups/{topup}", topup = topup), params)
    }
    /// Updates the metadata of a top-up.
    ///
    /// Other top-up details are not editable by design.
    pub fn update(
        client: &Client,
        topup: &stripe_core::topup::TopupId,
        params: UpdateTopup,
    ) -> Response<stripe_core::topup::Topup> {
        client.send_form(
            &format!("/topups/{topup}", topup = topup),
            params,
            http_types::Method::Post,
        )
    }
    /// Cancels a top-up.
    ///
    /// Only pending top-ups can be canceled.
    pub fn cancel(
        client: &Client,
        topup: &stripe_core::topup::TopupId,
        params: CancelTopup,
    ) -> Response<stripe_core::topup::Topup> {
        client.send_form(
            &format!("/topups/{topup}/cancel", topup = topup),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTopup<'a> {
    /// A positive integer representing how much to transfer.
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
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The ID of a source to transfer funds from.
    ///
    /// For most users, this should be left unspecified which will use the bank account that was set up in the dashboard for the specified currency.
    /// In test mode, this can be a test bank token (see [Testing Top-ups](https://stripe.com/docs/connect/testing#testing-top-ups)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    /// Extra information about a top-up for the source's bank statement.
    ///
    /// Limited to 15 ASCII characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// A string that identifies this top-up as part of a group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> CreateTopup<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self {
            amount,
            currency,
            description: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            source: Default::default(),
            statement_descriptor: Default::default(),
            transfer_group: Default::default(),
        }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListTopup<'a> {
    /// A positive integer representing how much to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<stripe_types::RangeQueryTs>,
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// Only return top-ups that have the given status.
    ///
    /// One of `canceled`, `failed`, `pending` or `succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTopupStatus>,
}
impl<'a> ListTopup<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return top-ups that have the given status.
///
/// One of `canceled`, `failed`, `pending` or `succeeded`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ListTopupStatus {
    Canceled,
    Failed,
    Pending,
    Succeeded,
}

impl ListTopupStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ListTopupStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "failed" => Ok(Self::Failed),
            "pending" => Ok(Self::Pending),
            "succeeded" => Ok(Self::Succeeded),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTopupStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTopupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ListTopupStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTopup<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTopup<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTopup<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateTopup<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelTopup<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelTopup<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
