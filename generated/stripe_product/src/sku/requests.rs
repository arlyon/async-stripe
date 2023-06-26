impl stripe_product::sku::Sku {
    /// Retrieves the details of an existing SKU.
    ///
    /// Supply the unique SKU identifier from either a SKU creation request or from the product, and Stripe will return the corresponding SKU information.
    pub fn retrieve(
        client: &stripe::Client,
        id: &str,
        params: RetrieveSku,
    ) -> stripe::Response<RetrieveReturned> {
        client.get_query(&format!("/skus/{id}", id = id), params)
    }
    /// Returns a list of your SKUs.
    ///
    /// The SKUs are returned sorted by creation date, with the most recently created SKUs appearing first.
    pub fn list(
        client: &stripe::Client,
        params: ListSku,
    ) -> stripe::Response<stripe_types::List<stripe_product::sku::Sku>> {
        client.get_query("/skus", params)
    }
    /// Updates the specific SKU by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  Note that a SKU’s `attributes` are not editable.
    /// Instead, you would need to deactivate the existing SKU and create a new one with the new attribute values.
    pub fn update(
        client: &stripe::Client,
        id: &str,
        params: UpdateSku,
    ) -> stripe::Response<stripe_product::sku::Sku> {
        client.send_form(&format!("/skus/{id}", id = id), params, http_types::Method::Post)
    }
    /// Creates a new SKU associated with a product.
    pub fn create(
        client: &stripe::Client,
        params: CreateSku,
    ) -> stripe::Response<stripe_product::sku::Sku> {
        client.send_form("/skus", params, http_types::Method::Post)
    }
    /// Delete a SKU.
    ///
    /// Deleting a SKU is only possible until it has been used in an order.
    pub fn delete(
        client: &stripe::Client,
        id: &str,
    ) -> stripe::Response<stripe_product::sku::DeletedSku> {
        client.send(&format!("/skus/{id}", id = id), http_types::Method::Delete)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum RetrieveReturned {
    Sku(stripe_product::sku::Sku),
    DeletedSku(stripe_product::sku::DeletedSku),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for RetrieveReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedSku(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::Sku(StripeDeserialize::deserialize(str)?))
        }
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSku<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSku<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListSku<'a> {
    /// Only return SKUs that are active or inactive (e.g., pass `false` to list all inactive products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Only return SKUs that have the specified key-value pairs in this partially constructed dictionary.
    ///
    /// Can be specified only if `product` is also supplied.
    /// For instance, if the associated product has attributes `["color", "size"]`, passing in `attributes[color]=red` returns all the SKUs for this product that have `color` set to `red`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<&'a std::collections::HashMap<String, String>>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Only return SKUs with the given IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<&'a [&'a str]>,
    /// Only return SKUs that are either in stock or out of stock (e.g., pass `false` to list all SKUs that are out of stock).
    ///
    /// If no value is provided, all SKUs are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID of the product whose SKUs will be retrieved.
    ///
    /// Must be a product with type `good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListSku<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSku<'a> {
    /// Whether this SKU is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// When specified, `attributes` will partially update the existing attributes dictionary on the product, with the postcondition that a value must be present for each attribute key on the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<&'a std::collections::HashMap<String, String>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,
    /// Description of the SKU's inventory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<UpdateSkuInventory>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensionsSpecs>,
    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The ID of the product that this SKU should belong to.
    ///
    /// The product must exist, have the same set of attribute names as the SKU's current product, and be of type `good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
}
impl<'a> UpdateSku<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Description of the SKU's inventory.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSkuInventory {
    /// The count of inventory available.
    ///
    /// Required if `type` is `finite`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Inventory type.
    ///
    /// Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    /// An indicator of the inventory available.
    ///
    /// Possible values are `in_stock`, `limited`, and `out_of_stock`.
    /// Will be present if and only if `type` is `bucket`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}
impl UpdateSkuInventory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSku<'a> {
    /// Whether the SKU is available for purchase.
    ///
    /// Default to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<&'a std::collections::HashMap<String, String>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The identifier for the SKU.
    ///
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,
    /// Description of the SKU's inventory.
    pub inventory: CreateSkuInventory,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensionsSpecs>,
    /// The cost of the item as a nonnegative integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,
    /// The ID of the product this SKU is associated with.
    ///
    /// Must be a product with type `good`.
    pub product: &'a str,
}
impl<'a> CreateSku<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        inventory: CreateSkuInventory,
        price: i64,
        product: &'a str,
    ) -> Self {
        Self {
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
            product,
        }
    }
}
/// Description of the SKU's inventory.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSkuInventory {
    /// The count of inventory available.
    ///
    /// Required if `type` is `finite`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Inventory type.
    ///
    /// Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    #[serde(rename = "type")]
    pub type_: Type,
    /// An indicator of the inventory available.
    ///
    /// Possible values are `in_stock`, `limited`, and `out_of_stock`.
    /// Will be present if and only if `type` is `bucket`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}
impl CreateSkuInventory {
    pub fn new(type_: Type) -> Self {
        Self { quantity: Default::default(), type_, value: Default::default() }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Bucket,
    Finite,
    Infinite,
}

impl Type {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bucket => "bucket",
            Self::Finite => "finite",
            Self::Infinite => "infinite",
        }
    }
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bucket" => Ok(Self::Bucket),
            "finite" => Ok(Self::Finite),
            "infinite" => Ok(Self::Infinite),

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
        self.as_str().fmt(f)
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Value {
    InStock,
    Limited,
    OutOfStock,
}

impl Value {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InStock => "in_stock",
            Self::Limited => "limited",
            Self::OutOfStock => "out_of_stock",
        }
    }
}

impl std::str::FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in_stock" => Ok(Self::InStock),
            "limited" => Ok(Self::Limited),
            "out_of_stock" => Ok(Self::OutOfStock),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for Value {
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
