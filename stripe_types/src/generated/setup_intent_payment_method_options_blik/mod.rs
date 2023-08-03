#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupIntentPaymentMethodOptionsBlik {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types::SetupIntentPaymentMethodOptionsMandateOptionsBlik>,
}
