#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TerminalReaderDeletedReader {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_terminal::terminal_reader_reader::TerminalReaderId,
}
impl stripe_types::Object for TerminalReaderDeletedReader {
    type Id = stripe_terminal::terminal_reader_reader::TerminalReaderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
