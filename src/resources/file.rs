use params::Timestamp;

#[derive(Debug, Deserialize)]
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
