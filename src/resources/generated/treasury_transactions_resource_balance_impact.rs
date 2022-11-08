// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasuryTransactionsResourceBalanceImpact".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasuryTransactionsResourceBalanceImpact {

    /// The change made to funds the user can spend right now.
    pub cash: i64,

    /// The change made to funds that are not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,

    /// The change made to funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}
