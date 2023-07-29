#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Installments {
    /// Installment plans that may be selected for this PaymentIntent.
    pub available_plans: Option<
        Vec<stripe_types::payment_intent::payment_method_options::card::installments::plan::Plan>,
    >,
    /// Whether Installments are enabled for this PaymentIntent.
    pub enabled: bool,
    /// Installment plan selected for this PaymentIntent.
    pub plan: Option<
        stripe_types::payment_intent::payment_method_options::card::installments::plan::Plan,
    >,
}
pub mod plan;
pub use plan::Plan;
