#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTaxRate<'a> {
    /// Optional flag to filter by tax rates that are either active or inactive (archived).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Optional range for filtering created date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Optional flag to filter by tax rates that are inclusive (or those that are not inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
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
impl<'a> ListTaxRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListTaxRate<'a> {
    /// Returns a list of your tax rates.
    /// Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::TaxRate>> {
        client.get_query("/tax_rates", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_shared::TaxRate>> {
        stripe::ListPaginator::from_list_params("/tax_rates", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTaxRate<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTaxRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTaxRate<'a> {
    /// Retrieves a tax rate with the given ID
    pub fn send(
        &self,
        client: &stripe::Client,
        tax_rate: &stripe_shared::TaxRateId,
    ) -> stripe::Response<stripe_shared::TaxRate> {
        client.get_query(&format!("/tax_rates/{tax_rate}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRate<'a> {
    /// Flag determining whether the tax rate is active or inactive (archived).
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The display name of the tax rate, which will be shown to users.
    pub display_name: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// This represents the tax rate percent out of 100.
    pub percentage: f64,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<stripe_shared::TaxRateTaxType>,
}
impl<'a> CreateTaxRate<'a> {
    pub fn new(display_name: &'a str, inclusive: bool, percentage: f64) -> Self {
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
impl<'a> CreateTaxRate<'a> {
    /// Creates a new tax rate.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_shared::TaxRate> {
        client.send_form("/tax_rates", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTaxRate<'a> {
    /// Flag determining whether the tax rate is active or inactive (archived).
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The display name of the tax rate, which will be shown to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<stripe_shared::TaxRateTaxType>,
}
impl<'a> UpdateTaxRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTaxRate<'a> {
    /// Updates an existing tax rate.
    pub fn send(
        &self,
        client: &stripe::Client,
        tax_rate: &stripe_shared::TaxRateId,
    ) -> stripe::Response<stripe_shared::TaxRate> {
        client.send_form(&format!("/tax_rates/{tax_rate}"), self, http_types::Method::Post)
    }
}