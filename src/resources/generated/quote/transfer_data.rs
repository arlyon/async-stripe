#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransferData {
    /// The amount in %s that will be transferred to the destination account when the invoice is paid.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount will be transferred to the destination.
    pub amount_percent: Option<f64>,
    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: crate::Expandable<crate::account::Account>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransferData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
