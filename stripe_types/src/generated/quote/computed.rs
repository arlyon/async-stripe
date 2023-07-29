#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Computed {
    /// The definitive totals and line items the customer will be charged on a recurring basis.
    ///
    /// Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only.
    /// Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Option<stripe_types::quote::recurring::Recurring>,
    pub upfront: stripe_types::quote::upfront::Upfront,
}
