use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTaxRateBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListTaxRateBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            created: None,
            ending_before: None,
            expand: None,
            inclusive: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of your tax rates.
/// Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTaxRate<'a> {
    inner: ListTaxRateBuilder<'a>,
}
impl<'a> ListTaxRate<'a> {
    /// Construct a new `ListTaxRate`.
    pub fn new() -> Self {
        Self { inner: ListTaxRateBuilder::new() }
    }
    /// Optional flag to filter by tax rates that are either active or inactive (archived).
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Optional range for filtering created date.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Optional flag to filter by tax rates that are inclusive (or those that are not inclusive).
    pub fn inclusive(mut self, inclusive: bool) -> Self {
        self.inner.inclusive = Some(inclusive);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListTaxRate<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTaxRate<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::TaxRate>> {
        stripe_client_core::ListPaginator::new_list("/tax_rates", self.inner)
    }
}

impl StripeRequest for ListTaxRate<'_> {
    type Output = stripe_types::List<stripe_shared::TaxRate>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax_rates").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTaxRateBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTaxRateBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a tax rate with the given ID
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTaxRate<'a> {
    inner: RetrieveTaxRateBuilder<'a>,
    tax_rate: &'a stripe_shared::TaxRateId,
}
impl<'a> RetrieveTaxRate<'a> {
    /// Construct a new `RetrieveTaxRate`.
    pub fn new(tax_rate: &'a stripe_shared::TaxRateId) -> Self {
        Self { tax_rate, inner: RetrieveTaxRateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTaxRate<'_> {
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

impl StripeRequest for RetrieveTaxRate<'_> {
    type Output = stripe_shared::TaxRate;

    fn build(&self) -> RequestBuilder {
        let tax_rate = self.tax_rate;
        RequestBuilder::new(StripeMethod::Get, format!("/tax_rates/{tax_rate}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTaxRateBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    display_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    inclusive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    jurisdiction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_type: Option<stripe_shared::TaxRateTaxType>,
}
impl<'a> CreateTaxRateBuilder<'a> {
    fn new(display_name: &'a str, inclusive: bool, percentage: f64) -> Self {
        Self {
            active: None,
            country: None,
            description: None,
            display_name,
            expand: None,
            inclusive,
            jurisdiction: None,
            metadata: None,
            percentage,
            state: None,
            tax_type: None,
        }
    }
}
/// Creates a new tax rate.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxRate<'a> {
    inner: CreateTaxRateBuilder<'a>,
}
impl<'a> CreateTaxRate<'a> {
    /// Construct a new `CreateTaxRate`.
    pub fn new(display_name: &'a str, inclusive: bool, percentage: f64) -> Self {
        Self { inner: CreateTaxRateBuilder::new(display_name, inclusive, percentage) }
    }
    /// Flag determining whether the tax rate is active or inactive (archived).
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub fn country(mut self, country: &'a str) -> Self {
        self.inner.country = Some(country);
        self
    }
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub fn jurisdiction(mut self, jurisdiction: &'a str) -> Self {
        self.inner.jurisdiction = Some(jurisdiction);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    /// For example, "NY" for New York, United States.
    pub fn state(mut self, state: &'a str) -> Self {
        self.inner.state = Some(state);
        self
    }
    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub fn tax_type(mut self, tax_type: stripe_shared::TaxRateTaxType) -> Self {
        self.inner.tax_type = Some(tax_type);
        self
    }
}
impl CreateTaxRate<'_> {
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

impl StripeRequest for CreateTaxRate<'_> {
    type Output = stripe_shared::TaxRate;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax_rates").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateTaxRateBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jurisdiction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_type: Option<stripe_shared::TaxRateTaxType>,
}
impl<'a> UpdateTaxRateBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            country: None,
            description: None,
            display_name: None,
            expand: None,
            jurisdiction: None,
            metadata: None,
            state: None,
            tax_type: None,
        }
    }
}
/// Updates an existing tax rate.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTaxRate<'a> {
    inner: UpdateTaxRateBuilder<'a>,
    tax_rate: &'a stripe_shared::TaxRateId,
}
impl<'a> UpdateTaxRate<'a> {
    /// Construct a new `UpdateTaxRate`.
    pub fn new(tax_rate: &'a stripe_shared::TaxRateId) -> Self {
        Self { tax_rate, inner: UpdateTaxRateBuilder::new() }
    }
    /// Flag determining whether the tax rate is active or inactive (archived).
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub fn country(mut self, country: &'a str) -> Self {
        self.inner.country = Some(country);
        self
    }
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// The display name of the tax rate, which will be shown to users.
    pub fn display_name(mut self, display_name: &'a str) -> Self {
        self.inner.display_name = Some(display_name);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub fn jurisdiction(mut self, jurisdiction: &'a str) -> Self {
        self.inner.jurisdiction = Some(jurisdiction);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    /// For example, "NY" for New York, United States.
    pub fn state(mut self, state: &'a str) -> Self {
        self.inner.state = Some(state);
        self
    }
    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub fn tax_type(mut self, tax_type: stripe_shared::TaxRateTaxType) -> Self {
        self.inner.tax_type = Some(tax_type);
        self
    }
}
impl UpdateTaxRate<'_> {
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

impl StripeRequest for UpdateTaxRate<'_> {
    type Output = stripe_shared::TaxRate;

    fn build(&self) -> RequestBuilder {
        let tax_rate = self.tax_rate;
        RequestBuilder::new(StripeMethod::Post, format!("/tax_rates/{tax_rate}")).form(&self.inner)
    }
}
