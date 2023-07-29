/// An Order describes a purchase being made by a customer, including the
/// products & quantities being purchased, the order status, the payment information,
/// and the billing/shipping details.
///
/// Related guide: [Orders overview](https://stripe.com/docs/orders).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Order {
    /// Order cost before any discounts or taxes are applied.
    ///
    /// A positive integer representing the subtotal of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    pub amount_subtotal: i64,
    /// Total order cost after discounts and taxes are applied.
    ///
    /// A positive integer representing the cost of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// To submit an order, the total must be either 0 or at least $0.50 USD or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    pub amount_total: i64,
    /// ID of the Connect application that created the Order, if any.
    pub application: Option<stripe_types::Expandable<stripe_types::application::Application>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<stripe_misc::order::automatic_tax::AutomaticTax>,
    /// Customer billing details associated with the order.
    pub billing_details: Option<stripe_misc::order::billing_details::BillingDetails>,
    /// The client secret of this Order.
    ///
    /// Used for client-side retrieval using a publishable key.
    /// The client secret can be used to complete a payment for an Order from your frontend.
    /// It should not be stored, logged, embedded in URLs, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    /// Refer to our docs for [creating and processing an order](https://stripe.com/docs/orders-beta/create-and-process) to learn about how client_secret should be handled.
    pub client_secret: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The customer which this orders belongs to.
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// The discounts applied to the order.
    ///
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_types::discount::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_misc::order::OrderId,
    /// A recent IP address of the purchaser used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    /// There is a maximum of 100 line items.
    #[serde(default)]
    pub line_items: stripe_types::List<stripe_types::line_item::LineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: OrderObject,
    pub payment: stripe_misc::order::payment::Payment,
    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<stripe_misc::order::shipping_cost::ShippingCost>,
    /// Customer shipping information associated with the order.
    pub shipping_details: Option<stripe_misc::order::shipping_details::ShippingDetails>,
    /// The overall status of the order.
    pub status: OrderStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<stripe_misc::order::tax_details::TaxDetails>,
    pub total_details: stripe_misc::order::total_details::TotalDetails,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OrderObject {
    Order,
}

impl OrderObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Order => "order",
        }
    }
}

impl std::str::FromStr for OrderObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "order" => Ok(Self::Order),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for OrderObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for OrderObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrderObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OrderObject"))
    }
}
/// The overall status of the order.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OrderStatus {
    Canceled,
    Complete,
    Open,
    Processing,
    Submitted,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Complete => "complete",
            Self::Open => "open",
            Self::Processing => "processing",
            Self::Submitted => "submitted",
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "complete" => Ok(Self::Complete),
            "open" => Ok(Self::Open),
            "processing" => Ok(Self::Processing),
            "submitted" => Ok(Self::Submitted),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for OrderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OrderStatus"))
    }
}
impl stripe_types::Object for Order {
    type Id = stripe_misc::order::OrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(OrderId, "or_");
pub mod automatic_tax;
pub use automatic_tax::AutomaticTax;
pub mod billing_details;
pub use billing_details::BillingDetails;
pub mod payment;
pub use payment::Payment;
pub mod shipping_cost;
pub use shipping_cost::ShippingCost;
pub mod shipping_details;
pub use shipping_details::ShippingDetails;
pub mod tax_details;
pub use tax_details::TaxDetails;
pub mod total_details;
pub use total_details::TotalDetails;
