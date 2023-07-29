#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tax {
    /// Amount of tax applied for this rate.
    pub amount: i64,
    pub rate: stripe_types::tax_rate::TaxRate,
}
