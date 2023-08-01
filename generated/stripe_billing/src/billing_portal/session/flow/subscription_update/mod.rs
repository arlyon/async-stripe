#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionUpdate {
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
