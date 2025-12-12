use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListPriceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring: Option<ListPriceRecurring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::PriceType>,
}
impl ListPriceBuilder {
    fn new() -> Self {
        Self {
            active: None,
            created: None,
            currency: None,
            ending_before: None,
            expand: None,
            limit: None,
            lookup_keys: None,
            product: None,
            recurring: None,
            starting_after: None,
            type_: None,
        }
    }
}
/// Only return prices with these recurring fields.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPriceRecurring {
    /// Filter by billing frequency. Either `day`, `week`, `month` or `year`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<ListPriceRecurringInterval>,
    /// Filter by the price's meter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter: Option<String>,
    /// Filter by the usage type for this price. Can be either `metered` or `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<ListPriceRecurringUsageType>,
}
impl ListPriceRecurring {
    pub fn new() -> Self {
        Self { interval: None, meter: None, usage_type: None }
    }
}
impl Default for ListPriceRecurring {
    fn default() -> Self {
        Self::new()
    }
}
/// Filter by billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListPriceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListPriceRecurringInterval {
    pub fn as_str(&self) -> &str {
        use ListPriceRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListPriceRecurringInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPriceRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ListPriceRecurringInterval");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListPriceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListPriceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListPriceRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListPriceRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Filter by the usage type for this price. Can be either `metered` or `licensed`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListPriceRecurringUsageType {
    Licensed,
    Metered,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListPriceRecurringUsageType {
    pub fn as_str(&self) -> &str {
        use ListPriceRecurringUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListPriceRecurringUsageType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPriceRecurringUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ListPriceRecurringUsageType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListPriceRecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListPriceRecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListPriceRecurringUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListPriceRecurringUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Returns a list of your active prices, excluding [inline prices](https://stripe.com/docs/products-prices/pricing-models#inline-pricing).
/// For the list of inactive prices, set `active` to false.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPrice {
    inner: ListPriceBuilder,
}
impl ListPrice {
    /// Construct a new `ListPrice`.
    pub fn new() -> Self {
        Self { inner: ListPriceBuilder::new() }
    }
    /// Only return prices that are active or inactive (e.g., pass `false` to list all inactive prices).
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return prices for the given currency.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
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
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Only return the price with these lookup_keys, if any exist. You can specify up to 10 lookup_keys.
    pub fn lookup_keys(mut self, lookup_keys: impl Into<Vec<String>>) -> Self {
        self.inner.lookup_keys = Some(lookup_keys.into());
        self
    }
    /// Only return prices for the given product.
    pub fn product(mut self, product: impl Into<String>) -> Self {
        self.inner.product = Some(product.into());
        self
    }
    /// Only return prices with these recurring fields.
    pub fn recurring(mut self, recurring: impl Into<ListPriceRecurring>) -> Self {
        self.inner.recurring = Some(recurring.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return prices of type `recurring` or `one_time`.
    pub fn type_(mut self, type_: impl Into<stripe_shared::PriceType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl Default for ListPrice {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPrice {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Price>> {
        stripe_client_core::ListPaginator::new_list("/prices", &self.inner)
    }
}

impl StripeRequest for ListPrice {
    type Output = stripe_types::List<stripe_shared::Price>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/prices").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrievePriceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePriceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the price with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePrice {
    inner: RetrievePriceBuilder,
    price: stripe_shared::PriceId,
}
impl RetrievePrice {
    /// Construct a new `RetrievePrice`.
    pub fn new(price: impl Into<stripe_shared::PriceId>) -> Self {
        Self { price: price.into(), inner: RetrievePriceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePrice {
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

impl StripeRequest for RetrievePrice {
    type Output = stripe_shared::Price;

    fn build(&self) -> RequestBuilder {
        let price = &self.price;
        RequestBuilder::new(StripeMethod::Get, format!("/prices/{price}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SearchPriceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    query: String,
}
impl SearchPriceBuilder {
    fn new(query: impl Into<String>) -> Self {
        Self { expand: None, limit: None, page: None, query: query.into() }
    }
}
/// Search for prices you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SearchPrice {
    inner: SearchPriceBuilder,
}
impl SearchPrice {
    /// Construct a new `SearchPrice`.
    pub fn new(query: impl Into<String>) -> Self {
        Self { inner: SearchPriceBuilder::new(query.into()) }
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
impl SearchPrice {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::SearchList<stripe_shared::Price>> {
        stripe_client_core::ListPaginator::new_search_list("/prices/search", &self.inner)
    }
}

impl StripeRequest for SearchPrice {
    type Output = stripe_types::SearchList<stripe_shared::Price>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/prices/search").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreatePriceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_scheme: Option<stripe_shared::PriceBillingScheme>,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_options:
        Option<std::collections::HashMap<stripe_types::Currency, CreatePriceCurrencyOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount: Option<CustomUnitAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data: Option<CreatePriceProductData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring: Option<CreatePriceRecurring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers: Option<Vec<CreatePriceTiers>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers_mode: Option<stripe_shared::PriceTiersMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lookup_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_quantity: Option<CreatePriceTransformQuantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<String>,
}
impl CreatePriceBuilder {
    fn new(currency: impl Into<stripe_types::Currency>) -> Self {
        Self {
            active: None,
            billing_scheme: None,
            currency: currency.into(),
            currency_options: None,
            custom_unit_amount: None,
            expand: None,
            lookup_key: None,
            metadata: None,
            nickname: None,
            product: None,
            product_data: None,
            recurring: None,
            tax_behavior: None,
            tiers: None,
            tiers_mode: None,
            transfer_lookup_key: None,
            transform_quantity: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Prices defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePriceCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CreatePriceCurrencyOptionsTiers>>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreatePriceCurrencyOptions {
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
impl Default for CreatePriceCurrencyOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePriceCurrencyOptionsTiers {
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
    pub up_to: CreatePriceCurrencyOptionsTiersUpTo,
}
impl CreatePriceCurrencyOptionsTiers {
    pub fn new(up_to: impl Into<CreatePriceCurrencyOptionsTiersUpTo>) -> Self {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePriceCurrencyOptionsTiersUpTo {
    Inf,
    #[serde(untagged)]
    I64(i64),
}
/// These fields can be used to create a new product that this price will belong to.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePriceProductData {
    /// Whether the product is currently available for purchase. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The identifier for the product.
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
}
impl CreatePriceProductData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            active: None,
            id: None,
            metadata: None,
            name: name.into(),
            statement_descriptor: None,
            tax_code: None,
            unit_label: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `usage_type`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePriceRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreatePriceRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// The meter tracking the usage of a metered price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter: Option<String>,
    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://docs.stripe.com/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<CreatePriceRecurringUsageType>,
}
impl CreatePriceRecurring {
    pub fn new(interval: impl Into<CreatePriceRecurringInterval>) -> Self {
        Self {
            interval: interval.into(),
            interval_count: None,
            meter: None,
            trial_period_days: None,
            usage_type: None,
        }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePriceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePriceRecurringInterval {
    pub fn as_str(&self) -> &str {
        use CreatePriceRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePriceRecurringInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePriceRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePriceRecurringInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePriceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePriceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePriceRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePriceRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Configures how the quantity per period should be determined.
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePriceRecurringUsageType {
    Licensed,
    Metered,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePriceRecurringUsageType {
    pub fn as_str(&self) -> &str {
        use CreatePriceRecurringUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePriceRecurringUsageType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePriceRecurringUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePriceRecurringUsageType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePriceRecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePriceRecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePriceRecurringUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePriceRecurringUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePriceTiers {
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
    pub up_to: CreatePriceTiersUpTo,
}
impl CreatePriceTiers {
    pub fn new(up_to: impl Into<CreatePriceTiersUpTo>) -> Self {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePriceTiersUpTo {
    Inf,
    #[serde(untagged)]
    I64(i64),
}
/// Apply a transformation to the reported usage or set quantity before computing the billed price.
/// Cannot be combined with `tiers`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePriceTransformQuantity {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: CreatePriceTransformQuantityRound,
}
impl CreatePriceTransformQuantity {
    pub fn new(
        divide_by: impl Into<i64>,
        round: impl Into<CreatePriceTransformQuantityRound>,
    ) -> Self {
        Self { divide_by: divide_by.into(), round: round.into() }
    }
}
/// After division, either round the result `up` or `down`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePriceTransformQuantityRound {
    Down,
    Up,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePriceTransformQuantityRound {
    pub fn as_str(&self) -> &str {
        use CreatePriceTransformQuantityRound::*;
        match self {
            Down => "down",
            Up => "up",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePriceTransformQuantityRound {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePriceTransformQuantityRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePriceTransformQuantityRound"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePriceTransformQuantityRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePriceTransformQuantityRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePriceTransformQuantityRound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePriceTransformQuantityRound {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates a new [Price](https://docs.stripe.com/api/prices) for an existing [Product](https://docs.stripe.com/api/products).
/// The Price can be recurring or one-time.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePrice {
    inner: CreatePriceBuilder,
}
impl CreatePrice {
    /// Construct a new `CreatePrice`.
    pub fn new(currency: impl Into<stripe_types::Currency>) -> Self {
        Self { inner: CreatePriceBuilder::new(currency.into()) }
    }
    /// Whether the price can be used for new purchases. Defaults to `true`.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Describes how to compute the price per period.
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    pub fn billing_scheme(
        mut self,
        billing_scheme: impl Into<stripe_shared::PriceBillingScheme>,
    ) -> Self {
        self.inner.billing_scheme = Some(billing_scheme.into());
        self
    }
    /// Prices defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency_options(
        mut self,
        currency_options: impl Into<
            std::collections::HashMap<stripe_types::Currency, CreatePriceCurrencyOptions>,
        >,
    ) -> Self {
        self.inner.currency_options = Some(currency_options.into());
        self
    }
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    pub fn custom_unit_amount(mut self, custom_unit_amount: impl Into<CustomUnitAmount>) -> Self {
        self.inner.custom_unit_amount = Some(custom_unit_amount.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A lookup key used to retrieve prices dynamically from a static string.
    /// This may be up to 200 characters.
    pub fn lookup_key(mut self, lookup_key: impl Into<String>) -> Self {
        self.inner.lookup_key = Some(lookup_key.into());
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
    /// A brief description of the price, hidden from customers.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.inner.nickname = Some(nickname.into());
        self
    }
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    pub fn product(mut self, product: impl Into<String>) -> Self {
        self.inner.product = Some(product.into());
        self
    }
    /// These fields can be used to create a new product that this price will belong to.
    pub fn product_data(mut self, product_data: impl Into<CreatePriceProductData>) -> Self {
        self.inner.product_data = Some(product_data.into());
        self
    }
    /// The recurring components of a price such as `interval` and `usage_type`.
    pub fn recurring(mut self, recurring: impl Into<CreatePriceRecurring>) -> Self {
        self.inner.recurring = Some(recurring.into());
        self
    }
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub fn tax_behavior(
        mut self,
        tax_behavior: impl Into<stripe_shared::PriceTaxBehavior>,
    ) -> Self {
        self.inner.tax_behavior = Some(tax_behavior.into());
        self
    }
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    pub fn tiers(mut self, tiers: impl Into<Vec<CreatePriceTiers>>) -> Self {
        self.inner.tiers = Some(tiers.into());
        self
    }
    /// Defines if the tiering price should be `graduated` or `volume` based.
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    pub fn tiers_mode(mut self, tiers_mode: impl Into<stripe_shared::PriceTiersMode>) -> Self {
        self.inner.tiers_mode = Some(tiers_mode.into());
        self
    }
    /// If set to true, will atomically remove the lookup key from the existing price, and assign it to this price.
    pub fn transfer_lookup_key(mut self, transfer_lookup_key: impl Into<bool>) -> Self {
        self.inner.transfer_lookup_key = Some(transfer_lookup_key.into());
        self
    }
    /// Apply a transformation to the reported usage or set quantity before computing the billed price.
    /// Cannot be combined with `tiers`.
    pub fn transform_quantity(
        mut self,
        transform_quantity: impl Into<CreatePriceTransformQuantity>,
    ) -> Self {
        self.inner.transform_quantity = Some(transform_quantity.into());
        self
    }
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    /// One of `unit_amount`, `unit_amount_decimal`, or `custom_unit_amount` is required, unless `billing_scheme=tiered`.
    pub fn unit_amount(mut self, unit_amount: impl Into<i64>) -> Self {
        self.inner.unit_amount = Some(unit_amount.into());
        self
    }
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    pub fn unit_amount_decimal(mut self, unit_amount_decimal: impl Into<String>) -> Self {
        self.inner.unit_amount_decimal = Some(unit_amount_decimal.into());
        self
    }
}
impl CreatePrice {
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

impl StripeRequest for CreatePrice {
    type Output = stripe_shared::Price;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/prices").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdatePriceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_options:
        Option<std::collections::HashMap<stripe_types::Currency, UpdatePriceCurrencyOptions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lookup_key: Option<bool>,
}
impl UpdatePriceBuilder {
    fn new() -> Self {
        Self {
            active: None,
            currency_options: None,
            expand: None,
            lookup_key: None,
            metadata: None,
            nickname: None,
            tax_behavior: None,
            transfer_lookup_key: None,
        }
    }
}
/// Prices defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePriceCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<UpdatePriceCurrencyOptionsTiers>>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpdatePriceCurrencyOptions {
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
impl Default for UpdatePriceCurrencyOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePriceCurrencyOptionsTiers {
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
    pub up_to: UpdatePriceCurrencyOptionsTiersUpTo,
}
impl UpdatePriceCurrencyOptionsTiers {
    pub fn new(up_to: impl Into<UpdatePriceCurrencyOptionsTiersUpTo>) -> Self {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePriceCurrencyOptionsTiersUpTo {
    Inf,
    #[serde(untagged)]
    I64(i64),
}
/// Updates the specified price by setting the values of the parameters passed.
/// Any parameters not provided are left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePrice {
    inner: UpdatePriceBuilder,
    price: stripe_shared::PriceId,
}
impl UpdatePrice {
    /// Construct a new `UpdatePrice`.
    pub fn new(price: impl Into<stripe_shared::PriceId>) -> Self {
        Self { price: price.into(), inner: UpdatePriceBuilder::new() }
    }
    /// Whether the price can be used for new purchases. Defaults to `true`.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Prices defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency_options(
        mut self,
        currency_options: impl Into<
            std::collections::HashMap<stripe_types::Currency, UpdatePriceCurrencyOptions>,
        >,
    ) -> Self {
        self.inner.currency_options = Some(currency_options.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A lookup key used to retrieve prices dynamically from a static string.
    /// This may be up to 200 characters.
    pub fn lookup_key(mut self, lookup_key: impl Into<String>) -> Self {
        self.inner.lookup_key = Some(lookup_key.into());
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
    /// A brief description of the price, hidden from customers.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.inner.nickname = Some(nickname.into());
        self
    }
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub fn tax_behavior(
        mut self,
        tax_behavior: impl Into<stripe_shared::PriceTaxBehavior>,
    ) -> Self {
        self.inner.tax_behavior = Some(tax_behavior.into());
        self
    }
    /// If set to true, will atomically remove the lookup key from the existing price, and assign it to this price.
    pub fn transfer_lookup_key(mut self, transfer_lookup_key: impl Into<bool>) -> Self {
        self.inner.transfer_lookup_key = Some(transfer_lookup_key.into());
        self
    }
}
impl UpdatePrice {
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

impl StripeRequest for UpdatePrice {
    type Output = stripe_shared::Price;

    fn build(&self) -> RequestBuilder {
        let price = &self.price;
        RequestBuilder::new(StripeMethod::Post, format!("/prices/{price}")).form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
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
