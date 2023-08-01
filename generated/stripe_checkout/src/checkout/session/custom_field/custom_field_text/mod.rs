#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldText {
    /// The value entered by the customer.
    pub value: Option<String>,
}
