#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedCard {
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::card::CardId,
}
impl stripe_types::Object for DeletedCard {
    type Id = stripe_types::card::CardId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
