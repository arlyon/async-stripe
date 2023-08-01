#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomField {
    /// The name of the custom field.
    pub name: String,
    /// The value of the custom field.
    pub value: String,
}
