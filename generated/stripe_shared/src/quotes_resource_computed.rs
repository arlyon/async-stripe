#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotesResourceComputed {
    /// The definitive totals and line items the customer will be charged on a recurring basis.
    /// Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only.
    /// Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Option<stripe_shared::QuotesResourceRecurring>,
    pub upfront: stripe_shared::QuotesResourceUpfront,
}
