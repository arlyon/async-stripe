#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalFlowsFlowSubscriptionCancel {
    /// Specify a retention strategy to be used in the cancellation flow.
    pub retention: Option<stripe_billing::PortalFlowsRetention>,
    /// The ID of the subscription to be canceled.
    pub subscription: String,
}
