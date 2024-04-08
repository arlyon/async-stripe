/// `Application Fee Refund` objects allow you to refund an application fee that
/// has previously been created but not yet refunded. Funds will be refunded to
/// the Stripe account from which the fee was originally collected.
///
/// Related guide: [Refunding application fees](https://stripe.com/docs/connect/destination-charges#refunding-app-fee).
///
/// For more details see <<https://stripe.com/docs/api/fee_refunds/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApplicationFeeRefund {
    /// Amount, in cents (or local equivalent).
    pub amount: i64,
    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the application fee that was refunded.
    pub fee: stripe_types::Expandable<stripe_shared::ApplicationFee>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationFeeRefundId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
impl stripe_types::Object for ApplicationFeeRefund {
    type Id = stripe_shared::ApplicationFeeRefundId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplicationFeeRefundId);
