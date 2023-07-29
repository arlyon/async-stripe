#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransferData {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: stripe_types::Expandable<stripe_types::account::Account>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransferData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
