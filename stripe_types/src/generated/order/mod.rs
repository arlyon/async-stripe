#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Order {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// List of items constituting the order.
    pub items: Option<Vec<stripe_types::order_item::OrderItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types::shipping_details::ShippingDetails>,
}
