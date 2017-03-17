use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;
