use crate::{Client, Response};

impl crate::fee_refund::FeeRefund {
    /// Refunds an application fee that has previously been collected but not yet refunded.
    /// Funds will be refunded to the Stripe account from which the fee was originally collected.
    ///
    /// You can optionally refund only part of an application fee.
    /// You can do so multiple times, until the entire fee has been refunded.
    ///
    /// Once entirely refunded, an application fee can’t be refunded again.
    /// This method will raise an error when called on an already-refunded application fee,
    /// or when trying to refund more money than is left on an application fee.
    pub fn create(
        client: &Client,
        id: &str,
        params: CreateFeeRefund,
    ) -> Response<crate::fee_refund::FeeRefund> {
        client.send_form(
            &format!("/application_fees/{id}/refunds", id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// You can see a list of the refunds belonging to a specific application fee.
    ///
    /// Note that the 10 most recent refunds are always available by default on the application fee object.
    /// If you need more than those 10, you can use this API method and the `limit` and `starting_after` parameters to page through additional refunds.
    pub fn list(
        client: &Client,
        id: &str,
        params: ListFeeRefund,
    ) -> Response<crate::List<crate::fee_refund::FeeRefund>> {
        client.get_query(&format!("/application_fees/{id}/refunds", id = id), params)
    }
    /// By default, you can see the 10 most recent refunds stored directly on the application fee object, but you can also retrieve details about a specific refund stored on the application fee.
    pub fn retrieve(
        client: &Client,
        fee: &str,
        id: &str,
        params: RetrieveFeeRefund,
    ) -> Response<crate::fee_refund::FeeRefund> {
        client
            .get_query(&format!("/application_fees/{fee}/refunds/{id}", fee = fee, id = id), params)
    }
    /// Updates the specified application fee refund by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request only accepts metadata as an argument.
    pub fn update(
        client: &Client,
        fee: &str,
        id: &str,
        params: UpdateFeeRefund,
    ) -> Response<crate::fee_refund::FeeRefund> {
        client.send_form(
            &format!("/application_fees/{fee}/refunds/{id}", fee = fee, id = id),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateFeeRefund<'a> {
    /// A positive integer, in _cents (or local equivalent)_, representing how much of this fee to refund.
    ///
    /// Can refund only up to the remaining unrefunded amount of the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
}
impl<'a> CreateFeeRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListFeeRefund<'a> {
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
impl<'a> ListFeeRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFeeRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFeeRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFeeRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
}
impl<'a> UpdateFeeRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}