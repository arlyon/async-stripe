#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPrice<'a> {
    /// Only return prices that are active or inactive (e.g., pass `false` to list all inactive prices).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return prices for the given currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return the price with these lookup_keys, if any exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_keys: Option<&'a [&'a str]>,
    /// Only return prices for the given product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Only return prices with these recurring fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<ListPriceRecurring>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return prices of type `recurring` or `one_time`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_shared::PriceType>,
}
impl<'a> ListPrice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return prices with these recurring fields.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPriceRecurring {
    /// Filter by billing frequency. Either `day`, `week`, `month` or `year`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<ListPriceRecurringInterval>,
    /// Filter by the usage type for this price. Can be either `metered` or `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<ListPriceRecurringUsageType>,
}
impl ListPriceRecurring {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Filter by billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListPriceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl ListPriceRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use ListPriceRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for ListPriceRecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPriceRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ListPriceRecurringInterval"))
    }
}
/// Filter by the usage type for this price. Can be either `metered` or `licensed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListPriceRecurringUsageType {
    Licensed,
    Metered,
}
impl ListPriceRecurringUsageType {
    pub fn as_str(self) -> &'static str {
        use ListPriceRecurringUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
        }
    }
}

impl std::str::FromStr for ListPriceRecurringUsageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPriceRecurringUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            _ => Err(()),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ListPriceRecurringUsageType"))
    }
}
impl<'a> ListPrice<'a> {
    /// Returns a list of your active prices, excluding [inline prices](https://stripe.com/docs/products-prices/pricing-models#inline-pricing).
    /// For the list of inactive prices, set `active` to false.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::Price>> {
        client.get_query("/prices", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_shared::Price>> {
        stripe::ListPaginator::from_list_params("/prices", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePrice<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePrice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePrice<'a> {
    /// Retrieves the price with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        price: &stripe_shared::PriceId,
    ) -> stripe::Response<stripe_shared::Price> {
        client.get_query(&format!("/prices/{price}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchPrice<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<&'a str>,
    /// The search query string.
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for prices](https://stripe.com/docs/search#query-fields-for-prices).
    pub query: &'a str,
}
impl<'a> SearchPrice<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { expand: None, limit: None, page: None, query }
    }
}
impl<'a> SearchPrice<'a> {
    /// Search for prices you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    /// Under normal operating.
    /// conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up.
    /// to an hour behind during outages. Search functionality is not available to merchants in India.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::SearchList<stripe_shared::Price>> {
        client.get_query("/prices/search", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::SearchList<stripe_shared::Price>> {
        stripe::ListPaginator::from_search_params("/prices/search", self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePrice<'a> {
    /// Whether the price can be used for new purchases. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Describes how to compute the price per period.
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<stripe_shared::PriceBillingScheme>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Prices defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<&'a std::collections::HashMap<stripe_types::Currency, CreatePriceCurrencyOptions>>,
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A lookup key used to retrieve prices dynamically from a static string.
    /// This may be up to 200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A brief description of the price, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,
    /// The ID of the product that this price will belong to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// These fields can be used to create a new product that this price will belong to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreatePriceProductData<'a>>,
    /// The recurring components of a price such as `interval` and `usage_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreatePriceRecurring>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<&'a [CreatePriceTiers<'a>]>,
    /// Defines if the tiering price should be `graduated` or `volume` based.
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<stripe_shared::PriceTiersMode>,
    /// If set to true, will atomically remove the lookup key from the existing price, and assign it to this price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_lookup_key: Option<bool>,
    /// Apply a transformation to the reported usage or set quantity before computing the billed price.
    /// Cannot be combined with `tiers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<CreatePriceTransformQuantity>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    /// One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreatePrice<'a> {
    pub fn new(currency: stripe_types::Currency) -> Self {
        Self {
            active: None,
            billing_scheme: None,
            currency,
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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreatePriceCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
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
        Self::default()
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
    pub fn new(up_to: CreatePriceCurrencyOptionsTiersUpTo) -> Self {
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
pub enum CreatePriceCurrencyOptionsTiersUpTo {
    Inf,
    I64(i64),
}
/// These fields can be used to create a new product that this price will belong to.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePriceProductData<'a> {
    /// Whether the product is currently available for purchase. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The identifier for the product.
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<&'a str>,
}
impl<'a> CreatePriceProductData<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            active: None,
            id: None,
            metadata: None,
            name,
            statement_descriptor: None,
            tax_code: None,
            unit_label: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `usage_type`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePriceRecurring {
    /// Specifies a usage aggregation strategy for prices of `usage_type=metered`. Defaults to `sum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<CreatePriceRecurringAggregateUsage>,
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreatePriceRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
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
    pub fn new(interval: CreatePriceRecurringInterval) -> Self {
        Self {
            aggregate_usage: None,
            interval,
            interval_count: None,
            trial_period_days: None,
            usage_type: None,
        }
    }
}
/// Specifies a usage aggregation strategy for prices of `usage_type=metered`. Defaults to `sum`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePriceRecurringAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}
impl CreatePriceRecurringAggregateUsage {
    pub fn as_str(self) -> &'static str {
        use CreatePriceRecurringAggregateUsage::*;
        match self {
            LastDuringPeriod => "last_during_period",
            LastEver => "last_ever",
            Max => "max",
            Sum => "sum",
        }
    }
}

impl std::str::FromStr for CreatePriceRecurringAggregateUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePriceRecurringAggregateUsage::*;
        match s {
            "last_during_period" => Ok(LastDuringPeriod),
            "last_ever" => Ok(LastEver),
            "max" => Ok(Max),
            "sum" => Ok(Sum),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreatePriceRecurringAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePriceRecurringAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePriceRecurringAggregateUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePriceRecurringAggregateUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePriceRecurringAggregateUsage")
        })
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePriceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreatePriceRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreatePriceRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreatePriceRecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePriceRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreatePriceRecurringInterval"))
    }
}
/// Configures how the quantity per period should be determined.
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePriceRecurringUsageType {
    Licensed,
    Metered,
}
impl CreatePriceRecurringUsageType {
    pub fn as_str(self) -> &'static str {
        use CreatePriceRecurringUsageType::*;
        match self {
            Licensed => "licensed",
            Metered => "metered",
        }
    }
}

impl std::str::FromStr for CreatePriceRecurringUsageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePriceRecurringUsageType::*;
        match s {
            "licensed" => Ok(Licensed),
            "metered" => Ok(Metered),
            _ => Err(()),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePriceRecurringUsageType")
        })
    }
}
/// Each element represents a pricing tier.
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePriceTiers<'a> {
    /// The flat billing amount for an entire tier, regardless of the number of units in the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but accepts a decimal value representing an integer in the minor units of the currency.
    /// Only one of `flat_amount` and `flat_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<&'a str>,
    /// The per unit billing amount for each individual unit for which this tier applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
    /// Specifies the upper bound of this tier.
    /// The lower bound of a tier is the upper bound of the previous tier adding one.
    /// Use `inf` to define a fallback tier.
    pub up_to: CreatePriceTiersUpTo,
}
impl<'a> CreatePriceTiers<'a> {
    pub fn new(up_to: CreatePriceTiersUpTo) -> Self {
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
pub enum CreatePriceTiersUpTo {
    Inf,
    I64(i64),
}
/// Apply a transformation to the reported usage or set quantity before computing the billed price.
/// Cannot be combined with `tiers`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePriceTransformQuantity {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: CreatePriceTransformQuantityRound,
}
impl CreatePriceTransformQuantity {
    pub fn new(divide_by: i64, round: CreatePriceTransformQuantityRound) -> Self {
        Self { divide_by, round }
    }
}
/// After division, either round the result `up` or `down`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePriceTransformQuantityRound {
    Down,
    Up,
}
impl CreatePriceTransformQuantityRound {
    pub fn as_str(self) -> &'static str {
        use CreatePriceTransformQuantityRound::*;
        match self {
            Down => "down",
            Up => "up",
        }
    }
}

impl std::str::FromStr for CreatePriceTransformQuantityRound {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePriceTransformQuantityRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(()),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePriceTransformQuantityRound")
        })
    }
}
impl<'a> CreatePrice<'a> {
    /// Creates a new price for an existing product. The price can be recurring or one-time.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_shared::Price> {
        client.send_form("/prices", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePrice<'a> {
    /// Whether the price can be used for new purchases. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Prices defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<&'a std::collections::HashMap<stripe_types::Currency, UpdatePriceCurrencyOptions>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A lookup key used to retrieve prices dynamically from a static string.
    /// This may be up to 200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A brief description of the price, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// If set to true, will atomically remove the lookup key from the existing price, and assign it to this price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_lookup_key: Option<bool>,
}
impl<'a> UpdatePrice<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Prices defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePriceCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
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
        Self::default()
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
    pub fn new(up_to: UpdatePriceCurrencyOptionsTiersUpTo) -> Self {
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
pub enum UpdatePriceCurrencyOptionsTiersUpTo {
    Inf,
    I64(i64),
}
impl<'a> UpdatePrice<'a> {
    /// Updates the specified price by setting the values of the parameters passed.
    /// Any parameters not provided are left unchanged.
    pub fn send(
        &self,
        client: &stripe::Client,
        price: &stripe_shared::PriceId,
    ) -> stripe::Response<stripe_shared::Price> {
        client.send_form(&format!("/prices/{price}"), self, http_types::Method::Post)
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
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: None, minimum: None, preset: None }
    }
}
