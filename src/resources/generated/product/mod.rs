/// Products describe the specific goods or services you offer to your customers.
/// For example, you might offer a Standard and Premium version of your goods or service; each version would be a separate Product.
/// They can be used in conjunction with [Prices](https://stripe.com/docs/api#prices) to configure pricing in Payment Links, Checkout, and Subscriptions.
///
/// Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription),
/// [share a Payment Link](https://stripe.com/docs/payments/payment-links/overview),
/// [accept payments with Checkout](https://stripe.com/docs/payments/accept-a-payment#create-product-prices-upfront),
/// and more about [Products and Prices](https://stripe.com/docs/products-prices/overview).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Product {
    /// Whether the product is currently available for purchase.
    pub active: bool,
    /// A list of up to 5 attributes that each SKU can provide values for (e.g., `["color", "size"]`).
    pub attributes: Option<Vec<String>>,
    /// A short one-line description of the product, meant to be displayable to the customer.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// An array of connect application identifiers that cannot purchase this product.
    ///
    /// Only applicable to products of `type=good`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_on: Option<Vec<String>>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price: Option<crate::Expandable<crate::price::Price>>,
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: crate::product::ProductId,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub images: Vec<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ProductObject,
    /// The dimensions of this product for shipping purposes.
    pub package_dimensions: Option<crate::package_dimensions::PackageDimensions>,
    /// Whether this product is shipped (i.e., physical goods).
    pub shippable: Option<bool>,
    /// Extra information about a product which will appear on your customer's credit card statement.
    ///
    /// In the case that multiple products are billed at once, the first statement descriptor will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub tax_code: Option<crate::Expandable<crate::tax_code::TaxCode>>,
    /// The type of the product.
    ///
    /// The product is either of type `good`, which is eligible for use with Orders and SKUs, or `service`, which is eligible for use with Subscriptions and Plans.
    #[serde(rename = "type")]
    pub type_: ProductType,
    /// A label that represents units of this product in Stripe and on customers’ receipts and invoices.
    ///
    /// When set, this will be included in associated invoice line item descriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: crate::Timestamp,
    /// A URL of a publicly-accessible webpage for this product.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Product {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ProductObject {
    Product,
}

impl ProductObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Product => "product",
        }
    }
}

impl AsRef<str> for ProductObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ProductObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the product.
///
/// The product is either of type `good`, which is eligible for use with Orders and SKUs, or `service`, which is eligible for use with Subscriptions and Plans.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Good,
    Service,
}

impl ProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Good => "good",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for ProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for Product {
    type Id = crate::product::ProductId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(ProductId);
pub mod deleted;
pub mod requests;
pub use deleted::DeletedProduct;
