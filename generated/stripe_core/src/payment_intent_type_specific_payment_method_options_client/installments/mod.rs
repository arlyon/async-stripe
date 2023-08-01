#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Installments {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<
        stripe_types::payment_intent::payment_method_options::card::installments::plan::Plan,
    >,
}
