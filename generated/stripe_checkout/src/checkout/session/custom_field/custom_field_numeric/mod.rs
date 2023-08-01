#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldNumeric {
    /// The value entered by the customer, containing only digits.
    pub value: Option<String>,
}
