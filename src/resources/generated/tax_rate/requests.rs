use crate::{Client, Response};

impl crate::tax_rate::TaxRate {
    /// Returns a list of your tax rates.
    ///
    /// Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.
    pub fn list(
        client: &Client,
        params: ListTaxRate,
    ) -> Response<crate::List<crate::tax_rate::TaxRate>> {
        client.get_query("/tax_rates", params)
    }
    /// Retrieves a tax rate with the given ID.
    pub fn retrieve(
        client: &Client,
        tax_rate: &crate::tax_rate::TaxRateId,
        params: RetrieveTaxRate,
    ) -> Response<crate::tax_rate::TaxRate> {
        client.get_query(&format!("/tax_rates/{tax_rate}", tax_rate = tax_rate), params)
    }
    /// Creates a new tax rate.
    pub fn create(client: &Client, params: CreateTaxRate) -> Response<crate::tax_rate::TaxRate> {
        client.send_form("/tax_rates", params, http_types::Method::Post)
    }
    /// Updates an existing tax rate.
    pub fn update(
        client: &Client,
        tax_rate: &crate::tax_rate::TaxRateId,
        params: UpdateTaxRate,
    ) -> Response<crate::tax_rate::TaxRate> {
        client.send_form(
            &format!("/tax_rates/{tax_rate}", tax_rate = tax_rate),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListTaxRate<'a> {
    /// Optional flag to filter by tax rates that are either active or inactive (archived).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Optional range for filtering created date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Optional flag to filter by tax rates that are inclusive (or those that are not inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
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
impl<'a> ListTaxRate<'a> {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRate<'a> {
    /// Flag determining whether the tax rate is active or inactive (archived).
    ///
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
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
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// This represents the tax rate percent out of 100.
    pub percentage: f64,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<CreateTaxRateTaxType>,
}
impl<'a> CreateTaxRate<'a> {
    pub fn new(display_name: &'a str, inclusive: bool, percentage: f64) -> Self {
        Self {
            active: Default::default(),
            country: Default::default(),
            description: Default::default(),
            display_name,
            expand: Default::default(),
            inclusive,
            jurisdiction: Default::default(),
            metadata: Default::default(),
            percentage,
            state: Default::default(),
            tax_type: Default::default(),
        }
    }
}
/// The high-level tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateTaxRateTaxType {
    Gst,
    Hst,
    Jct,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

impl CreateTaxRateTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gst => "gst",
            Self::Hst => "hst",
            Self::Jct => "jct",
            Self::Pst => "pst",
            Self::Qst => "qst",
            Self::Rst => "rst",
            Self::SalesTax => "sales_tax",
            Self::Vat => "vat",
        }
    }
}

impl AsRef<str> for CreateTaxRateTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxRateTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTaxRate<'a> {
    /// Flag determining whether the tax rate is active or inactive (archived).
    ///
    /// Inactive tax rates cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
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
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<UpdateTaxRateTaxType>,
}
impl<'a> UpdateTaxRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The high-level tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateTaxRateTaxType {
    Gst,
    Hst,
    Jct,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

impl UpdateTaxRateTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gst => "gst",
            Self::Hst => "hst",
            Self::Jct => "jct",
            Self::Pst => "pst",
            Self::Qst => "qst",
            Self::Rst => "rst",
            Self::SalesTax => "sales_tax",
            Self::Vat => "vat",
        }
    }
}

impl AsRef<str> for UpdateTaxRateTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateTaxRateTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
