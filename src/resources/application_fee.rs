use crate::ids::ApplicationFeeId;
use crate::params::{Expandable, List, Object, Timestamp};
use crate::resources::{
    Account, Application, ApplicationFeeRefund, BalanceTransaction, Charge, Currency,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "PlatformFee".
///
/// For more details see [https://stripe.com/docs/api/application_fees/object](https://stripe.com/docs/api/application_fees/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFee {
    /// Unique identifier for the object.
    pub id: ApplicationFeeId,

    /// ID of the Stripe account this fee was taken from.
    pub account: Expandable<Account>,

    /// Amount earned, in %s.
    pub amount: i64,

    /// Amount in %s refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that earned the fee.
    pub application: Expandable<Application>,

    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// ID of the charge that the application fee was taken from.
    pub charge: Expandable<Charge>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originating_transaction: Option<Expandable<Charge>>,

    /// Whether the fee has been fully refunded.
    ///
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the fee.
    pub refunds: List<ApplicationFeeRefund>,
}

impl Object for ApplicationFee {
    type Id = ApplicationFeeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn object(&self) -> &'static str {
        "application_fee"
    }
}
