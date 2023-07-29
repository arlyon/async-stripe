#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Installments {
    /// Installment plan selected for the payment.
    pub plan: Option<
        stripe_types::payment_intent::payment_method_options::card::installments::plan::Plan,
    >,
}
