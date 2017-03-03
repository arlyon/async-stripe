use resources::{Card, CardParams};

#[derive(Serialize)]
pub enum SourceParams {
    Token(String),
    Card(CardParams),
}

#[derive(Deserialize)]
#[serde(tag = "object")]
pub enum Source {
    // BitcoinReceiver(...),

    #[serde(rename = "card")]
    Card(Card),
}
