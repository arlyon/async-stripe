// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;
use crate::IssuingAuthorization;

impl Object for IssuingAuthorization {
    type Id = IssuingAuthorizationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.authorization"
    }
}
