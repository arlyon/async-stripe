use stripe::{Client, Response};

impl stripe_treasury::treasury::credit_reversal::CreditReversal {
    /// Returns a list of CreditReversals.
    pub fn list(
        client: &Client,
        params: ListCreditReversal,
    ) -> Response<stripe_types::List<stripe_treasury::treasury::credit_reversal::CreditReversal>>
    {
        client.get_query("/treasury/credit_reversals", params)
    }
    /// Retrieves the details of an existing CreditReversal by passing the unique CreditReversal ID from either the CreditReversal creation request or CreditReversal list.
    pub fn retrieve(
        client: &Client,
        credit_reversal: &str,
        params: RetrieveCreditReversal,
    ) -> Response<stripe_treasury::treasury::credit_reversal::CreditReversal> {
        client.get_query(
            &format!(
                "/treasury/credit_reversals/{credit_reversal}",
                credit_reversal = credit_reversal
            ),
            params,
        )
    }
    /// Reverses a ReceivedCredit and creates a CreditReversal object.
    pub fn create(
        client: &Client,
        params: CreateCreditReversal,
    ) -> Response<stripe_treasury::treasury::credit_reversal::CreditReversal> {
        client.send_form("/treasury/credit_reversals", params, http_types::Method::Post)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCreditReversal<'a> {
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
    /// Only return CreditReversals for the ReceivedCredit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return CreditReversals for a given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListCreditReversalStatus>,
}
impl<'a> ListCreditReversal<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            financial_account,
            limit: Default::default(),
            received_credit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
/// Only return CreditReversals for a given status.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ListCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}

impl ListCreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Posted => "posted",
            Self::Processing => "processing",
        }
    }
}

impl std::str::FromStr for ListCreditReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "posted" => Ok(Self::Posted),
            "processing" => Ok(Self::Processing),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListCreditReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ListCreditReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCreditReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCreditReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCreditReversal<'a> {
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
    /// The ReceivedCredit to reverse.
    pub received_credit: &'a str,
}
impl<'a> CreateCreditReversal<'a> {
    pub fn new(received_credit: &'a str) -> Self {
        Self { expand: Default::default(), metadata: Default::default(), received_credit }
    }
}
