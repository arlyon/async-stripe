#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CreditedItems {
    /// Invoice containing the credited invoice line items.
    pub invoice: String,
    /// Credited invoice line items.
    pub invoice_line_items: Vec<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditedItems {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
