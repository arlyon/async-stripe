// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;
use crate::ApplicationFeeRefund;

impl Object for ApplicationFeeRefund {
    type Id = ApplicationFeeRefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "fee_refund"
    }
}
