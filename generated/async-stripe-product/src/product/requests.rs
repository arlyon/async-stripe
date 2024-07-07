use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Delete a product.
/// Deleting a product is only possible if it has no prices associated with it.
/// Additionally, deleting a product with `type=good` is only possible if it has no SKUs associated with it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteProduct<'a> {
    id: &'a stripe_shared::ProductId,
}
impl<'a> DeleteProduct<'a> {
    /// Construct a new `DeleteProduct`.
    pub fn new(id: &'a stripe_shared::ProductId) -> Self {
        Self { id }
    }
}
impl DeleteProduct<'_> {
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

impl StripeRequest for DeleteProduct<'_> {
    type Output = stripe_shared::DeletedProduct;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Delete, format!("/products/{id}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListProductBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
}
impl<'a> ListProductBuilder<'a> {
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
pub struct ListProduct<'a> {
    inner: ListProductBuilder<'a>,
}
impl<'a> ListProduct<'a> {
    /// Construct a new `ListProduct`.
    pub fn new() -> Self {
        Self { inner: ListProductBuilder::new() }
    }
    /// Only return products that are active or inactive (e.g., pass `false` to list all inactive products).
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Only return products that were created during the given date interval.
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
    /// Only return products with the given IDs.
    /// Cannot be used with [starting_after](https://stripe.com/docs/api#list_products-starting_after) or [ending_before](https://stripe.com/docs/api#list_products-ending_before).
    pub fn ids(mut self, ids: &'a [&'a str]) -> Self {
        self.inner.ids = Some(ids);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Only return products that can be shipped (i.e., physical, not digital products).
    pub fn shippable(mut self, shippable: bool) -> Self {
        self.inner.shippable = Some(shippable);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Only return products of this type.
    pub fn type_(mut self, type_: stripe_shared::ProductType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
    /// Only return products with the given url.
    pub fn url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }
}
impl<'a> Default for ListProduct<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListProduct<'_> {
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
        stripe_client_core::ListPaginator::new_list("/products", self.inner)
    }
}

impl StripeRequest for ListProduct<'_> {
    type Output = stripe_types::List<stripe_shared::Product>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/products").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveProductBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveProductBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing product.
/// Supply the unique product ID from either a product creation request or the product list, and Stripe will return the corresponding product information.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveProduct<'a> {
    inner: RetrieveProductBuilder<'a>,
    id: &'a stripe_shared::ProductId,
}
impl<'a> RetrieveProduct<'a> {
    /// Construct a new `RetrieveProduct`.
    pub fn new(id: &'a stripe_shared::ProductId) -> Self {
        Self { id, inner: RetrieveProductBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveProduct<'_> {
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

impl StripeRequest for RetrieveProduct<'_> {
    type Output = stripe_shared::Product;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/products/{id}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct SearchProductBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<&'a str>,
    query: &'a str,
}
impl<'a> SearchProductBuilder<'a> {
    fn new(query: &'a str) -> Self {
        Self { expand: None, limit: None, page: None, query }
    }
}
/// Search for products you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SearchProduct<'a> {
    inner: SearchProductBuilder<'a>,
}
impl<'a> SearchProduct<'a> {
    /// Construct a new `SearchProduct`.
    pub fn new(query: &'a str) -> Self {
        Self { inner: SearchProductBuilder::new(query) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    pub fn page(mut self, page: &'a str) -> Self {
        self.inner.page = Some(page);
        self
    }
}
impl SearchProduct<'_> {
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
        stripe_client_core::ListPaginator::new_search_list("/products/search", self.inner)
    }
}

impl StripeRequest for SearchProduct<'_> {
    type Output = stripe_types::SearchList<stripe_shared::Product>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/products/search").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateProductBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_price_data: Option<CreateProductDefaultPriceData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketing_features: Option<&'a [Features<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_dimensions: Option<PackageDimensionsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<&'a str>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
}
impl<'a> CreateProductBuilder<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            active: None,
            default_price_data: None,
            description: None,
            expand: None,
            id: None,
            images: None,
            marketing_features: None,
            metadata: None,
            name,
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
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object.
/// This Price will be set as the default price for this product.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateProductDefaultPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Prices defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            CreateProductDefaultPriceDataCurrencyOptions,
        >,
    >,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateProductDefaultPriceDataRecurring>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateProductDefaultPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateProductDefaultPriceData<'a> {
    pub fn new(currency: stripe_types::Currency) -> Self {
        Self {
            currency,
            currency_options: None,
            recurring: None,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Prices defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CreateProductDefaultPriceDataCurrencyOptionsCustomUnitAmount>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
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
/// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptionsCustomUnitAmount {
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
impl CreateProductDefaultPriceDataCurrencyOptionsCustomUnitAmount {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: None, minimum: None, preset: None }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior",
            )
        })
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Clone, Debug, serde::Serialize)]
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
    pub fn new(up_to: CreateProductDefaultPriceDataCurrencyOptionsTiersUpTo) -> Self {
        Self {
            flat_amount: None,
            flat_amount_decimal: None,
            unit_amount: None,
            unit_amount_decimal: None,
            up_to,
        }
    }
}
/// Specifies the upper bound of this tier.
/// The lower bound of a tier is the upper bound of the previous tier adding one.
/// Use `inf` to define a fallback tier.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum CreateProductDefaultPriceDataCurrencyOptionsTiersUpTo {
    Inf,
    I64(i64),
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub fn new(interval: CreateProductDefaultPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateProductDefaultPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreateProductDefaultPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreateProductDefaultPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateProductDefaultPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductDefaultPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateProductDefaultPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateProductDefaultPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateProductDefaultPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateProductDefaultPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateProductDefaultPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductDefaultPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateProductDefaultPriceDataTaxBehavior")
        })
    }
}
/// Creates a new product object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateProduct<'a> {
    inner: CreateProductBuilder<'a>,
}
impl<'a> CreateProduct<'a> {
    /// Construct a new `CreateProduct`.
    pub fn new(name: &'a str) -> Self {
        Self { inner: CreateProductBuilder::new(name) }
    }
    /// Whether the product is currently available for purchase. Defaults to `true`.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object.
    /// This Price will be set as the default price for this product.
    pub fn default_price_data(
        mut self,
        default_price_data: CreateProductDefaultPriceData<'a>,
    ) -> Self {
        self.inner.default_price_data = Some(default_price_data);
        self
    }
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// An identifier will be randomly generated by Stripe.
    /// You can optionally override this ID, but the ID must be unique across all products in your Stripe account.
    pub fn id(mut self, id: &'a str) -> Self {
        self.inner.id = Some(id);
        self
    }
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub fn images(mut self, images: &'a [&'a str]) -> Self {
        self.inner.images = Some(images);
        self
    }
    /// A list of up to 15 marketing features for this product.
    /// These are displayed in [pricing tables](https://stripe.com/docs/payments/checkout/pricing-table).
    pub fn marketing_features(mut self, marketing_features: &'a [Features<'a>]) -> Self {
        self.inner.marketing_features = Some(marketing_features);
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
    /// The dimensions of this product for shipping purposes.
    pub fn package_dimensions(mut self, package_dimensions: PackageDimensionsSpecs) -> Self {
        self.inner.package_dimensions = Some(package_dimensions);
        self
    }
    /// Whether this product is shipped (i.e., physical goods).
    pub fn shippable(mut self, shippable: bool) -> Self {
        self.inner.shippable = Some(shippable);
        self
    }
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    ///  It must contain at least one letter.
    pub fn statement_descriptor(mut self, statement_descriptor: &'a str) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor);
        self
    }
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: &'a str) -> Self {
        self.inner.tax_code = Some(tax_code);
        self
    }
    /// The type of the product.
    /// Defaults to `service` if not explicitly specified, enabling use of this product with Subscriptions and Plans.
    /// Set this parameter to `good` to use this product with Orders and SKUs.
    /// On API versions before `2018-02-05`, this field defaults to `good` for compatibility reasons.
    pub fn type_(mut self, type_: stripe_shared::ProductType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    pub fn unit_label(mut self, unit_label: &'a str) -> Self {
        self.inner.unit_label = Some(unit_label);
        self
    }
    /// A URL of a publicly-accessible webpage for this product.
    pub fn url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }
}
impl CreateProduct<'_> {
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

impl StripeRequest for CreateProduct<'_> {
    type Output = stripe_shared::Product;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/products").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateProductBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketing_features: Option<&'a [Features<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_dimensions: Option<PackageDimensionsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
}
impl<'a> UpdateProductBuilder<'a> {
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
pub struct UpdateProduct<'a> {
    inner: UpdateProductBuilder<'a>,
    id: &'a stripe_shared::ProductId,
}
impl<'a> UpdateProduct<'a> {
    /// Construct a new `UpdateProduct`.
    pub fn new(id: &'a stripe_shared::ProductId) -> Self {
        Self { id, inner: UpdateProductBuilder::new() }
    }
    /// Whether the product is available for purchase.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    pub fn default_price(mut self, default_price: &'a str) -> Self {
        self.inner.default_price = Some(default_price);
        self
    }
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub fn images(mut self, images: &'a [&'a str]) -> Self {
        self.inner.images = Some(images);
        self
    }
    /// A list of up to 15 marketing features for this product.
    /// These are displayed in [pricing tables](https://stripe.com/docs/payments/checkout/pricing-table).
    pub fn marketing_features(mut self, marketing_features: &'a [Features<'a>]) -> Self {
        self.inner.marketing_features = Some(marketing_features);
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
    /// The product's name, meant to be displayable to the customer.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// The dimensions of this product for shipping purposes.
    pub fn package_dimensions(mut self, package_dimensions: PackageDimensionsSpecs) -> Self {
        self.inner.package_dimensions = Some(package_dimensions);
        self
    }
    /// Whether this product is shipped (i.e., physical goods).
    pub fn shippable(mut self, shippable: bool) -> Self {
        self.inner.shippable = Some(shippable);
        self
    }
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    ///  It must contain at least one letter. May only be set if `type=service`.
    pub fn statement_descriptor(mut self, statement_descriptor: &'a str) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor);
        self
    }
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: &'a str) -> Self {
        self.inner.tax_code = Some(tax_code);
        self
    }
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    /// May only be set if `type=service`.
    pub fn unit_label(mut self, unit_label: &'a str) -> Self {
        self.inner.unit_label = Some(unit_label);
        self
    }
    /// A URL of a publicly-accessible webpage for this product.
    pub fn url(mut self, url: &'a str) -> Self {
        self.inner.url = Some(url);
        self
    }
}
impl UpdateProduct<'_> {
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

impl StripeRequest for UpdateProduct<'_> {
    type Output = stripe_shared::Product;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/products/{id}")).form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Features<'a> {
    /// The marketing feature name. Up to 80 characters long.
    pub name: &'a str,
}
impl<'a> Features<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
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
    pub fn new(height: f64, length: f64, weight: f64, width: f64) -> Self {
        Self { height, length, weight, width }
    }
}
