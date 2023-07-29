#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NextAction {
    /// Contains the refund details.
    pub display_details:
        Option<stripe_types::refund::next_action_display_details::NextActionDisplayDetails>,
    /// Type of the next action to perform.
    #[serde(rename = "type")]
    pub type_: String,
}
