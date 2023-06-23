#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ProrationDetails {
    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<stripe_types::credited_items::CreditedItems>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ProrationDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
