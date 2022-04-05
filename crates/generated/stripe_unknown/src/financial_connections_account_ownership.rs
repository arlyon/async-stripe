// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};
use stripe::{
    ids::FinancialConnectionsAccountOwnershipId,
    params::{List, Object, Timestamp},
    resources::FinancialConnectionsAccountOwner,
};

/// The resource representing a Stripe "BankConnectionsResourceOwnership".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialConnectionsAccountOwnership {
    /// Unique identifier for the object.
    pub id: FinancialConnectionsAccountOwnershipId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// A paginated list of owners for this account.
    pub owners: List<FinancialConnectionsAccountOwner>,
}

impl Object for FinancialConnectionsAccountOwnership {
    type Id = FinancialConnectionsAccountOwnershipId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "financial_connections.account_ownership"
    }
}
