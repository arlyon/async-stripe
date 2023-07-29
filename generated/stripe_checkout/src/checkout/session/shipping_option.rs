#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    /// The shipping rate.
    pub shipping_rate: stripe_types::Expandable<stripe_product::shipping_rate::ShippingRate>,
}
