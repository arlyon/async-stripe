/// For more details see <<https://stripe.com/docs/api/application_fees/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApplicationFee {
    /// ID of the Stripe account this fee was taken from.
    pub account: stripe_types::Expandable<stripe_shared::Account>,
    /// Amount earned, in cents (or local equivalent).
    pub amount: i64,
    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,
    /// ID of the Connect application that earned the fee.
    pub application: stripe_types::Expandable<stripe_shared::Application>,
    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// ID of the charge that the application fee was taken from.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_shared::ApplicationFeeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Whether the fee has been fully refunded.
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the fee.
    pub refunds: stripe_types::List<stripe_shared::ApplicationFeeRefund>,
}
impl stripe_types::Object for ApplicationFee {
    type Id = stripe_shared::ApplicationFeeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ApplicationFeeId, "fee_");
