// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_client::{
    ids::FinancialConnectionsAccountOwnerId,
    params::{Object, Timestamp},
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BankConnectionsResourceOwner".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialConnectionsAccountOwner {
    /// Unique identifier for the object.
    pub id: FinancialConnectionsAccountOwnerId,

    /// The email address of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The full name of the owner.
    pub name: String,

    /// The ownership object that this owner belongs to.
    pub ownership: String,

    /// The raw phone number of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The raw physical address of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_address: Option<String>,

    /// The timestamp of the refresh that updated this owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<Timestamp>,
}

impl Object for FinancialConnectionsAccountOwner {
    type Id = FinancialConnectionsAccountOwnerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "financial_connections.account_owner"
    }
}
