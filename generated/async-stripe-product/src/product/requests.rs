use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete a product.
/// Deleting a product is only possible if it has no prices associated with it.
/// Additionally, deleting a product with `type=good` is only possible if it has no SKUs associated with it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteProduct {
    id: stripe_shared::ProductId,
}
impl DeleteProduct {
    /// Construct a new `DeleteProduct`.
    pub fn new(id: impl Into<stripe_shared::ProductId>) -> Self {
        Self { id: id.into() }
    }
}
impl DeleteProduct {
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

impl StripeRequest for DeleteProduct {
    type Output = stripe_shared::DeletedProduct;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Delete, format!("/products/{id}"))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListProductBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}
impl ListProductBuilder {
    fn new() -> Self {
        Self {
            active: None,
            created: None,
            ending_before: None,
            expand: None,
            ids: None,
            limit: None,
            shippable: None,
            starting_after: None,
            type_: None,
            url: None,
        }
    }
}
/// Returns a list of your products.
/// The products are returned sorted by creation date, with the most recently created products appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListProduct {
    inner: ListProductBuilder,
}
impl ListProduct {
    /// Construct a new `ListProduct`.
    pub fn new() -> Self {
        Self { inner: ListProductBuilder::new() }
    }
    /// Only return products that are active or inactive (e.g., pass `false` to list all inactive products).
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Only return products that were created during the given date interval.
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
    /// Only return products with the given IDs.
    /// Cannot be used with [starting_after](https://api.stripe.com#list_products-starting_after) or [ending_before](https://api.stripe.com#list_products-ending_before).
    pub fn ids(mut self, ids: impl Into<Vec<String>>) -> Self {
        self.inner.ids = Some(ids.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Only return products that can be shipped (i.e., physical, not digital products).
    pub fn shippable(mut self, shippable: impl Into<bool>) -> Self {
        self.inner.shippable = Some(shippable.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return products of this type.
    pub fn type_(mut self, type_: impl Into<stripe_shared::ProductType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
    /// Only return products with the given url.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.inner.url = Some(url.into());
        self
    }
}
impl Default for ListProduct {
    fn default() -> Self {
        Self::new()
    }
}
impl ListProduct {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Product>> {
        stripe_client_core::ListPaginator::new_list("/products", &self.inner)
    }
}

impl StripeRequest for ListProduct {
    type Output = stripe_types::List<stripe_shared::Product>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/products").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveProductBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveProductBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing product.
/// Supply the unique product ID from either a product creation request or the product list, and Stripe will return the corresponding product information.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveProduct {
    inner: RetrieveProductBuilder,
    id: stripe_shared::ProductId,
}
impl RetrieveProduct {
    /// Construct a new `RetrieveProduct`.
    pub fn new(id: impl Into<stripe_shared::ProductId>) -> Self {
        Self { id: id.into(), inner: RetrieveProductBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveProduct {
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

impl StripeRequest for RetrieveProduct {
    type Output = stripe_shared::Product;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/products/{id}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct SearchProductBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    query: String,
}
impl SearchProductBuilder {
    fn new(query: impl Into<String>) -> Self {
        Self { expand: None, limit: None, page: None, query: query.into() }
    }
}
/// Search for products you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SearchProduct {
    inner: SearchProductBuilder,
}
impl SearchProduct {
    /// Construct a new `SearchProduct`.
    pub fn new(query: impl Into<String>) -> Self {
        Self { inner: SearchProductBuilder::new(query.into()) }
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
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.inner.page = Some(page.into());
        self
    }
}
impl SearchProduct {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::SearchList<stripe_shared::Product>> {
        stripe_client_core::ListPaginator::new_search_list("/products/search", &self.inner)
    }
}

impl StripeRequest for SearchProduct {
    type Output = stripe_types::SearchList<stripe_shared::Product>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/products/search").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateProductBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_price_data: Option<CreateProductDefaultPriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketing_features: Option<Vec<Features>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_dimensions: Option<PackageDimensionsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}
impl CreateProductBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self {
            active: None,
            default_price_data: None,
            description: None,
            expand: None,
            id: None,
            images: None,
            marketing_features: None,
            metadata: None,
            name: name.into(),
            package_dimensions: None,
            shippable: None,
            statement_descriptor: None,
            tax_code: None,
            type_: None,
            unit_label: None,
            url: None,
        }
    }
}
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object.
/// This Price will be set as the default price for this product.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateProductDefaultPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Prices defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            CreateProductDefaultPriceDataCurrencyOptions,
        >,
    >,
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateProductDefaultPriceDataRecurring>,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateProductDefaultPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    /// One of `unit_amount`, `unit_amount_decimal`, or `custom_unit_amount` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreateProductDefaultPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>) -> Self {
        Self {
            currency: currency.into(),
            currency_options: None,
            custom_unit_amount: None,
            metadata: None,
            recurring: None,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Prices defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CreateProductDefaultPriceDataCurrencyOptionsTiers>>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreateProductDefaultPriceDataCurrencyOptions {
    pub fn new() -> Self {
        Self {
            custom_unit_amount: None,
            tax_behavior: None,
            tiers: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
impl Default for CreateProductDefaultPriceDataCurrencyOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    pub fn as_str(&self) -> &str {
        use CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptionsTiers {
    /// The flat billing amount for an entire tier, regardless of the number of units in the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but accepts a decimal value representing an integer in the minor units of the currency.
    /// Only one of `flat_amount` and `flat_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<String>,
    /// The per unit billing amount for each individual unit for which this tier applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
    /// Specifies the upper bound of this tier.
    /// The lower bound of a tier is the upper bound of the previous tier adding one.
    /// Use `inf` to define a fallback tier.
    pub up_to: CreateProductDefaultPriceDataCurrencyOptionsTiersUpTo,
}
impl CreateProductDefaultPriceDataCurrencyOptionsTiers {
    pub fn new(up_to: impl Into<CreateProductDefaultPriceDataCurrencyOptionsTiersUpTo>) -> Self {
        Self {
            flat_amount: None,
            flat_amount_decimal: None,
            unit_amount: None,
            unit_amount_decimal: None,
            up_to: up_to.into(),
        }
    }
}
/// Specifies the upper bound of this tier.
/// The lower bound of a tier is the upper bound of the previous tier adding one.
/// Use `inf` to define a fallback tier.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateProductDefaultPriceDataCurrencyOptionsTiersUpTo {
    Inf,
    #[serde(untagged)]
    I64(i64),
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateProductDefaultPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreateProductDefaultPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateProductDefaultPriceDataRecurring {
    pub fn new(interval: impl Into<CreateProductDefaultPriceDataRecurringInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateProductDefaultPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateProductDefaultPriceDataRecurringInterval {
    pub fn as_str(&self) -> &str {
        use CreateProductDefaultPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateProductDefaultPriceDataRecurringInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductDefaultPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateProductDefaultPriceDataRecurringInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateProductDefaultPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateProductDefaultPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateProductDefaultPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateProductDefaultPriceDataRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateProductDefaultPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateProductDefaultPriceDataTaxBehavior {
    pub fn as_str(&self) -> &str {
        use CreateProductDefaultPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateProductDefaultPriceDataTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductDefaultPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateProductDefaultPriceDataTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateProductDefaultPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateProductDefaultPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateProductDefaultPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateProductDefaultPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates a new product object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateProduct {
    inner: CreateProductBuilder,
}
impl CreateProduct {
    /// Construct a new `CreateProduct`.
    pub fn new(name: impl Into<String>) -> Self {
        Self { inner: CreateProductBuilder::new(name.into()) }
    }
    /// Whether the product is currently available for purchase. Defaults to `true`.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object.
    /// This Price will be set as the default price for this product.
    pub fn default_price_data(
        mut self,
        default_price_data: impl Into<CreateProductDefaultPriceData>,
    ) -> Self {
        self.inner.default_price_data = Some(default_price_data.into());
        self
    }
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// An identifier will be randomly generated by Stripe.
    /// You can optionally override this ID, but the ID must be unique across all products in your Stripe account.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.inner.id = Some(id.into());
        self
    }
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub fn images(mut self, images: impl Into<Vec<String>>) -> Self {
        self.inner.images = Some(images.into());
        self
    }
    /// A list of up to 15 marketing features for this product.
    /// These are displayed in [pricing tables](https://docs.stripe.com/payments/checkout/pricing-table).
    pub fn marketing_features(mut self, marketing_features: impl Into<Vec<Features>>) -> Self {
        self.inner.marketing_features = Some(marketing_features.into());
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
    /// The dimensions of this product for shipping purposes.
    pub fn package_dimensions(
        mut self,
        package_dimensions: impl Into<PackageDimensionsSpecs>,
    ) -> Self {
        self.inner.package_dimensions = Some(package_dimensions.into());
        self
    }
    /// Whether this product is shipped (i.e., physical goods).
    pub fn shippable(mut self, shippable: impl Into<bool>) -> Self {
        self.inner.shippable = Some(shippable.into());
        self
    }
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    ///  It must contain at least one letter. Only used for subscription payments.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: impl Into<String>) -> Self {
        self.inner.tax_code = Some(tax_code.into());
        self
    }
    /// The type of the product.
    /// Defaults to `service` if not explicitly specified, enabling use of this product with Subscriptions and Plans.
    /// Set this parameter to `good` to use this product with Orders and SKUs.
    /// On API versions before `2018-02-05`, this field defaults to `good` for compatibility reasons.
    pub fn type_(mut self, type_: impl Into<stripe_shared::ProductType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    pub fn unit_label(mut self, unit_label: impl Into<String>) -> Self {
        self.inner.unit_label = Some(unit_label.into());
        self
    }
    /// A URL of a publicly-accessible webpage for this product.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.inner.url = Some(url.into());
        self
    }
}
impl CreateProduct {
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

impl StripeRequest for CreateProduct {
    type Output = stripe_shared::Product;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/products").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateProductBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketing_features: Option<Vec<Features>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_dimensions: Option<PackageDimensionsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}
impl UpdateProductBuilder {
    fn new() -> Self {
        Self {
            active: None,
            default_price: None,
            description: None,
            expand: None,
            images: None,
            marketing_features: None,
            metadata: None,
            name: None,
            package_dimensions: None,
            shippable: None,
            statement_descriptor: None,
            tax_code: None,
            unit_label: None,
            url: None,
        }
    }
}
/// Updates the specific product by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateProduct {
    inner: UpdateProductBuilder,
    id: stripe_shared::ProductId,
}
impl UpdateProduct {
    /// Construct a new `UpdateProduct`.
    pub fn new(id: impl Into<stripe_shared::ProductId>) -> Self {
        Self { id: id.into(), inner: UpdateProductBuilder::new() }
    }
    /// Whether the product is available for purchase.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// The ID of the [Price](https://docs.stripe.com/api/prices) object that is the default price for this product.
    pub fn default_price(mut self, default_price: impl Into<String>) -> Self {
        self.inner.default_price = Some(default_price.into());
        self
    }
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub fn images(mut self, images: impl Into<Vec<String>>) -> Self {
        self.inner.images = Some(images.into());
        self
    }
    /// A list of up to 15 marketing features for this product.
    /// These are displayed in [pricing tables](https://docs.stripe.com/payments/checkout/pricing-table).
    pub fn marketing_features(mut self, marketing_features: impl Into<Vec<Features>>) -> Self {
        self.inner.marketing_features = Some(marketing_features.into());
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
    /// The product's name, meant to be displayable to the customer.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// The dimensions of this product for shipping purposes.
    pub fn package_dimensions(
        mut self,
        package_dimensions: impl Into<PackageDimensionsSpecs>,
    ) -> Self {
        self.inner.package_dimensions = Some(package_dimensions.into());
        self
    }
    /// Whether this product is shipped (i.e., physical goods).
    pub fn shippable(mut self, shippable: impl Into<bool>) -> Self {
        self.inner.shippable = Some(shippable.into());
        self
    }
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    /// It must contain at least one letter.
    /// May only be set if `type=service`.
    /// Only used for subscription payments.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: impl Into<String>) -> Self {
        self.inner.tax_code = Some(tax_code.into());
        self
    }
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    /// May only be set if `type=service`.
    pub fn unit_label(mut self, unit_label: impl Into<String>) -> Self {
        self.inner.unit_label = Some(unit_label.into());
        self
    }
    /// A URL of a publicly-accessible webpage for this product.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.inner.url = Some(url.into());
        self
    }
}
impl UpdateProduct {
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

impl StripeRequest for UpdateProduct {
    type Output = stripe_shared::Product;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/products/{id}")).form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CustomUnitAmount {
    /// Pass in `true` to enable `custom_unit_amount`, otherwise omit `custom_unit_amount`.
    pub enabled: bool,
    /// The maximum unit amount the customer can specify for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum unit amount the customer can specify for this item.
    /// Must be at least the minimum charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    /// The starting unit amount which can be updated by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<i64>,
}
impl CustomUnitAmount {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None, preset: None }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct Features {
    /// The marketing feature name. Up to 80 characters long.
    pub name: String,
}
impl Features {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PackageDimensionsSpecs {
    /// Height, in inches. Maximum precision is 2 decimal places.
    pub height: f64,
    /// Length, in inches. Maximum precision is 2 decimal places.
    pub length: f64,
    /// Weight, in ounces. Maximum precision is 2 decimal places.
    pub weight: f64,
    /// Width, in inches. Maximum precision is 2 decimal places.
    pub width: f64,
}
impl PackageDimensionsSpecs {
    pub fn new(
        height: impl Into<f64>,
        length: impl Into<f64>,
        weight: impl Into<f64>,
        width: impl Into<f64>,
    ) -> Self {
        Self {
            height: height.into(),
            length: length.into(),
            weight: weight.into(),
            width: width.into(),
        }
    }
}
