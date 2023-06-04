#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CurrencyOption {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: i64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CurrencyOption {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
