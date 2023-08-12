#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTestClock {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::test_clock::TestHelpersTestClockId,
}
impl stripe_types::Object for DeletedTestClock {
    type Id = stripe_types::test_clock::TestHelpersTestClockId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
