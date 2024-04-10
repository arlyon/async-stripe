#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EphemeralKey {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which the key will expire. Measured in seconds since the Unix epoch.
    pub expires: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::EphemeralKeyId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The key's secret. You can use this value to make authorized requests to the Stripe API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
impl stripe_types::Object for EphemeralKey {
    type Id = stripe_misc::EphemeralKeyId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(EphemeralKeyId);
