/// An Add Invoice Item describes the prices and quantities that will be added as pending invoice items when entering a phase.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionScheduleAddInvoiceItem {
    /// ID of the price used to generate the invoice item.
    pub price: stripe_types::Expandable<stripe_types::Price>,
    /// The quantity of the invoice item.
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item.
    ///
    /// When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_types::TaxRate>>,
}
