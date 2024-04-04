/// Represents a cart to be displayed on the reader
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalReaderReaderResourceCart {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// List of line items in the cart.
    pub line_items: Vec<stripe_terminal::TerminalReaderReaderResourceLineItem>,
    /// Tax amount for the entire cart.
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub tax: Option<i64>,
    /// Total amount for the entire cart, including tax.
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub total: i64,
}
