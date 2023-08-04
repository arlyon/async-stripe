#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlatformFee {
    /// ID of the Stripe account this fee was taken from.
    pub account: stripe_types::Expandable<stripe_types::Account>,
    /// Amount earned, in %s.
    pub amount: i64,
    /// Amount in %s refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,
    /// ID of the Connect application that earned the fee.
    pub application: stripe_types::Expandable<stripe_types::Application>,
    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction: Option<stripe_types::Expandable<stripe_types::BalanceTransaction>>,
    /// ID of the charge that the application fee was taken from.
    pub charge: stripe_types::Expandable<stripe_types::Charge>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_types::platform_fee::ApplicationFeeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<stripe_types::Expandable<stripe_types::Charge>>,
    /// Whether the fee has been fully refunded.
    ///
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the fee.
    pub refunds: stripe_types::List<stripe_types::FeeRefund>,
}
impl stripe_types::Object for PlatformFee {
    type Id = stripe_types::platform_fee::ApplicationFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ApplicationFeeId, "fee_");
