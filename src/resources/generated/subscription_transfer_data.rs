// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable};
use crate::resources::{Account};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionTransferData".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionTransferData {

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    pub amount_percent: Option<f64>,

    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: Expandable<Account>,
}
