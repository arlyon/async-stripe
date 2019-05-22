// ======================================
// This file was automatically generated.
// ======================================

use crate::params::Object;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Application".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    /// The name of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Object for Application {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "application"
    }
}
