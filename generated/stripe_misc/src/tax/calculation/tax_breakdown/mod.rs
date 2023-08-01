#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxBreakdown {
    /// The amount of tax, in integer cents.
    pub amount: i64,
    /// Specifies whether the tax amount is included in the line item amount.
    pub inclusive: bool,
    pub tax_rate_details:
        stripe_misc::tax::calculation::tax_breakdown::tax_rate_details::TaxRateDetails,
    /// The amount on which tax is calculated, in integer cents.
    pub taxable_amount: i64,
}
pub mod tax_rate_details;
pub use tax_rate_details::TaxRateDetails;
