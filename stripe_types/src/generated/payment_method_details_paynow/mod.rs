#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsPaynow {
    /// Reference number associated with this PayNow payment.
    pub reference: Option<String>,
}
