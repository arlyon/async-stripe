/// `Application Fee Refund` objects allow you to refund an application fee that
/// has previously been created but not yet refunded.
///
/// Funds will be refunded to the Stripe account from which the fee was originally collected.  Related guide: [Refunding application fees](https://stripe.com/docs/connect/destination-charges#refunding-app-fee).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FeeRefund {
    /// Amount, in %s.
    pub amount: i64,
    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_types::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the application fee that was refunded.
    pub fee: stripe_types::Expandable<stripe_types::PlatformFee>,
    /// Unique identifier for the object.
    pub id: stripe_types::fee_refund::FeeRefundId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
impl stripe_types::Object for FeeRefund {
    type Id = stripe_types::fee_refund::FeeRefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FeeRefundId);
