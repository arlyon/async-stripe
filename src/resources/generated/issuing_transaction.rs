// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;
use crate::IssuingTransaction;

impl Object for IssuingTransaction {
    type Id = IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.transaction"
    }
}
