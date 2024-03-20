#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in cents (or local equivalent).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the account that funds are being collected for.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ConnectCollectionTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
impl stripe_types::Object for ConnectCollectionTransfer {
    type Id = stripe_shared::ConnectCollectionTransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ConnectCollectionTransferId, "connct_");
