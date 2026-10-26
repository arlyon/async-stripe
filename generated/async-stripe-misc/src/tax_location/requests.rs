use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListTaxLocationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(rename = "type")]
    type_: stripe_misc::TaxLocationType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTaxLocationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListTaxLocationBuilder").finish_non_exhaustive()
    }
}
impl ListTaxLocationBuilder {
    fn new(type_: impl Into<stripe_misc::TaxLocationType>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: type_.into(),
        }
    }
}
/// Retrieve a list of all tax locations.
/// Tax locations can represent the venues for services, tickets, or other product types.
///
/// The response includes detailed information for each tax location, such as its address, type, and description.
///
/// You can paginate through the list by using the `limit` parameter to control the number of results returned in each request.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListTaxLocation {
    inner: ListTaxLocationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTaxLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListTaxLocation").finish_non_exhaustive()
    }
}
impl ListTaxLocation {
    /// Construct a new `ListTaxLocation`.
    pub fn new(type_: impl Into<stripe_misc::TaxLocationType>) -> Self {
        Self { inner: ListTaxLocationBuilder::new(type_.into()) }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListTaxLocation {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::TaxLocation>> {
        stripe_client_core::ListPaginator::new_list("/tax/locations", &self.inner)
    }
}

impl StripeRequest for ListTaxLocation {
    type Output = stripe_types::List<stripe_misc::TaxLocation>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax/locations").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveTaxLocationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveTaxLocationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveTaxLocationBuilder").finish_non_exhaustive()
    }
}
impl RetrieveTaxLocationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Fetch the details of a specific tax location using its unique identifier.
/// Use a tax location to calculate taxes based on the location of the end product, such as a performance, instead of the customer address.
/// For more details, check the [integration guide](https://docs.stripe.com/tax/tax-for-tickets/integration-guide).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveTaxLocation {
    inner: RetrieveTaxLocationBuilder,
    location: stripe_misc::TaxLocationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveTaxLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveTaxLocation").finish_non_exhaustive()
    }
}
impl RetrieveTaxLocation {
    /// Construct a new `RetrieveTaxLocation`.
    pub fn new(location: impl Into<stripe_misc::TaxLocationId>) -> Self {
        Self { location: location.into(), inner: RetrieveTaxLocationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTaxLocation {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveTaxLocation {
    type Output = stripe_misc::TaxLocation;

    fn build(&self) -> RequestBuilder {
        let location = &self.location;
        RequestBuilder::new(StripeMethod::Get, format!("/tax/locations/{location}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateTaxLocationBuilder {
    address: CreateTaxLocationAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(rename = "type")]
    type_: stripe_misc::TaxLocationType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxLocationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxLocationBuilder").finish_non_exhaustive()
    }
}
impl CreateTaxLocationBuilder {
    fn new(
        address: impl Into<CreateTaxLocationAddress>,
        type_: impl Into<stripe_misc::TaxLocationType>,
    ) -> Self {
        Self { address: address.into(), description: None, expand: None, type_: type_.into() }
    }
}
/// The physical address of the tax location.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxLocationAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix, such as "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxLocationAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxLocationAddress").finish_non_exhaustive()
    }
}
impl CreateTaxLocationAddress {
    pub fn new(country: impl Into<String>) -> Self {
        Self {
            city: None,
            country: country.into(),
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
        }
    }
}
/// Create a tax location to use in calculating taxes for a service, ticket, or other type of product.
/// The resulting object contains the ID, address, type, and description of the tax location.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxLocation {
    inner: CreateTaxLocationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxLocation").finish_non_exhaustive()
    }
}
impl CreateTaxLocation {
    /// Construct a new `CreateTaxLocation`.
    pub fn new(
        address: impl Into<CreateTaxLocationAddress>,
        type_: impl Into<stripe_misc::TaxLocationType>,
    ) -> Self {
        Self { inner: CreateTaxLocationBuilder::new(address.into(), type_.into()) }
    }
    /// Details to identify the tax location by its venue, types of events held, or available services, such as "A spacious auditorium suitable for large concerts and events.".
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateTaxLocation {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateTaxLocation {
    type Output = stripe_misc::TaxLocation;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax/locations").form(&self.inner)
    }
}
