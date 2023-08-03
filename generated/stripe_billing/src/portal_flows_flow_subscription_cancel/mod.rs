#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalFlowsFlowSubscriptionCancel {
    /// The ID of the subscription to be canceled.
    pub subscription: String,
}
