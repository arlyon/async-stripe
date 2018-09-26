#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}
