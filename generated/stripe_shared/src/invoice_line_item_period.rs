#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
