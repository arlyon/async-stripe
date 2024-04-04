#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTestHelpersTestClock {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::TestHelpersTestClockId,
}
impl stripe_types::Object for DeletedTestHelpersTestClock {
    type Id = stripe_shared::TestHelpersTestClockId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
