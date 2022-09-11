// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_common::resources::Currency;
use async_stripe_core::{
    ids::IssuingDisputeId,
    params::{Expandable, Metadata, Object, Timestamp},
    resources::IssuingDisputeStatus,
};
use serde::{Deserialize, Serialize};

use crate::resources::{BalanceTransaction, IssuingDisputeEvidence, IssuingTransaction};

/// The resource representing a Stripe "IssuingDispute".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDispute {
    /// Unique identifier for the object.
    pub id: IssuingDisputeId,

    /// Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,

    /// List of balance transactions associated with the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transactions: Option<Vec<BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The currency the `transaction` was made in.
    pub currency: Currency,

    pub evidence: IssuingDisputeEvidence,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Current status of the dispute.
    pub status: IssuingDisputeStatus,

    /// The transaction being disputed.
    pub transaction: Expandable<IssuingTransaction>,

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<IssuingDisputeTreasury>,
}

impl Object for IssuingDispute {
    type Id = IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.dispute"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDisputeTreasury {
    /// The Treasury [DebitReversal](https://stripe.com/docs/api/treasury/debit_reversals) representing this Issuing dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<String>,

    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) that is being disputed.
    pub received_debit: String,
}
