#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApplePayDomain {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub domain_name: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::ApplePayDomainId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
impl stripe_types::Object for ApplePayDomain {
    type Id = stripe_misc::ApplePayDomainId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplePayDomainId);
