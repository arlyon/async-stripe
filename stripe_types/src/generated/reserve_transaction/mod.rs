#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReserveTransaction {
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::reserve_transaction::ReserveTransactionId,
}
impl stripe_types::Object for ReserveTransaction {
    type Id = stripe_types::reserve_transaction::ReserveTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ReserveTransactionId, "rtx_");
