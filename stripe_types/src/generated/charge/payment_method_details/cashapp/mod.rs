#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Cashapp {
    /// A unique and immutable identifier assigned by Cash App to every buyer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    /// A public identifier for buyers using Cash App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtag: Option<String>,
}
