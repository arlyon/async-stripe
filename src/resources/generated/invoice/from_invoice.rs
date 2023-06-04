#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FromInvoice {
    /// The relation between this invoice and the cloned invoice.
    pub action: String,
    /// The invoice that was cloned.
    pub invoice: crate::Expandable<crate::invoice::Invoice>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FromInvoice {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
