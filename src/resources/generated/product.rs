// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{ProductId, TaxCodeId};
use crate::params::{
    Deleted, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{Currency, PackageDimensions, Price, TaxCode, UpTo};

/// The resource representing a Stripe "Product".
///
/// For more details see <https://stripe.com/docs/api/products/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Product {
    /// Unique identifier for the object.
    pub id: ProductId,

    /// Whether the product is currently available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price: Option<Expandable<Price>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The product's name, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    /// Extra information about a product which will appear on your customer's credit card statement.
    ///
    /// In the case that multiple products are billed at once, the first statement descriptor will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<Expandable<TaxCode>>,

    /// A label that represents units of this product in Stripe and on customers’ receipts and invoices.
    ///
    /// When set, this will be included in associated invoice line item descriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,

    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Product {
    /// Returns a list of your products.
    ///
    /// The products are returned sorted by creation date, with the most recently created products appearing first.
    pub fn list(client: &Client, params: &ListProducts<'_>) -> Response<List<Product>> {
        client.get_query("/products", &params)
    }

    /// Creates a new product object.
    pub fn create(client: &Client, params: CreateProduct<'_>) -> Response<Product> {
        client.post_form("/products", &params)
    }

    /// Retrieves the details of an existing product.
    ///
    /// Supply the unique product ID from either a product creation request or the product list, and Stripe will return the corresponding product information.
    pub fn retrieve(client: &Client, id: &ProductId, expand: &[&str]) -> Response<Product> {
        client.get_query(&format!("/products/{}", id), &Expand { expand })
    }

    /// Updates the specific product by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(client: &Client, id: &ProductId, params: UpdateProduct<'_>) -> Response<Product> {
        client.post_form(&format!("/products/{}", id), &params)
    }

    /// Delete a product.
    ///
    /// Deleting a product is only possible if it has no prices associated with it.
    /// Additionally, deleting a product with `type=good` is only possible if it has no SKUs associated with it.
    pub fn delete(client: &Client, id: &ProductId) -> Response<Deleted<ProductId>> {
        client.delete(&format!("/products/{}", id))
    }
}

impl Object for Product {
    type Id = ProductId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "product"
    }
}

/// The parameters for `Product::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateProduct<'a> {
    /// Whether the product is currently available for purchase.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object.
    ///
    /// This Price will be set as the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price_data: Option<CreateProductDefaultPriceData>,

    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// An identifier will be randomly generated by Stripe.
    ///
    /// You can optionally override this ID, but the ID must be unique across all products in your Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,

    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,

    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

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
    pub tax_code: Option<TaxCodeId>,

    /// A label that represents units of this product in Stripe and on customers’ receipts and invoices.
    ///
    /// When set, this will be included in associated invoice line item descriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<&'a str>,

    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}

impl<'a> CreateProduct<'a> {
    pub fn new(name: &'a str) -> Self {
        CreateProduct {
            active: Default::default(),
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
            unit_label: Default::default(),
            url: Default::default(),
        }
    }
}

/// The parameters for `Product::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListProducts<'a> {
    /// Only return products that are active or inactive (e.g., pass `false` to list all inactive products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Only return products that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<ProductId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Only return products with the given IDs.
    ///
    /// Cannot be used with [starting_after](https://stripe.com/docs/api#list_products-starting_after) or [ending_before](https://stripe.com/docs/api#list_products-ending_before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return products that can be shipped (i.e., physical, not digital products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<ProductId>,

    /// Only return products with the given url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}

impl<'a> ListProducts<'a> {
    pub fn new() -> Self {
        ListProducts {
            active: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            ids: Default::default(),
            limit: Default::default(),
            shippable: Default::default(),
            starting_after: Default::default(),
            url: Default::default(),
        }
    }
}
impl Paginable for ListProducts<'_> {
    type O = Product;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `Product::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateProduct<'a> {
    /// Whether the product is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price: Option<&'a str>,

    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The product's name, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

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
    pub tax_code: Option<String>,

    /// A label that represents units of this product in Stripe and on customers’ receipts and invoices.
    ///
    /// When set, this will be included in associated invoice line item descriptions.
    /// May only be set if `type=service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<&'a str>,

    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl<'a> UpdateProduct<'a> {
    pub fn new() -> Self {
        UpdateProduct {
            active: Default::default(),
            default_price: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            images: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            package_dimensions: Default::default(),
            shippable: Default::default(),
            statement_descriptor: Default::default(),
            tax_code: Default::default(),
            unit_label: Default::default(),
            url: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateProductDefaultPriceData {
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CreateProductDefaultPriceDataCurrencyOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateProductDefaultPriceDataRecurring>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateProductDefaultPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_unit_amount: Option<CreateProductDefaultPriceDataCurrencyOptionsCustomUnitAmount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CreateProductDefaultPriceDataCurrencyOptionsTiers>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateProductDefaultPriceDataRecurring {
    pub interval: CreateProductDefaultPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptionsCustomUnitAmount {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateProductDefaultPriceDataCurrencyOptionsTiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,

    pub up_to: Option<UpTo>,
}

/// An enum representing the possible values of an `CreateProductDefaultPriceDataCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateProductDefaultPriceDataCurrencyOptionsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateProductDefaultPriceDataRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateProductDefaultPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateProductDefaultPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateProductDefaultPriceDataRecurringInterval::Day => "day",
            CreateProductDefaultPriceDataRecurringInterval::Month => "month",
            CreateProductDefaultPriceDataRecurringInterval::Week => "week",
            CreateProductDefaultPriceDataRecurringInterval::Year => "year",
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
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateProductDefaultPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `CreateProductDefaultPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateProductDefaultPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateProductDefaultPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateProductDefaultPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateProductDefaultPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateProductDefaultPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateProductDefaultPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateProductDefaultPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateProductDefaultPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
