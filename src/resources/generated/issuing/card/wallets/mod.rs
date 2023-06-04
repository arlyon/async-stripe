#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Wallets {
    pub apple_pay: crate::issuing::card::wallets::apple_pay::ApplePay,
    pub google_pay: crate::issuing::card::wallets::google_pay::GooglePay,
    /// Unique identifier for a card used with digital wallets.
    pub primary_account_identifier: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Wallets {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod apple_pay;
pub use apple_pay::ApplePay;
pub mod google_pay;
pub use google_pay::GooglePay;
