#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CurrencyOption {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: i64,
}
