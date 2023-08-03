
/// Search for products you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
///
/// Under normal operating conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
/// Search functionality is not available to merchants in India.
pub fn search(client: &stripe::Client, params: SearchProduct) -> stripe::Response<SearchReturned> {
    client.get_query("/products/search", params)
}
/// Creates a new product object.
pub fn create(client: &stripe::Client, params: CreateProduct) -> stripe::Response<stripe_types::Product> {
    client.send_form("/products", params, http_types::Method::Post)
}
/// Retrieves the details of an existing product.
///
/// Supply the unique product ID from either a product creation request or the product list, and Stripe will return the corresponding product information.
pub fn retrieve(client: &stripe::Client, id: &stripe_types::product::ProductId, params: RetrieveProduct) -> stripe::Response<stripe_types::Product> {
    client.get_query(&format!("/products/{id}", id = id), params)
}
/// Updates the specific product by setting the values of the parameters passed.
///
/// Any parameters not provided will be left unchanged.
pub fn update(client: &stripe::Client, id: &stripe_types::product::ProductId, params: UpdateProduct) -> stripe::Response<stripe_types::Product> {
    client.send_form(&format!("/products/{id}", id = id), params, http_types::Method::Post)
}
/// Returns a list of your products.
///
/// The products are returned sorted by creation date, with the most recently created products appearing first.
pub fn list(client: &stripe::Client, params: ListProduct) -> stripe::Response<stripe_types::List<stripe_types::Product>> {
    client.get_query("/products", params)
}
/// Delete a product.
///
/// Deleting a product is only possible if it has no prices associated with it.
/// Additionally, deleting a product with `type=good` is only possible if it has no SKUs associated with it.
pub fn delete(client: &stripe::Client, id: &stripe_types::product::ProductId) -> stripe::Response<stripe_types::DeletedProduct> {
    client.send(&format!("/products/{id}", id = id), http_types::Method::Delete)
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchReturned {
    pub data: Vec<stripe_types::Product>,
    pub has_more: bool,
    pub next_page: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SearchReturnedObject,
    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    pub url: String,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SearchReturnedObject {
    SearchResult,
}

impl SearchReturnedObject {
    pub fn as_str(self) -> &'static str {
        use SearchReturnedObject::*;
        match self {
            SearchResult => "search_result",
        }
    }
}

impl std::str::FromStr for SearchReturnedObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SearchReturnedObject::*;
        match s {
            "search_result" => Ok(SearchResult),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SearchReturnedObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SearchReturnedObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchReturnedObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SearchReturnedObject"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchProduct<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for pagination across multiple pages of results.
    ///
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<&'a str>,
    /// The search query string.
    ///
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for products](https://stripe.com/docs/search#query-fields-for-products).
    pub query: &'a str,
}
impl<'a> SearchProduct<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { expand: Default::default(), limit: Default::default(), page: Default::default(), query }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateProduct<'a> {
    /// Whether the product is currently available for purchase.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A list of up to 5 alphanumeric attributes.
    ///
    /// Should only be set if type=`good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<&'a [&'a str]>,
    /// A short one-line description of the product, meant to be displayable to the customer.
    ///
    /// May only be set if type=`good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    /// An array of Connect application names or identifiers that should not be able to order the SKUs for this product.
    ///
    /// May only be set if type=`good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_on: Option<&'a [&'a str]>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object.
    ///
    /// This Price will be set as the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price_data: Option<CreateProductDefaultPriceData<'a>>,
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// An identifier will be randomly generated by Stripe.
    ///
    /// You can optionally override this ID, but the ID must be unique across all products in your Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensionsSpecs>,
    /// Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    ///
    /// While most banks display this information consistently, some may display it incorrectly or not at all.  This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.  It must contain at least one letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of the product.
    ///
    /// Defaults to `service` if not explicitly specified, enabling use of this product with Subscriptions and Plans.
    /// Set this parameter to `good` to use this product with Orders and SKUs.
    /// On API versions before `2018-02-05`, this field defaults to `good` for compatibility reasons.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    /// A label that represents units of this product.
    ///
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<&'a str>,
    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> CreateProduct<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            active: Default::default(),
            attributes: Default::default(),
            caption: Default::default(),
            deactivate_on: Default::default(),
            default_price_data: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            id: Default::default(),
            images: Default::default(),
            metadata: Default::default(),
            name,
            package_dimensions: Default::default(),
            shippable: Default::default(),
            statement_descriptor: Default::default(),
            tax_code: Default::default(),
            type_: Default::default(),
            unit_label: Default::default(),
            url: Default::default(),
        }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object.
///
/// This Price will be set as the default price for this product.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateProductDefaultPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Prices defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<&'a std::collections::HashMap<stripe_types::Currency, CreateProductDefaultPriceDataCurrencyOptions>>,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateProductDefaultPriceDataRecurring>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    ///
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateProductDefaultPriceData<'a> {
    pub fn new(currency: stripe_types::Currency) -> Self {
        Self { currency, currency_options: Default::default(), recurring: Default::default(), tax_behavior: Default::default(), unit_amount: Default::default(), unit_amount_decimal: Default::default() }
    }
}
/// Prices defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CreateProductDefaultPriceDataCurrencyOptionsCustomUnitAmount>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CreateProductDefaultPriceDataCurrencyOptionsTiers>>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreateProductDefaultPriceDataCurrencyOptions {
    pub fn new() -> Self {
        Self::default()
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
    ///
    /// Must be at least the minimum charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    /// The starting unit amount which can be updated by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<i64>,
}
impl CreateProductDefaultPriceDataCurrencyOptionsCustomUnitAmount {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: Default::default(), minimum: Default::default(), preset: Default::default() }
    }
}
/// Each element represents a pricing tier.
///
/// This parameter requires `billing_scheme` to be set to `tiered`.
/// See also the documentation for `billing_scheme`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptionsTiers {
    /// The flat billing amount for an entire tier, regardless of the number of units in the tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but accepts a decimal value representing an integer in the minor units of the currency.
    ///
    /// Only one of `flat_amount` and `flat_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<String>,
    /// The per unit billing amount for each individual unit for which this tier applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
    /// Specifies the upper bound of this tier.
    ///
    /// The lower bound of a tier is the upper bound of the previous tier adding one.
    /// Use `inf` to define a fallback tier.
    pub up_to: CreateProductDefaultPriceDataCurrencyOptionsTiersUpTo,
}
impl CreateProductDefaultPriceDataCurrencyOptionsTiers {
    pub fn new(up_to: CreateProductDefaultPriceDataCurrencyOptionsTiersUpTo) -> Self {
        Self { flat_amount: Default::default(), flat_amount_decimal: Default::default(), unit_amount: Default::default(), unit_amount_decimal: Default::default(), up_to }
    }
}
/// Specifies the upper bound of this tier.
///
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
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreateProductDefaultPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateProductDefaultPriceDataRecurring {
    pub fn new(interval: CreateProductDefaultPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductDefaultPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateProductDefaultPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveProduct<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveProduct<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateProduct<'a> {
    /// Whether the product is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A list of up to 5 alphanumeric attributes that each SKU can provide values for (e.g., `["color", "size"]`).
    ///
    /// If a value for `attributes` is specified, the list specified will replace the existing attributes list on this product.
    /// Any attributes not present after the update will be deleted from the SKUs for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<&'a [&'a str]>,
    /// A short one-line description of the product, meant to be displayable to the customer.
    ///
    /// May only be set if `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    /// An array of Connect application names or identifiers that should not be able to order the SKUs for this product.
    ///
    /// May only be set if `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_on: Option<&'a [&'a str]>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price: Option<&'a str>,
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensionsSpecs>,
    /// Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    ///
    /// While most banks display this information consistently, some may display it incorrectly or not at all.  This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.  It must contain at least one letter.
    /// May only be set if `type=service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// A label that represents units of this product.
    ///
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    /// May only be set if `type=service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<&'a str>,
    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> UpdateProduct<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListProduct<'a> {
    /// Only return products that are active or inactive (e.g., pass `false` to list all inactive products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Only return products that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Only return products with the given IDs.
    ///
    /// Cannot be used with [starting_after](https://stripe.com/docs/api#list_products-starting_after) or [ending_before](https://stripe.com/docs/api#list_products-ending_before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return products that can be shipped (i.e., physical, not digital products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return products of this type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    /// Only return products with the given url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> ListProduct<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl TaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for TaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PackageDimensionsSpecs {
    /// Height, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub height: f64,
    /// Length, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub length: f64,
    /// Weight, in ounces.
    ///
    /// Maximum precision is 2 decimal places.
    pub weight: f64,
    /// Width, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub width: f64,
}
impl PackageDimensionsSpecs {
    pub fn new(height: f64, length: f64, weight: f64, width: f64) -> Self {
        Self { height, length, weight, width }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Good,
    Service,
}

impl Type {
    pub fn as_str(self) -> &'static str {
        use Type::*;
        match self {
            Good => "good",
            Service => "service",
        }
    }
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Type::*;
        match s {
            "good" => Ok(Good),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
