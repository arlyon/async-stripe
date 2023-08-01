#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FromInvoice {
    /// The relation between this invoice and the cloned invoice.
    pub action: String,
    /// The invoice that was cloned.
    pub invoice: stripe_types::Expandable<stripe_types::invoice::Invoice>,
}
