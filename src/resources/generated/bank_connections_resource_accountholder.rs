// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable};
use crate::resources::{Account, Customer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BankConnectionsResourceAccountholder".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BankConnectionsResourceAccountholder {

    /// The ID of the Stripe account this account belongs to.
    ///
    /// Should only be present if `account_holder.type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Expandable<Account>>,

    /// ID of the Stripe customer this account belongs to.
    ///
    /// Present if and only if `account_holder.type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// Type of account holder that this account belongs to.
    #[serde(rename = "type")]
    pub type_: BankConnectionsResourceAccountholderType,
}

/// An enum representing the possible values of an `BankConnectionsResourceAccountholder`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceAccountholderType {
    Account,
    Customer,
}

impl BankConnectionsResourceAccountholderType {
    pub fn as_str(self) -> &'static str {
        match self {
            BankConnectionsResourceAccountholderType::Account => "account",
            BankConnectionsResourceAccountholderType::Customer => "customer",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceAccountholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BankConnectionsResourceAccountholderType {
    fn default() -> Self {
        Self::Account
    }
}
