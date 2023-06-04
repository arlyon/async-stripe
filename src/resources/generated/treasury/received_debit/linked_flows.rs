#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LinkedFlows {
    /// The DebitReversal created as a result of this ReceivedDebit being reversed.
    pub debit_reversal: Option<String>,
    /// Set if the ReceivedDebit is associated with an InboundTransfer's return of funds.
    pub inbound_transfer: Option<String>,
    /// Set if the ReceivedDebit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    pub issuing_authorization: Option<String>,
    /// Set if the ReceivedDebit is also viewable as an [Issuing Dispute](https://stripe.com/docs/api#issuing_disputes) object.
    pub issuing_transaction: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LinkedFlows {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
