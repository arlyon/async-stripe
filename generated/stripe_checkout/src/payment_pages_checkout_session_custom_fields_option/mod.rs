#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsOption {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: String,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}
