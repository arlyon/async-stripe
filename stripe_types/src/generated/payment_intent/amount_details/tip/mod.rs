#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tip {
    /// Portion of the amount that corresponds to a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
