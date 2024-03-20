#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedPlan {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::PlanId,
}
impl stripe_types::Object for DeletedPlan {
    type Id = stripe_shared::PlanId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
