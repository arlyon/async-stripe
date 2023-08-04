#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedApplePayDomain {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_misc::apple_pay_domain::ApplePayDomainId,
}
impl stripe_types::Object for DeletedApplePayDomain {
    type Id = stripe_misc::apple_pay_domain::ApplePayDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
