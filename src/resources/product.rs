// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::ProductId;
use crate::params::{Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::PackageDimensions;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Product".
///
/// For more details see [https://stripe.com/docs/api/products/object](https://stripe.com/docs/api/products/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Product {
    /// Unique identifier for the object.
    pub id: ProductId,

    /// Whether the product is currently available for purchase.
    #[serde(default)]
    pub active: bool,

    /// A list of up to 5 attributes that each SKU can provide values for (e.g., `["color", "size"]`).
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,

    /// A short one-line description of the product, meant to be displayable to the customer.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// An array of connect application identifiers that cannot purchase this product.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_on: Option<Vec<String>>,

    // Always true for a deleted object
    #[serde(default)]
    deleted: bool,

    /// The product's description, meant to be displayable to the customer.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The product's name, meant to be displayable to the customer.
    ///
    /// Applicable to both `service` and `good` types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The dimensions of this product for shipping purposes.
    ///
    /// A SKU associated with this product can override this value by having its own `package_dimensions`.
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// Whether this product is a shipped good.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(default)]
    pub shippable: bool,

    /// Extra information about a product which will appear on your customer's credit card statement.
    ///
    /// In the case that multiple products are billed at once, the first statement descriptor will be used.
    /// Only available on products of type=`service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The type of the product.
    ///
    /// The product is either of type `good`, which is eligible for use with Orders and SKUs, or `service`, which is eligible for use with Subscriptions and Plans.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ProductType>,

    /// A label that represents units of this product, such as seat(s), in Stripe and on customers’ receipts and invoices.
    ///
    /// Only available on products of type=`service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,

    /// A URL of a publicly-accessible webpage for this product.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Product {
    /// Returns a list of your products.
    ///
    /// The products are returned sorted by creation date, with the most recently created products appearing first.
    pub fn list(client: &Client, params: ProductListParams<'_>) -> Response<List<Product>> {
        client.get_query("/products", &params)
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

/// The parameters for `Product::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ProductListParams<'a> {
    /// Only return products that are active or inactive (e.g., pass `false` to list all inactive products).
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// Only return products that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a ProductId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// Only return products that can be shipped (i.e., physical, not digital products).
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a ProductId>,

    /// Only return products of this type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<ProductType>,
}

impl<'a> ProductListParams<'a> {
    pub fn new() -> Self {
        ProductListParams {
            active: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            shippable: Default::default(),
            starting_after: Default::default(),
            type_: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `Product`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Good,
    Service,
}
