use serde::{Deserialize, Serialize};

use crate::resources::{BankAccount, Card};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(BankAccount),
    Card(Card),
}
impl std::default::Default for ExternalAccount {
    fn default() -> Self {
        Self::BankAccount(Default::default())
    }
}
