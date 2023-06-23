use stripe::{Client, Response};

impl stripe_treasury::treasury::debit_reversal::DebitReversal {
    /// Reverses a ReceivedDebit and creates a DebitReversal object.
    pub fn create(
        client: &Client,
        params: CreateDebitReversal,
    ) -> Response<stripe_treasury::treasury::debit_reversal::DebitReversal> {
        client.send_form("/treasury/debit_reversals", params, http_types::Method::Post)
    }
    /// Retrieves a DebitReversal object.
    pub fn retrieve(
        client: &Client,
        debit_reversal: &str,
        params: RetrieveDebitReversal,
    ) -> Response<stripe_treasury::treasury::debit_reversal::DebitReversal> {
        client.get_query(
            &format!("/treasury/debit_reversals/{debit_reversal}", debit_reversal = debit_reversal),
            params,
        )
    }
    /// Returns a list of DebitReversals.
    pub fn list(
        client: &Client,
        params: ListDebitReversal,
    ) -> Response<stripe_types::List<stripe_treasury::treasury::debit_reversal::DebitReversal>>
    {
        client.get_query("/treasury/debit_reversals", params)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateDebitReversal<'a> {
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
    /// The ReceivedDebit to reverse.
    pub received_debit: &'a str,
}
impl<'a> CreateDebitReversal<'a> {
    pub fn new(received_debit: &'a str) -> Self {
        Self { expand: Default::default(), metadata: Default::default(), received_debit }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveDebitReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveDebitReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListDebitReversal<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Returns objects associated with this FinancialAccount.
    pub financial_account: &'a str,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return DebitReversals for the ReceivedDebit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<&'a str>,
    /// Only return DebitReversals for a given resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ListDebitReversalResolution>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return DebitReversals for a given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListDebitReversalStatus>,
}
impl<'a> ListDebitReversal<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            financial_account,
            limit: Default::default(),
            received_debit: Default::default(),
            resolution: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
/// Only return DebitReversals for a given resolution.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListDebitReversalResolution {
    Lost,
    Won,
}

impl ListDebitReversalResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lost => "lost",
            Self::Won => "won",
        }
    }
}

impl AsRef<str> for ListDebitReversalResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListDebitReversalResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Only return DebitReversals for a given status.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListDebitReversalStatus {
    Canceled,
    Completed,
    Processing,
}

impl ListDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Completed => "completed",
            Self::Processing => "processing",
        }
    }
}

impl AsRef<str> for ListDebitReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
