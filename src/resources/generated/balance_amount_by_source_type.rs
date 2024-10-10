// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BalanceAmountBySourceType".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BalanceAmountBySourceType {
    /// Amount for bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<i64>,

    /// Amount for card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<i64>,

    /// Amount for FPX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<i64>,
}
