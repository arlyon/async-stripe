#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RefundNextAction {
    /// Contains the refund details.
    pub display_details: Option<stripe_types::RefundNextActionDisplayDetails>,
    /// Type of the next action to perform.
    #[serde(rename = "type")]
    pub type_: String,
}
