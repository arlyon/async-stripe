/// An Order describes a purchase being made by a customer, including the
/// products & quantities being purchased, the order status, the payment information,
/// and the billing/shipping details.
///
/// Related guide: [Orders overview](https://stripe.com/docs/orders).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
    pub application: Option<crate::Expandable<crate::application::Application>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::order::automatic_tax::AutomaticTax>,
    /// Customer billing details associated with the order.
    pub billing_details: Option<crate::order::billing_details::BillingDetails>,
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
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// The customer which this orders belongs to.
    pub customer: Option<crate::Expandable<crate::customer::Customer>>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// The discounts applied to the order.
    ///
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<crate::Expandable<crate::discount::Discount>>>,
    /// Unique identifier for the object.
    pub id: crate::order::OrderId,
    /// A recent IP address of the purchaser used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    /// There is a maximum of 100 line items.
    #[serde(default)]
    pub line_items: crate::List<Option<crate::line_item::LineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::Metadata>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: OrderObject,
    pub payment: crate::order::payment::Payment,
    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<crate::order::shipping_cost::ShippingCost>,
    /// Customer shipping information associated with the order.
    pub shipping_details: Option<crate::order::shipping_details::ShippingDetails>,
    /// The overall status of the order.
    pub status: OrderStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<crate::order::tax_details::TaxDetails>,
    pub total_details: crate::order::total_details::TotalDetails,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Order {
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
/// The overall status of the order.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for Order {
    type Id = crate::order::OrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(OrderId, "or_");
pub mod automatic_tax;
pub mod requests;
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
