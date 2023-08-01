#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionCancel {
    /// The ID of the subscription to be canceled.
    pub subscription: String,
}
