// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::ApplicationFeeRefundId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{ApplicationFee, BalanceTransaction, Currency};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "FeeRefund".
///
/// For more details see [https://stripe.com/docs/api/fee_refunds/object](https://stripe.com/docs/api/fee_refunds/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFeeRefund {
    /// Unique identifier for the object.
    pub id: ApplicationFeeRefundId,

    /// Amount, in %s.
    pub amount: i64,

    /// Balance transaction that describes the impact on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// ID of the application fee that was refunded.
    pub fee: Expandable<ApplicationFee>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,
}

impl Object for ApplicationFeeRefund {
    type Id = ApplicationFeeRefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "fee_refund"
    }
}
