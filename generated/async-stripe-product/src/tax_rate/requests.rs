use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTaxRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListTaxRateBuilder {
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
pub struct ListTaxRate {
    inner: ListTaxRateBuilder,
}
impl ListTaxRate {
    /// Construct a new `ListTaxRate`.
    pub fn new() -> Self {
        Self { inner: ListTaxRateBuilder::new() }
    }
    /// Optional flag to filter by tax rates that are either active or inactive (archived).
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Optional range for filtering created date.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
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
    /// Optional flag to filter by tax rates that are inclusive (or those that are not inclusive).
    pub fn inclusive(mut self, inclusive: impl Into<bool>) -> Self {
        self.inner.inclusive = Some(inclusive.into());
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
impl Default for ListTaxRate {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTaxRate {
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
        stripe_client_core::ListPaginator::new_list("/tax_rates", &self.inner)
    }
}

impl StripeRequest for ListTaxRate {
    type Output = stripe_types::List<stripe_shared::TaxRate>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax_rates").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTaxRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTaxRateBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a tax rate with the given ID
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTaxRate {
    inner: RetrieveTaxRateBuilder,
    tax_rate: stripe_shared::TaxRateId,
}
impl RetrieveTaxRate {
    /// Construct a new `RetrieveTaxRate`.
    pub fn new(tax_rate: impl Into<stripe_shared::TaxRateId>) -> Self {
        Self { tax_rate: tax_rate.into(), inner: RetrieveTaxRateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTaxRate {
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

impl StripeRequest for RetrieveTaxRate {
    type Output = stripe_shared::TaxRate;

    fn build(&self) -> RequestBuilder {
        let tax_rate = &self.tax_rate;
        RequestBuilder::new(StripeMethod::Get, format!("/tax_rates/{tax_rate}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTaxRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    inclusive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    jurisdiction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_type: Option<stripe_shared::TaxRateTaxType>,
}
impl CreateTaxRateBuilder {
    fn new(
        display_name: impl Into<String>,
        inclusive: impl Into<bool>,
        percentage: impl Into<f64>,
    ) -> Self {
        Self {
            active: None,
            country: None,
            description: None,
            display_name: display_name.into(),
            expand: None,
            inclusive: inclusive.into(),
            jurisdiction: None,
            metadata: None,
            percentage: percentage.into(),
            state: None,
            tax_type: None,
        }
    }
}
/// Creates a new tax rate.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxRate {
    inner: CreateTaxRateBuilder,
}
impl CreateTaxRate {
    /// Construct a new `CreateTaxRate`.
    pub fn new(
        display_name: impl Into<String>,
        inclusive: impl Into<bool>,
        percentage: impl Into<f64>,
    ) -> Self {
        Self {
            inner: CreateTaxRateBuilder::new(
                display_name.into(),
                inclusive.into(),
                percentage.into(),
            ),
        }
    }
    /// Flag determining whether the tax rate is active or inactive (archived).
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.inner.country = Some(country.into());
        self
    }
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub fn jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.inner.jurisdiction = Some(jurisdiction.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix.
    /// For example, "NY" for New York, United States.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.inner.state = Some(state.into());
        self
    }
    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub fn tax_type(mut self, tax_type: impl Into<stripe_shared::TaxRateTaxType>) -> Self {
        self.inner.tax_type = Some(tax_type.into());
        self
    }
}
impl CreateTaxRate {
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

impl StripeRequest for CreateTaxRate {
    type Output = stripe_shared::TaxRate;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax_rates").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateTaxRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jurisdiction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_type: Option<stripe_shared::TaxRateTaxType>,
}
impl UpdateTaxRateBuilder {
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
pub struct UpdateTaxRate {
    inner: UpdateTaxRateBuilder,
    tax_rate: stripe_shared::TaxRateId,
}
impl UpdateTaxRate {
    /// Construct a new `UpdateTaxRate`.
    pub fn new(tax_rate: impl Into<stripe_shared::TaxRateId>) -> Self {
        Self { tax_rate: tax_rate.into(), inner: UpdateTaxRateBuilder::new() }
    }
    /// Flag determining whether the tax rate is active or inactive (archived).
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.inner.country = Some(country.into());
        self
    }
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The display name of the tax rate, which will be shown to users.
    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.inner.display_name = Some(display_name.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub fn jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.inner.jurisdiction = Some(jurisdiction.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix.
    /// For example, "NY" for New York, United States.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.inner.state = Some(state.into());
        self
    }
    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub fn tax_type(mut self, tax_type: impl Into<stripe_shared::TaxRateTaxType>) -> Self {
        self.inner.tax_type = Some(tax_type.into());
        self
    }
}
impl UpdateTaxRate {
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

impl StripeRequest for UpdateTaxRate {
    type Output = stripe_shared::TaxRate;

    fn build(&self) -> RequestBuilder {
        let tax_rate = &self.tax_rate;
        RequestBuilder::new(StripeMethod::Post, format!("/tax_rates/{tax_rate}")).form(&self.inner)
    }
}
