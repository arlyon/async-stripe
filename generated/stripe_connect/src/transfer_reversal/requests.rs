
/// When you create a new reversal, you must specify a transfer to create it on.
///
/// When reversing transfers, you can optionally reverse part of the transfer.
///
/// You can do so as many times as you wish until the entire transfer has been reversed.  Once entirely reversed, a transfer can’t be reversed again.
/// This method will return an error when called on an already-reversed transfer, or when trying to reverse more money than is left on a transfer.
pub fn create(client: &stripe::Client, id: &stripe_types::transfer_reversal::TransferReversalId, params: CreateTransferReversal) -> stripe::Response<stripe_types::TransferReversal> {
    client.send_form(&format!("/transfers/{id}/reversals", id = id), params, http_types::Method::Post)
}
/// You can see a list of the reversals belonging to a specific transfer.
///
/// Note that the 10 most recent reversals are always available by default on the transfer object.
/// If you need more than those 10, you can use this API method and the `limit` and `starting_after` parameters to page through additional reversals.
pub fn list(client: &stripe::Client, id: &stripe_types::transfer_reversal::TransferReversalId, params: ListTransferReversal) -> stripe::Response<stripe_types::List<stripe_types::TransferReversal>> {
    client.get_query(&format!("/transfers/{id}/reversals", id = id), params)
}
/// By default, you can see the 10 most recent reversals stored directly on the transfer object, but you can also retrieve details about a specific reversal stored on the transfer.
pub fn retrieve(client: &stripe::Client, id: &str, transfer: &stripe_types::transfer::TransferId, params: RetrieveTransferReversal) -> stripe::Response<stripe_types::TransferReversal> {
    client.get_query(&format!("/transfers/{transfer}/reversals/{id}", id = id, transfer = transfer), params)
}
/// Updates the specified reversal by setting the values of the parameters passed.
///
/// Any parameters not provided will be left unchanged.  This request only accepts metadata and description as arguments.
pub fn update(client: &stripe::Client, id: &str, transfer: &stripe_types::transfer::TransferId, params: UpdateTransferReversal) -> stripe::Response<stripe_types::TransferReversal> {
    client.send_form(&format!("/transfers/{transfer}/reversals/{id}", id = id, transfer = transfer), params, http_types::Method::Post)
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTransferReversal<'a> {
    /// A positive integer in cents (or local equivalent) representing how much of this transfer to reverse.
    ///
    /// Can only reverse up to the unreversed amount remaining of the transfer.
    /// Partial transfer reversals are only allowed for transfers to Stripe Accounts.
    /// Defaults to the entire transfer amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// An arbitrary string which you can attach to a reversal object.
    ///
    /// It is displayed alongside the reversal in the Dashboard.
    /// This will be unset if you POST an empty value.
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
    /// Boolean indicating whether the application fee should be refunded when reversing this transfer.
    ///
    /// If a full transfer reversal is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded with an amount proportional to the amount of the transfer reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
}
impl<'a> CreateTransferReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListTransferReversal<'a> {
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
}
impl<'a> ListTransferReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTransferReversal<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTransferReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTransferReversal<'a> {
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
impl<'a> UpdateTransferReversal<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
