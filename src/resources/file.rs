use crate::params::{Paginated, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe file.
///
/// For more details see https://stripe.com/docs/api#file_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub id: String,
    pub object: String,
    pub created: Timestamp,
    pub purpose: String,
    pub size: u64,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub file_type: String, // (csv, pdf, jpg, png)
    pub url: String,
}

impl Paginated for File {
    fn cursor(&self) -> &str {
        &self.id
    }
}
