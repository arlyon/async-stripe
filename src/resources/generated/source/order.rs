#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Order {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// List of items constituting the order.
    pub items: Option<Vec<crate::source::order_item::OrderItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<crate::shipping_details::ShippingDetails>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Order {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
