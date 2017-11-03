use params::Timestamp;

#[derive(Debug, Deserialize)]
pub struct Review {
    pub id: String,
    pub object: String,
    pub charge: String,
    pub created: Timestamp,
    pub livemode: bool,
    pub open: bool,
    pub reason: String,
}
