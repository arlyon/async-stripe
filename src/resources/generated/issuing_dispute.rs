// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;
use crate::IssuingDispute;

impl Object for IssuingDispute {
    type Id = IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.dispute"
    }
}
