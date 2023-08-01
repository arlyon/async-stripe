#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AcssDebitData {
    /// The statement descriptor associate with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
