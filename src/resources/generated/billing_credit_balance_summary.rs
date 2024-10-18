// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable, Object};
use crate::resources::{BillingCreditGrantsResourceAmount, Customer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CreditBalanceSummary".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditBalanceSummary {

    /// The credit balances.
    ///
    /// One entry per credit grant currency.
    /// If a customer only has credit grants in a single currency, then this will have a single balance entry.
    pub balances: Vec<CreditBalance>,

    /// The customer the balance is for.
    pub customer: Expandable<Customer>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl Object for BillingCreditBalanceSummary {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "billing.credit_balance_summary"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreditBalance {

    pub available_balance: BillingCreditGrantsResourceAmount,

    pub ledger_balance: BillingCreditGrantsResourceAmount,
}
