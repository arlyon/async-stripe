#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    line1: String,
    line2: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
}
