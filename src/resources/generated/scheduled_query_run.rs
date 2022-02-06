// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;
use crate::ScheduledQueryRun;

impl Object for ScheduledQueryRun {
    type Id = ScheduledQueryRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "scheduled_query_run"
    }
}
