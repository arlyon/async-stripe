// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;
use crate::IssuingCardholder;

impl Object for IssuingCardholder {
    type Id = IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.cardholder"
    }
}
