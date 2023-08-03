#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodUsBankAccountStatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<stripe_types::PaymentMethodUsBankAccountBlocked>,
}
