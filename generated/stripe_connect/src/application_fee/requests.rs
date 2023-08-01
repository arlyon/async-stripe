
/// Returns a list of application fees you’ve previously collected.
///
/// The application fees are returned in sorted order, with the most recent fees appearing first.
pub fn list(
    client: &stripe::Client,
    params: ListApplicationFee,
) -> stripe::Response<stripe_types::List<stripe_types::application_fee::ApplicationFee>> {
    client.get_query("/application_fees", params)
}
/// Retrieves the details of an application fee that your account has collected.
///
/// The same information is returned when refunding the application fee.
pub fn retrieve(
    client: &stripe::Client,
    id: &stripe_types::application_fee::ApplicationFeeId,
    params: RetrieveApplicationFee,
) -> stripe::Response<stripe_types::application_fee::ApplicationFee> {
    client.get_query(&format!("/application_fees/{id}", id = id), params)
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListApplicationFee<'a> {
    /// Only return application fees for the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
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
}
impl<'a> ListApplicationFee<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveApplicationFee<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveApplicationFee<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
