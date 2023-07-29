#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxAmount {
    /// The amount, in %s, of the tax.
    pub amount: i64,
    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,
    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: stripe_types::Expandable<stripe_types::tax_rate::TaxRate>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxAmount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
