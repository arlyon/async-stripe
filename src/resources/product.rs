// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::ProductId;
use crate::params::{Deleted, Expand, List, Metadata, Object, RangeQuery, Timestamp};
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
    pub fn list(client: &Client, params: ListProducts<'_>) -> Response<List<Product>> {
        client.get_query("/products", &params)
    }

    /// Creates a new product object.
    ///
    /// To create a product for use with subscriptions, see [Subscriptions Products](https://stripe.com/docs/api#create_service_product).
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
    /// Any parameters not provided will be left unchanged.  Note that a product’s `attributes` are not editable.
    /// Instead, you would need to deactivate the existing product and create a new one with the new attribute values.
    pub fn update(client: &Client, id: &ProductId, params: UpdateProduct<'_>) -> Response<Product> {
        client.post_form(&format!("/products/{}", id), &params)
    }

    /// Delete a product.
    ///
    /// Deleting a product with type=`good` is only possible if it has no SKUs associated with it.
    /// Deleting a product with type=`service` is only possible if it has no plans associated with it.
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
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<String>>,

    /// A short one-line description of the product, meant to be displayable to the customer.
    ///
    /// May only be set if type=`good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deactivate_on: Option<Vec<String>>,

    /// The product's description, meant to be displayable to the customer.
    ///
    /// May only be set if type=`good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// An identifier will be randomly generated by Stripe.
    ///
    /// You can optionally override this ID, but the ID must be unique across all products in your Stripe account.
    /// Applicable to both `service` and `good` types.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,

    /// A set of key-value pairs that you can attach to a product object.
    ///
    /// It can be useful for storing additional information about the product in a structured format.
    /// Applicable to both `service` and `good` types.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,

    /// The product's name, meant to be displayable to the customer.
    ///
    /// Applicable to both `service` and `good` types.
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_dimensions: Option<PackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods).
    ///
    /// Defaults to `true`.
    /// May only be set if type=`good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,

    /// An arbitrary string to be displayed on your customer's credit card statement.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include <>"' characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    /// It must contain at least one letter.
    /// May only be set if type=`service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,

    /// The type of the product.
    ///
    /// The product is either of type `service`, which is eligible for use with Subscriptions and Plans or `good`, which is eligible for use with Orders and SKUs.
    #[serde(rename = "type")]
    type_: ProductType,

    /// A label that represents units of this product, such as seat(s), in Stripe and on customers’ receipts and invoices.
    ///
    /// Only available on products of type=`service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_label: Option<&'a str>,

    /// A URL of a publicly-accessible webpage for this product.
    ///
    /// May only be set if type=`good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
}

impl<'a> CreateProduct<'a> {
    pub fn new(name: &'a str, type_: ProductType) -> Self {
        CreateProduct {
            active: Default::default(),
            attributes: Default::default(),
            caption: Default::default(),
            deactivate_on: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            id: Default::default(),
            images: Default::default(),
            metadata: Default::default(),
            name,
            package_dimensions: Default::default(),
            shippable: Default::default(),
            statement_descriptor: Default::default(),
            type_,
            unit_label: Default::default(),
            url: Default::default(),
        }
    }
}

/// The parameters for `Product::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListProducts<'a> {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<String>>,

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

    /// Only return products with the given url.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
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
            type_: Default::default(),
            url: Default::default(),
        }
    }
}

/// The parameters for `Product::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateProduct<'a> {
    /// Whether the product is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<String>>,

    /// A short one-line description of the product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deactivate_on: Option<Vec<String>>,

    /// The product's description, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,

    /// A set of key-value pairs that you can attach to a product object.
    ///
    /// It can be useful for storing additional information about the product in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,

    /// The product's name, meant to be displayable to the customer.
    ///
    /// Applicable to both `service` and `good` types.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_dimensions: Option<PackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods).
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    shippable: Option<bool>,

    /// An arbitrary string to be displayed on your customer's credit card statement.
    ///
    /// This may be up to 22 characters.
    /// The statement description may not include <>"' characters, and will appear on your customer's statement in capital letters.
    /// Non-ASCII characters are automatically stripped.
    /// While most banks display this information consistently, some may display it incorrectly or not at all.
    /// It must contain at least one letter.
    /// May only be set if type=`service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,

    /// A label that represents units of this product, such as seat(s), in Stripe and on customers’ receipts and invoices.
    ///
    /// Only available on products of type=`service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_label: Option<&'a str>,

    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
}

impl<'a> UpdateProduct<'a> {
    pub fn new() -> Self {
        UpdateProduct {
            active: Default::default(),
            attributes: Default::default(),
            caption: Default::default(),
            deactivate_on: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            images: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            package_dimensions: Default::default(),
            shippable: Default::default(),
            statement_descriptor: Default::default(),
            unit_label: Default::default(),
            url: Default::default(),
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
