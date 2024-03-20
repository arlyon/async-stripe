#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListClimateSupplier<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListClimateSupplier<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListClimateSupplier<'a> {
    /// Lists all available Climate supplier objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::ClimateSupplier>> {
        client.get_query("/climate/suppliers", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::ClimateSupplier>> {
        stripe::ListPaginator::from_list_params("/climate/suppliers", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveClimateSupplier<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveClimateSupplier<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveClimateSupplier<'a> {
    /// Retrieves a Climate supplier object.
    pub fn send(
        &self,
        client: &stripe::Client,
        supplier: &stripe_misc::ClimateSupplierId,
    ) -> stripe::Response<stripe_misc::ClimateSupplier> {
        client.get_query(&format!("/climate/suppliers/{supplier}"), self)
    }
}
