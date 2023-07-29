#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxAmount {
    /// The amount, in %s, of the tax.
    pub amount: i64,
    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,
    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: stripe_types::Expandable<stripe_types::tax_rate::TaxRate>,
}
