// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BalanceAmountBySourceType".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BalanceAmountBySourceType {

    /// Amount coming from [legacy US ACH payments](https://docs.stripe.com/ach-deprecated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<i64>,

    /// Amount coming from most payment methods, including cards as well as [non-legacy bank debits](https://docs.stripe.com/payments/bank-debits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<i64>,

    /// Amount coming from [FPX](https://docs.stripe.com/payments/fpx), a Malaysian payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<i64>,
}
