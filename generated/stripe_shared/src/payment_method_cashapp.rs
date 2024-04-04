#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodCashapp {
    /// A unique and immutable identifier assigned by Cash App to every buyer.
    pub buyer_id: Option<String>,
    /// A public identifier for buyers using Cash App.
    pub cashtag: Option<String>,
}
