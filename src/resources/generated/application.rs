// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::ApplicationId;
use crate::params::Object;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Application".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: ApplicationId,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The name of the application.
    pub name: Option<String>,
}

impl Object for Application {
    type Id = ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "application"
    }
}
