// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::TaxRateId;
use crate::params::{Expand, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxRate".
///
/// For more details see <https://stripe.com/docs/api/tax_rates/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxRate {
    /// Unique identifier for the object.
    pub id: TaxRateId,

    /// Defaults to `true`.
    ///
    /// When set to `false`, this tax rate cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub active: bool,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
    /// It will not be visible to your customers.
    pub description: Option<String>,

    /// The display name of the tax rates as it will appear to your customer on their receipt email, PDF, and the hosted invoice page.
    pub display_name: String,

    /// Actual/effective tax rate percentage out of 100.
    ///
    /// For tax calculations with automatic_tax[enabled]=true, this percentage reflects the rate actually used to calculate tax based on the product's taxability and whether the user is registered to collect taxes in the corresponding jurisdiction.
    pub effective_percentage: Option<f64>,

    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,

    /// The jurisdiction for the tax rate.
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub jurisdiction: Option<String>,

    /// The level of the jurisdiction that imposes this tax rate.
    ///
    /// Will be `null` for manually defined tax rates.
    pub jurisdiction_level: Option<TaxRateJurisdictionLevel>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// Tax rate percentage out of 100.
    ///
    /// For tax calculations with automatic_tax[enabled]=true, this percentage includes the statutory tax rate of non-taxable jurisdictions.
    pub percentage: f64,

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,

    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<TaxRateTaxType>,
}

impl TaxRate {
    /// Returns a list of your tax rates.
    ///
    /// Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.
    pub fn list(client: &Client, params: &ListTaxRates<'_>) -> Response<List<TaxRate>> {
        client.get_query("/tax_rates", params)
    }

    /// Creates a new tax rate.
    pub fn create(client: &Client, params: CreateTaxRate<'_>) -> Response<TaxRate> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/tax_rates", &params)
    }

    /// Retrieves a tax rate with the given ID.
    pub fn retrieve(client: &Client, id: &TaxRateId, expand: &[&str]) -> Response<TaxRate> {
        client.get_query(&format!("/tax_rates/{}", id), Expand { expand })
    }

    /// Updates an existing tax rate.
    pub fn update(client: &Client, id: &TaxRateId, params: UpdateTaxRate<'_>) -> Response<TaxRate> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/tax_rates/{}", id), &params)
    }
}

impl Object for TaxRate {
    type Id = TaxRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_rate"
    }
}

/// The parameters for `TaxRate::create`.
#[derive(Clone, Debug, Serialize)]
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
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

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
    pub metadata: Option<Metadata>,

    /// This represents the tax rate percent out of 100.
    pub percentage: f64,

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,

    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<TaxRateTaxType>,
}

impl<'a> CreateTaxRate<'a> {
    pub fn new(display_name: &'a str, percentage: f64) -> Self {
        CreateTaxRate {
            active: Default::default(),
            country: Default::default(),
            description: Default::default(),
            display_name,
            expand: Default::default(),
            inclusive: Default::default(),
            jurisdiction: Default::default(),
            metadata: Default::default(),
            percentage,
            state: Default::default(),
            tax_type: Default::default(),
        }
    }
}

/// The parameters for `TaxRate::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTaxRates<'a> {
    /// Optional flag to filter by tax rates that are either active or inactive (archived).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Optional range for filtering created date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TaxRateId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Optional flag to filter by tax rates that are inclusive (or those that are not inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<TaxRateId>,
}

impl<'a> ListTaxRates<'a> {
    pub fn new() -> Self {
        ListTaxRates {
            active: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            inclusive: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListTaxRates<'_> {
    type O = TaxRate;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `TaxRate::update`.
#[derive(Clone, Debug, Serialize, Default)]
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
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

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
    pub metadata: Option<Metadata>,

    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,

    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<TaxRateTaxType>,
}

impl<'a> UpdateTaxRate<'a> {
    pub fn new() -> Self {
        UpdateTaxRate {
            active: Default::default(),
            country: Default::default(),
            description: Default::default(),
            display_name: Default::default(),
            expand: Default::default(),
            jurisdiction: Default::default(),
            metadata: Default::default(),
            state: Default::default(),
            tax_type: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `TaxRate`'s `jurisdiction_level` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxRateJurisdictionLevel {
    City,
    Country,
    County,
    District,
    Multiple,
    State,
}

impl TaxRateJurisdictionLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxRateJurisdictionLevel::City => "city",
            TaxRateJurisdictionLevel::Country => "country",
            TaxRateJurisdictionLevel::County => "county",
            TaxRateJurisdictionLevel::District => "district",
            TaxRateJurisdictionLevel::Multiple => "multiple",
            TaxRateJurisdictionLevel::State => "state",
        }
    }
}

impl AsRef<str> for TaxRateJurisdictionLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxRateJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxRateJurisdictionLevel {
    fn default() -> Self {
        Self::City
    }
}

/// An enum representing the possible values of an `TaxRate`'s `tax_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxRateTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    ServiceTax,
    Vat,
}

impl TaxRateTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxRateTaxType::AmusementTax => "amusement_tax",
            TaxRateTaxType::CommunicationsTax => "communications_tax",
            TaxRateTaxType::Gst => "gst",
            TaxRateTaxType::Hst => "hst",
            TaxRateTaxType::Igst => "igst",
            TaxRateTaxType::Jct => "jct",
            TaxRateTaxType::LeaseTax => "lease_tax",
            TaxRateTaxType::Pst => "pst",
            TaxRateTaxType::Qst => "qst",
            TaxRateTaxType::Rst => "rst",
            TaxRateTaxType::SalesTax => "sales_tax",
            TaxRateTaxType::ServiceTax => "service_tax",
            TaxRateTaxType::Vat => "vat",
        }
    }
}

impl AsRef<str> for TaxRateTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxRateTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxRateTaxType {
    fn default() -> Self {
        Self::AmusementTax
    }
}
