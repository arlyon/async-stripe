#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomText {
    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<stripe_types::position::Position>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<stripe_types::position::Position>,
}
