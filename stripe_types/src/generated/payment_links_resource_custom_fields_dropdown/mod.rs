#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceCustomFieldsDropdown {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: Vec<stripe_types::PaymentLinksResourceCustomFieldsDropdownOption>,
}
