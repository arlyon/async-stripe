/// An Add Invoice Item describes the prices and quantities that will be added as pending invoice items when entering a phase.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AddInvoiceItem {
    /// ID of the price used to generate the invoice item.
    pub price: stripe_types::Expandable<stripe_types::price::Price>,
    /// The quantity of the invoice item.
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item.
    ///
    /// When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_types::tax_rate::TaxRate>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AddInvoiceItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
