use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deleted {
    pub deleted: bool,
    pub id: String,
}
