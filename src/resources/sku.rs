// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::SkuId;
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{Currency, PackageDimensions, Product};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SKU".
///
/// For more details see [https://stripe.com/docs/api/skus/object](https://stripe.com/docs/api/skus/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sku {
    /// Unique identifier for the object.
    pub id: SkuId,

    /// Whether the SKU is available for purchase.
    #[serde(default)]
    pub active: bool,

    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Metadata>,

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

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Inventory>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,

    /// The ID of the product this SKU is associated with.
    ///
    /// The product must be currently active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Expandable<Product>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,
}

impl Sku {
    /// Returns a list of your SKUs.
    ///
    /// The SKUs are returned sorted by creation date, with the most recently created SKUs appearing first.
    pub fn list(client: &Client, params: ListSkus<'_>) -> Response<List<Sku>> {
        client.get_query("/skus", &params)
    }

    /// Creates a new SKU associated with a product.
    pub fn create(client: &Client, params: CreateSku<'_>) -> Response<Sku> {
        client.post_form("/skus", &params)
    }

    /// Retrieves the details of an existing SKU.
    ///
    /// Supply the unique SKU identifier from either a SKU creation request or from the product, and Stripe will return the corresponding SKU information.
    pub fn retrieve(client: &Client, id: &SkuId, expand: &[&str]) -> Response<Sku> {
        client.get_query(&format!("/skus/{}", id), &Expand { expand })
    }

    /// Updates the specific SKU by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  Note that a SKU’s `attributes` are not editable.
    /// Instead, you would need to deactivate the existing SKU and create a new one with the new attribute values.
    pub fn update(client: &Client, id: &SkuId, params: UpdateSku<'_>) -> Response<Sku> {
        client.post_form(&format!("/skus/{}", id), &params)
    }

    /// Delete a SKU.
    ///
    /// Deleting a SKU is only possible until it has been used in an order.
    pub fn delete(client: &Client, id: &SkuId) -> Response<Deleted<SkuId>> {
        client.delete(&format!("/skus/{}", id))
    }
}

impl Object for Sku {
    type Id = SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "sku"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inventory {
    /// The count of inventory available.
    ///
    /// Will be present if and only if `type` is `finite`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Inventory type.
    ///
    /// Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    #[serde(rename = "type")]
    pub type_: String,

    /// An indicator of the inventory available.
    ///
    /// Possible values are `in_stock`, `limited`, and `out_of_stock`.
    /// Will be present if and only if `type` is `bucket`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The parameters for `Sku::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateSku<'a> {
    /// Whether the SKU is available for purchase.
    ///
    /// Default to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Metadata>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The identifier for the SKU.
    ///
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,
    pub inventory: Option<Inventory>,

    /// A set of key-value pairs that you can attach to a SKU object.
    ///
    /// It can be useful for storing additional information about the SKU in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// The cost of the item as a nonnegative integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,
}

impl<'a> CreateSku<'a> {
    pub fn new(currency: Currency, inventory: Option<Inventory>, price: i64) -> Self {
        CreateSku {
            active: Default::default(),
            attributes: Default::default(),
            currency,
            expand: Default::default(),
            id: Default::default(),
            image: Default::default(),
            inventory,
            metadata: Default::default(),
            package_dimensions: Default::default(),
            price,
        }
    }
}

/// The parameters for `Sku::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSkus<'a> {
    /// Only return SKUs that are active or inactive (e.g., pass `false` to list all inactive products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Metadata>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a SkuId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,

    /// Only return SKUs that are either in stock or out of stock (e.g., pass `false` to list all SKUs that are out of stock).
    ///
    /// If no value is provided, all SKUs are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,

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
    pub starting_after: Option<SkuId>,
}

impl<'a> ListSkus<'a> {
    pub fn new() -> Self {
        ListSkus {
            active: Default::default(),
            attributes: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            ids: Default::default(),
            in_stock: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `Sku::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateSku<'a> {
    /// Whether this SKU is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Metadata>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Inventory>,

    /// A set of key-value pairs that you can attach to a SKU object.
    ///
    /// It can be useful for storing additional information about the SKU in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

impl<'a> UpdateSku<'a> {
    pub fn new() -> Self {
        UpdateSku {
            active: Default::default(),
            attributes: Default::default(),
            currency: Default::default(),
            expand: Default::default(),
            image: Default::default(),
            inventory: Default::default(),
            metadata: Default::default(),
            package_dimensions: Default::default(),
            price: Default::default(),
        }
    }
}
