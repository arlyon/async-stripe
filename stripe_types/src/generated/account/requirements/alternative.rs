#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Alternative {
    /// Fields that can be provided to satisfy all fields in `original_fields_due`.
    pub alternative_fields_due: Vec<String>,
    /// Fields that are due and can be satisfied by providing all fields in `alternative_fields_due`.
    pub original_fields_due: Vec<String>,
}
