#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Computed {
    /// The definitive totals and line items the customer will be charged on a recurring basis.
    ///
    /// Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only.
    /// Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Option<stripe_types::quote::recurring::Recurring>,
    pub upfront: stripe_types::quote::upfront::Upfront,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Computed {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
