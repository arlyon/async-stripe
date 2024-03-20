#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
    /// The value entered by the customer.
    pub value: Option<String>,
}
