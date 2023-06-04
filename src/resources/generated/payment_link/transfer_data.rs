#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransferData {
    /// The amount in %s that will be transferred to the destination account.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    /// The connected account receiving the transfer.
    pub destination: crate::Expandable<crate::account::Account>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransferData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
