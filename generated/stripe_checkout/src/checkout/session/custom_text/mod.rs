#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomText {
    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address:
        Option<stripe_checkout::checkout::session::custom_text::position::Position>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<stripe_checkout::checkout::session::custom_text::position::Position>,
}
pub mod position;
pub use position::Position;
