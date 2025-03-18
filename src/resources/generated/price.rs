// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::PriceId;
use crate::params::{
    CurrencyMap, Expand, Expandable, IdOrCreate, List, Metadata, Object, Paginable, RangeQuery,
    Timestamp,
};
use crate::resources::{CreateProduct, Currency, CustomUnitAmount, Product, UpTo};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Price".
///
/// For more details see <https://stripe.com/docs/api/prices/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Price {
    /// Unique identifier for the object.
    pub id: PriceId,

    /// Whether the price can be used for new purchases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Describes how to compute the price per period.
    ///
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<PriceBillingScheme>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Prices defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<CurrencyOption>>,

    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CustomUnitAmount>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// A lookup key used to retrieve prices dynamically from a static string.
    ///
    /// This may be up to 200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A brief description of the price, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The ID of the product this price is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Expandable<Product>>,

    /// The recurring components of a price such as `interval` and `usage_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PriceTaxBehavior>,

    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PriceTier>>,

    /// Defines if the tiering price should be `graduated` or `volume` based.
    ///
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price.
    /// In `graduated` tiering, pricing can change as the quantity grows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<PriceTiersMode>,

    /// Apply a transformation to the reported usage or set quantity before computing the amount billed.
    ///
    /// Cannot be combined with `tiers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<TransformQuantity>,

    /// One of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PriceType>,

    /// The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible.
    ///
    /// Only set if `billing_scheme=per_unit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal places.
    ///
    /// Only set if `billing_scheme=per_unit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

impl Price {
    /// Returns a list of your active prices, excluding [inline prices](https://stripe.com/docs/products-prices/pricing-models#inline-pricing).
    ///
    /// For the list of inactive prices, set `active` to false.
    pub fn list(client: &Client, params: &ListPrices<'_>) -> Response<List<Price>> {
        client.get_query("/prices", params)
    }

    /// Creates a new price for an existing product.
    ///
    /// The price can be recurring or one-time.
    pub fn create(client: &Client, params: CreatePrice<'_>) -> Response<Price> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/prices", &params)
    }

    /// Retrieves the price with the given ID.
    pub fn retrieve(client: &Client, id: &PriceId, expand: &[&str]) -> Response<Price> {
        client.get_query(&format!("/prices/{}", id), Expand { expand })
    }

    /// Updates the specified price by setting the values of the parameters passed.
    ///
    /// Any parameters not provided are left unchanged.
    pub fn update(client: &Client, id: &PriceId, params: UpdatePrice<'_>) -> Response<Price> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/prices/{}", id), &params)
    }
}

impl Object for Price {
    type Id = PriceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "price"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CurrencyOption {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    pub custom_unit_amount: Option<CustomUnitAmount>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub tax_behavior: Option<CurrencyOptionTaxBehavior>,

    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PriceTier>>,

    /// The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible.
    ///
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount: Option<i64>,

    /// The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal places.
    ///
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PriceTier {
    /// Price for the entire tier.
    pub flat_amount: Option<i64>,

    /// Same as `flat_amount`, but contains a decimal value with at most 12 decimal places.
    pub flat_amount_decimal: Option<String>,

    /// Per unit price for units relevant to the tier.
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,

    /// Up to and including to this quantity will be contained in the tier.
    pub up_to: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Recurring {
    /// Specifies a usage aggregation strategy for prices of `usage_type=metered`.
    ///
    /// Defaults to `sum`.
    pub aggregate_usage: Option<RecurringAggregateUsage>,

    /// The frequency at which a subscription is billed.
    ///
    /// One of `day`, `week`, `month` or `year`.
    pub interval: RecurringInterval,

    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,

    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,

    /// Configures how the quantity per period should be determined.
    ///
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: RecurringUsageType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransformQuantity {
    /// Divide usage by this number.
    pub divide_by: i64,

    /// After division, either round the result `up` or `down`.
    pub round: TransformQuantityRound,
}

/// The parameters for `Price::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreatePrice<'a> {
    /// Whether the price can be used for new purchases.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Describes how to compute the price per period.
    ///
    /// Either `per_unit` or `tiered`.
    /// `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`).
    /// `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<PriceBillingScheme>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Prices defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<CreatePriceCurrencyOptions>>,

    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CreatePriceCustomUnitAmount>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A lookup key used to retrieve prices dynamically from a static string.
    ///
    /// This may be up to 200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A brief description of the price, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,

    /// The ID of the product that this price will belong to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<IdOrCreate<'a, CreateProduct<'a>>>,

    /// These fields can be used to create a new product that this price will belong to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreatePriceProductData>,

    /// The recurring components of a price such as `interval` and `usage_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreatePriceRecurring>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PriceTaxBehavior>,

    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CreatePriceTiers>>,

    /// Defines if the tiering price should be `graduated` or `volume` based.
    ///
    /// In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<PriceTiersMode>,

    /// If set to true, will atomically remove the lookup key from the existing price, and assign it to this price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_lookup_key: Option<bool>,

    /// Apply a transformation to the reported usage or set quantity before computing the billed price.
    ///
    /// Cannot be combined with `tiers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<CreatePriceTransformQuantity>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    ///
    /// One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}

impl<'a> CreatePrice<'a> {
    pub fn new(currency: Currency) -> Self {
        CreatePrice {
            active: Default::default(),
            billing_scheme: Default::default(),
            currency,
            currency_options: Default::default(),
            custom_unit_amount: Default::default(),
            expand: Default::default(),
            lookup_key: Default::default(),
            metadata: Default::default(),
            nickname: Default::default(),
            product: Default::default(),
            product_data: Default::default(),
            recurring: Default::default(),
            tax_behavior: Default::default(),
            tiers: Default::default(),
            tiers_mode: Default::default(),
            transfer_lookup_key: Default::default(),
            transform_quantity: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}

/// The parameters for `Price::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPrices<'a> {
    /// Only return prices that are active or inactive (e.g., pass `false` to list all inactive prices).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return prices for the given currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PriceId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return the price with these lookup_keys, if any exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_keys: Option<Vec<String>>,

    /// Only return prices for the given product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<IdOrCreate<'a, CreateProduct<'a>>>,

    /// Only return prices with these recurring fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<ListPricesRecurring>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<PriceId>,

    /// Only return prices of type `recurring` or `one_time`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PriceType>,
}

impl<'a> ListPrices<'a> {
    pub fn new() -> Self {
        ListPrices {
            active: Default::default(),
            created: Default::default(),
            currency: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            lookup_keys: Default::default(),
            product: Default::default(),
            recurring: Default::default(),
            starting_after: Default::default(),
            type_: Default::default(),
        }
    }
}
impl Paginable for ListPrices<'_> {
    type O = Price;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `Price::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePrice<'a> {
    /// Whether the price can be used for new purchases.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Prices defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<UpdatePriceCurrencyOptions>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A lookup key used to retrieve prices dynamically from a static string.
    ///
    /// This may be up to 200 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A brief description of the price, hidden from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PriceTaxBehavior>,

    /// If set to true, will atomically remove the lookup key from the existing price, and assign it to this price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_lookup_key: Option<bool>,
}

impl<'a> UpdatePrice<'a> {
    pub fn new() -> Self {
        UpdatePrice {
            active: Default::default(),
            currency_options: Default::default(),
            expand: Default::default(),
            lookup_key: Default::default(),
            metadata: Default::default(),
            nickname: Default::default(),
            tax_behavior: Default::default(),
            transfer_lookup_key: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CreatePriceCurrencyOptionsCustomUnitAmount>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreatePriceCurrencyOptionsTaxBehavior>,

    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CreatePriceCurrencyOptionsTiers>>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceCustomUnitAmount {
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceProductData {
    /// Whether the product is currently available for purchase.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The identifier for the product.
    ///
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The product's name, meant to be displayable to the customer.
    pub name: String,

    /// An arbitrary string to be displayed on your customer's credit card or bank statement.
    ///
    /// While most banks display this information consistently, some may display it incorrectly or not at all.  This may be up to 22 characters.
    /// The statement description may not include `<`, `>`, `\`, `"`, `'` characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// A label that represents units of this product.
    ///
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceRecurring {
    /// Specifies a usage aggregation strategy for prices of `usage_type=metered`.
    ///
    /// Defaults to `sum`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<CreatePriceRecurringAggregateUsage>,

    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreatePriceRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,

    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,

    /// Configures how the quantity per period should be determined.
    ///
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<CreatePriceRecurringUsageType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceTiers {
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
    pub up_to: Option<UpTo>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceTransformQuantity {
    /// Divide usage by this number.
    pub divide_by: i64,

    /// After division, either round the result `up` or `down`.
    pub round: CreatePriceTransformQuantityRound,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListPricesRecurring {
    /// Filter by billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<ListPricesRecurringInterval>,

    /// Filter by the usage type for this price.
    ///
    /// Can be either `metered` or `licensed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<ListPricesRecurringUsageType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePriceCurrencyOptions {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<UpdatePriceCurrencyOptionsCustomUnitAmount>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdatePriceCurrencyOptionsTaxBehavior>,

    /// Each element represents a pricing tier.
    ///
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<UpdatePriceCurrencyOptionsTiers>>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceCurrencyOptionsCustomUnitAmount {
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePriceCurrencyOptionsTiers {
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
    pub up_to: Option<UpTo>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePriceCurrencyOptionsCustomUnitAmount {
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePriceCurrencyOptionsTiers {
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
    pub up_to: Option<UpTo>,
}

/// An enum representing the possible values of an `CreatePriceCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePriceCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreatePriceCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePriceCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            CreatePriceCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            CreatePriceCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreatePriceCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePriceCurrencyOptionsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePriceCurrencyOptionsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreatePriceRecurring`'s `aggregate_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePriceRecurringAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

impl CreatePriceRecurringAggregateUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePriceRecurringAggregateUsage::LastDuringPeriod => "last_during_period",
            CreatePriceRecurringAggregateUsage::LastEver => "last_ever",
            CreatePriceRecurringAggregateUsage::Max => "max",
            CreatePriceRecurringAggregateUsage::Sum => "sum",
        }
    }
}

impl AsRef<str> for CreatePriceRecurringAggregateUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePriceRecurringAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePriceRecurringAggregateUsage {
    fn default() -> Self {
        Self::LastDuringPeriod
    }
}

/// An enum representing the possible values of an `CreatePriceRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePriceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreatePriceRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePriceRecurringInterval::Day => "day",
            CreatePriceRecurringInterval::Month => "month",
            CreatePriceRecurringInterval::Week => "week",
            CreatePriceRecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for CreatePriceRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePriceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePriceRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `CreatePriceRecurring`'s `usage_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePriceRecurringUsageType {
    Licensed,
    Metered,
}

impl CreatePriceRecurringUsageType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePriceRecurringUsageType::Licensed => "licensed",
            CreatePriceRecurringUsageType::Metered => "metered",
        }
    }
}

impl AsRef<str> for CreatePriceRecurringUsageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePriceRecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePriceRecurringUsageType {
    fn default() -> Self {
        Self::Licensed
    }
}

/// An enum representing the possible values of an `CreatePriceTransformQuantity`'s `round` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePriceTransformQuantityRound {
    Down,
    Up,
}

impl CreatePriceTransformQuantityRound {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePriceTransformQuantityRound::Down => "down",
            CreatePriceTransformQuantityRound::Up => "up",
        }
    }
}

impl AsRef<str> for CreatePriceTransformQuantityRound {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePriceTransformQuantityRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePriceTransformQuantityRound {
    fn default() -> Self {
        Self::Down
    }
}

/// An enum representing the possible values of an `CurrencyOption`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CurrencyOptionTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CurrencyOptionTaxBehavior::Exclusive => "exclusive",
            CurrencyOptionTaxBehavior::Inclusive => "inclusive",
            CurrencyOptionTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CurrencyOptionTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CurrencyOptionTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `ListPricesRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListPricesRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl ListPricesRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            ListPricesRecurringInterval::Day => "day",
            ListPricesRecurringInterval::Month => "month",
            ListPricesRecurringInterval::Week => "week",
            ListPricesRecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for ListPricesRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListPricesRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ListPricesRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `ListPricesRecurring`'s `usage_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListPricesRecurringUsageType {
    Licensed,
    Metered,
}

impl ListPricesRecurringUsageType {
    pub fn as_str(self) -> &'static str {
        match self {
            ListPricesRecurringUsageType::Licensed => "licensed",
            ListPricesRecurringUsageType::Metered => "metered",
        }
    }
}

impl AsRef<str> for ListPricesRecurringUsageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListPricesRecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ListPricesRecurringUsageType {
    fn default() -> Self {
        Self::Licensed
    }
}

/// An enum representing the possible values of an `Price`'s `billing_scheme` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PriceBillingScheme {
    PerUnit,
    Tiered,
}

impl PriceBillingScheme {
    pub fn as_str(self) -> &'static str {
        match self {
            PriceBillingScheme::PerUnit => "per_unit",
            PriceBillingScheme::Tiered => "tiered",
        }
    }
}

impl AsRef<str> for PriceBillingScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PriceBillingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PriceBillingScheme {
    fn default() -> Self {
        Self::PerUnit
    }
}

/// An enum representing the possible values of an `Price`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PriceTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PriceTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            PriceTaxBehavior::Exclusive => "exclusive",
            PriceTaxBehavior::Inclusive => "inclusive",
            PriceTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PriceTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PriceTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PriceTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `Price`'s `tiers_mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PriceTiersMode {
    Graduated,
    Volume,
}

impl PriceTiersMode {
    pub fn as_str(self) -> &'static str {
        match self {
            PriceTiersMode::Graduated => "graduated",
            PriceTiersMode::Volume => "volume",
        }
    }
}

impl AsRef<str> for PriceTiersMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PriceTiersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PriceTiersMode {
    fn default() -> Self {
        Self::Graduated
    }
}

/// An enum representing the possible values of an `Price`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PriceType {
    OneTime,
    Recurring,
}

impl PriceType {
    pub fn as_str(self) -> &'static str {
        match self {
            PriceType::OneTime => "one_time",
            PriceType::Recurring => "recurring",
        }
    }
}

impl AsRef<str> for PriceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PriceType {
    fn default() -> Self {
        Self::OneTime
    }
}

/// An enum representing the possible values of an `Recurring`'s `aggregate_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurringAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

impl RecurringAggregateUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            RecurringAggregateUsage::LastDuringPeriod => "last_during_period",
            RecurringAggregateUsage::LastEver => "last_ever",
            RecurringAggregateUsage::Max => "max",
            RecurringAggregateUsage::Sum => "sum",
        }
    }
}

impl AsRef<str> for RecurringAggregateUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecurringAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RecurringAggregateUsage {
    fn default() -> Self {
        Self::LastDuringPeriod
    }
}

/// An enum representing the possible values of an `Recurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl RecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            RecurringInterval::Day => "day",
            RecurringInterval::Month => "month",
            RecurringInterval::Week => "week",
            RecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for RecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `Recurring`'s `usage_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurringUsageType {
    Licensed,
    Metered,
}

impl RecurringUsageType {
    pub fn as_str(self) -> &'static str {
        match self {
            RecurringUsageType::Licensed => "licensed",
            RecurringUsageType::Metered => "metered",
        }
    }
}

impl AsRef<str> for RecurringUsageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RecurringUsageType {
    fn default() -> Self {
        Self::Licensed
    }
}

/// An enum representing the possible values of an `TransformQuantity`'s `round` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransformQuantityRound {
    Down,
    Up,
}

impl TransformQuantityRound {
    pub fn as_str(self) -> &'static str {
        match self {
            TransformQuantityRound::Down => "down",
            TransformQuantityRound::Up => "up",
        }
    }
}

impl AsRef<str> for TransformQuantityRound {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransformQuantityRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TransformQuantityRound {
    fn default() -> Self {
        Self::Down
    }
}

/// An enum representing the possible values of an `UpdatePriceCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePriceCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdatePriceCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePriceCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            UpdatePriceCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            UpdatePriceCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdatePriceCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePriceCurrencyOptionsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePriceCurrencyOptionsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
