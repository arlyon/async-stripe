
/// A list of [all tax codes available](https://stripe.com/docs/tax/tax-categories) to add to Products in order to allow specific tax calculations.
pub fn list(
    client: &stripe::Client,
    params: ListTaxCode,
) -> stripe::Response<stripe_types::List<stripe_types::tax_code::TaxCode>> {
    client.get_query("/tax_codes", params)
}
/// Retrieves the details of an existing tax code.
///
/// Supply the unique tax code ID and Stripe will return the corresponding tax code information.
pub fn retrieve(
    client: &stripe::Client,
    id: &stripe_types::tax_code::TaxCodeId,
    params: RetrieveTaxCode,
) -> stripe::Response<stripe_types::tax_code::TaxCode> {
    client.get_query(&format!("/tax_codes/{id}", id = id), params)
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListTaxCode<'a> {
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
impl<'a> ListTaxCode<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTaxCode<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTaxCode<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
