/// Represents a cart to be displayed on the reader.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Cart {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: crate::Currency,
    /// List of line items in the cart.
pub line_items: Vec<crate::terminal::reader::reader_action::set_reader_display_action::cart::line_item::LineItem>,
    /// Tax amount for the entire cart.
    ///
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
pub tax: Option<i64>,
    /// Total amount for the entire cart, including tax.
    ///
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
pub total: i64,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Cart {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod line_item;
pub use line_item::LineItem;
