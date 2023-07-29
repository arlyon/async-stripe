#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Paynow {
    /// Reference number associated with this PayNow payment.
    pub reference: Option<String>,
}
