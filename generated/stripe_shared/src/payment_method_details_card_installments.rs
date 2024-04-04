#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsCardInstallments {
    /// Installment plan selected for the payment.
    pub plan: Option<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>,
}
