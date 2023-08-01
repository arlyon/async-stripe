#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LinkedFlows {
    /// If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to the ReceivedDebit return.
    pub received_debit: Option<String>,
}
