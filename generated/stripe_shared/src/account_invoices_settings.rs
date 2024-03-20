#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountInvoicesSettings {
    /// The list of default Account Tax IDs to automatically include on invoices.
    /// Account Tax IDs get added when an invoice is finalized.
    pub default_account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
}
