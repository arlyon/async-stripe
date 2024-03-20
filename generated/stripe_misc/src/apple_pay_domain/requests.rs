#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteApplePayDomain {}
impl DeleteApplePayDomain {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteApplePayDomain {
    /// Delete an apple pay domain.
    pub fn send(
        &self,
        client: &stripe::Client,
        domain: &str,
    ) -> stripe::Response<stripe_misc::DeletedApplePayDomain> {
        client.send_form(&format!("/apple_pay/domains/{domain}"), self, http_types::Method::Delete)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListApplePayDomain<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<&'a str>,
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
impl<'a> ListApplePayDomain<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListApplePayDomain<'a> {
    /// List apple pay domains.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::ApplePayDomain>> {
        client.get_query("/apple_pay/domains", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::ApplePayDomain>> {
        stripe::ListPaginator::from_list_params("/apple_pay/domains", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveApplePayDomain<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveApplePayDomain<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveApplePayDomain<'a> {
    /// Retrieve an apple pay domain.
    pub fn send(
        &self,
        client: &stripe::Client,
        domain: &str,
    ) -> stripe::Response<stripe_misc::ApplePayDomain> {
        client.get_query(&format!("/apple_pay/domains/{domain}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateApplePayDomain<'a> {
    pub domain_name: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreateApplePayDomain<'a> {
    pub fn new(domain_name: &'a str) -> Self {
        Self { domain_name, expand: None }
    }
}
impl<'a> CreateApplePayDomain<'a> {
    /// Create an apple pay domain.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_misc::ApplePayDomain> {
        client.send_form("/apple_pay/domains", self, http_types::Method::Post)
    }
}
