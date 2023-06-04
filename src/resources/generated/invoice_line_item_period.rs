#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: crate::Timestamp,
    /// The start of the period.
    pub start: crate::Timestamp,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceLineItemPeriod {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
