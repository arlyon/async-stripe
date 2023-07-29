#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LinkedFlows {
    /// Set if there is an Issuing dispute associated with the DebitReversal.
    pub issuing_dispute: Option<String>,
}
