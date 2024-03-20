#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettings {
    pub issuer: stripe_shared::ConnectAccountReference,
}
