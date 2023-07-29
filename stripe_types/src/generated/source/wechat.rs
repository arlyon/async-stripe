#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Wechat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepay_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
