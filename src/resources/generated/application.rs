// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::params::Object;

/// The resource representing a Stripe "Application".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Application {
    /// The name of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,
}

impl Object for Application {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "application"
    }
}
