#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedTerminalReader {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalReaderId,
}
impl stripe_types::Object for DeletedTerminalReader {
    type Id = stripe_terminal::TerminalReaderId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
