#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryCreditReversal<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Returns objects associated with this FinancialAccount.
    pub financial_account: &'a str,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return CreditReversals for the ReceivedCredit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return CreditReversals for a given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_treasury::TreasuryCreditReversalStatus>,
}
impl<'a> ListTreasuryCreditReversal<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            received_credit: None,
            starting_after: None,
            status: None,
        }
    }
}
impl<'a> ListTreasuryCreditReversal<'a> {
    /// Returns a list of CreditReversals.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryCreditReversal>> {
        client.get_query("/treasury/credit_reversals", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryCreditReversal>> {
        stripe::ListPaginator::from_list_params("/treasury/credit_reversals", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryCreditReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryCreditReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryCreditReversal<'a> {
    /// Retrieves the details of an existing CreditReversal by passing the unique CreditReversal ID from either the CreditReversal creation request or CreditReversal list.
    pub fn send(
        &self,
        client: &stripe::Client,
        credit_reversal: &stripe_treasury::TreasuryCreditReversalId,
    ) -> stripe::Response<stripe_treasury::TreasuryCreditReversal> {
        client.get_query(&format!("/treasury/credit_reversals/{credit_reversal}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryCreditReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The ReceivedCredit to reverse.
    pub received_credit: &'a str,
}
impl<'a> CreateTreasuryCreditReversal<'a> {
    pub fn new(received_credit: &'a str) -> Self {
        Self { expand: None, metadata: None, received_credit }
    }
}
impl<'a> CreateTreasuryCreditReversal<'a> {
    /// Reverses a ReceivedCredit and creates a CreditReversal object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_treasury::TreasuryCreditReversal> {
        client.send_form("/treasury/credit_reversals", self, http_types::Method::Post)
    }
}
