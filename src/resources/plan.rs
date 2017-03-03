use params::Metadata;

#[derive(Deserialize)]
pub struct Plan {
    pub id: String,
    pub amount: u64,
    pub created: i64, // unix timestamp
    pub currency: String, // eg. "usd"
    pub interval: String, // (day, week, month, year)
    pub interval_count: u64,
    pub livemode: bool,
    pub metadata: Metadata,
    pub name: String,
    pub statement_descriptor: Option<String>,
    pub trial_period_days: Option<u64>,
}
