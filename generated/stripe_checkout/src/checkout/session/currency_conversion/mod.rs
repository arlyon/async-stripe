#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CurrencyConversion {
    /// Total of all items in source currency before discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total of all items in source currency after discounts and taxes are applied.
    pub amount_total: i64,
    /// Exchange rate used to convert source currency amounts to customer currency amounts.
    pub fx_rate: String,
    /// Creation currency of the CheckoutSession before localization.
    pub source_currency: stripe_types::Currency,
}
