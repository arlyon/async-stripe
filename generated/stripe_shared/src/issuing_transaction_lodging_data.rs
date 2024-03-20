#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    pub check_in_at: Option<i64>,
    /// The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}
