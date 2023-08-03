#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,
    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<stripe_types::InvoiceItemThresholdReason>,
}
