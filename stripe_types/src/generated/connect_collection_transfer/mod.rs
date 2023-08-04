#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in %s.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the account that funds are being collected for.
    pub destination: stripe_types::Expandable<stripe_types::Account>,
    /// Unique identifier for the object.
    pub id: stripe_types::connect_collection_transfer::ConnectCollectionTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
impl stripe_types::Object for ConnectCollectionTransfer {
    type Id = stripe_types::connect_collection_transfer::ConnectCollectionTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ConnectCollectionTransferId, "connct_");
