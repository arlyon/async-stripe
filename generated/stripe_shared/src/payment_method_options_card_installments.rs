#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptionsCardInstallments {
    /// Installment plans that may be selected for this PaymentIntent.
    pub available_plans: Option<Vec<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>>,
    /// Whether Installments are enabled for this PaymentIntent.
    pub enabled: bool,
    /// Installment plan selected for this PaymentIntent.
    pub plan: Option<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>,
}
