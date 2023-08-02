#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldDropdown {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: Vec<stripe_checkout::custom_field_dropdown_option::CustomFieldDropdownOption>,
    /// The option selected by the customer.
    ///
    /// This will be the `value` for the option.
    pub value: Option<String>,
}
