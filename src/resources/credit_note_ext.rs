use crate::client::{Client, Response};
use crate::ids::CreditNoteId;
use crate::resources::CreditNote;

impl CreditNote {
    /// Marks a credit note as void.
    ///
    /// You can only void a credit note if the associated invoice is open.
    pub fn void(client: &Client, id: &CreditNoteId) -> Response<CreditNote> {
        client.post(&format!("/credit_notes/{}/void", id))
    }
}
