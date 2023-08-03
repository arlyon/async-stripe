
/// List apple pay domains.
pub fn list(client: &stripe::Client, params: ListApplePayDomain) -> stripe::Response<stripe_types::List<stripe_misc::ApplePayDomain>> {
    client.get_query("/apple_pay/domains", params)
}
/// Create an apple pay domain.
pub fn create(client: &stripe::Client, params: CreateApplePayDomain) -> stripe::Response<stripe_misc::ApplePayDomain> {
    client.send_form("/apple_pay/domains", params, http_types::Method::Post)
}
/// Retrieve an apple pay domain.
pub fn retrieve(client: &stripe::Client, domain: &stripe_misc::apple_pay_domain::ApplePayDomainId, params: RetrieveApplePayDomain) -> stripe::Response<stripe_misc::ApplePayDomain> {
    client.get_query(&format!("/apple_pay/domains/{domain}", domain = domain), params)
}
/// Delete an apple pay domain.
pub fn delete(client: &stripe::Client, domain: &stripe_misc::apple_pay_domain::ApplePayDomainId) -> stripe::Response<stripe_misc::DeletedApplePayDomain> {
    client.send(&format!("/apple_pay/domains/{domain}", domain = domain), http_types::Method::Delete)
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListApplePayDomain<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<&'a str>,
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
impl<'a> ListApplePayDomain<'a> {
    pub fn new() -> Self {
        Self::default()
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
        Self { domain_name, expand: Default::default() }
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
