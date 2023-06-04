use crate::{Client, Response};

impl crate::country_spec::CountrySpec {
    /// Lists all Country Spec objects available in the API.
    pub fn list(
        client: &Client,
        params: ListCountrySpec,
    ) -> Response<crate::List<crate::country_spec::CountrySpec>> {
        client.get_query("/country_specs", params)
    }
    /// Returns a Country Spec for a given Country code.
    pub fn retrieve(
        client: &Client,
        country: &str,
        params: RetrieveCountrySpec,
    ) -> Response<crate::country_spec::CountrySpec> {
        client.get_query(&format!("/country_specs/{country}", country = country), params)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCountrySpec<'a> {
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
impl<'a> ListCountrySpec<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCountrySpec<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCountrySpec<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
