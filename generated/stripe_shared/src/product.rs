/// Products describe the specific goods or services you offer to your customers.
/// For example, you might offer a Standard and Premium version of your goods or service; each version would be a separate Product.
/// They can be used in conjunction with [Prices](https://stripe.com/docs/api#prices) to configure pricing in Payment Links, Checkout, and Subscriptions.
///
/// Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription),.
/// [share a Payment Link](https://stripe.com/docs/payment-links),
/// [accept payments with Checkout](https://stripe.com/docs/payments/accept-a-payment#create-product-prices-upfront),.
/// and more about [Products and Prices](https://stripe.com/docs/products-prices/overview)
///
/// For more details see <<https://stripe.com/docs/api/products/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Product {
    /// Whether the product is currently available for purchase.
    pub active: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    pub description: Option<String>,
    /// A list of up to 15 features for this product.
    /// These are displayed in [pricing tables](https://stripe.com/docs/payments/checkout/pricing-table).
    pub features: Vec<stripe_shared::ProductFeature>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ProductId,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub images: Vec<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// The dimensions of this product for shipping purposes.
    pub package_dimensions: Option<stripe_shared::PackageDimensions>,
    /// Whether this product is shipped (i.e., physical goods).
    pub shippable: Option<bool>,
    /// Extra information about a product which will appear on your customer's credit card statement.
    /// In the case that multiple products are billed at once, the first statement descriptor will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub tax_code: Option<stripe_types::Expandable<stripe_shared::TaxCode>>,
    /// The type of the product.
    /// The product is either of type `good`, which is eligible for use with Orders and SKUs, or `service`, which is eligible for use with Subscriptions and Plans.
    #[serde(rename = "type")]
    pub type_: stripe_shared::ProductType,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
    /// Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: stripe_types::Timestamp,
    /// A URL of a publicly-accessible webpage for this product.
    pub url: Option<String>,
}
impl stripe_types::Object for Product {
    type Id = stripe_shared::ProductId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ProductId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProductType {
    Good,
    Service,
}
impl ProductType {
    pub fn as_str(self) -> &'static str {
        use ProductType::*;
        match self {
            Good => "good",
            Service => "service",
        }
    }
}

impl std::str::FromStr for ProductType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProductType::*;
        match s {
            "good" => Ok(Good),
            "service" => Ok(Service),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ProductType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ProductType"))
    }
}
