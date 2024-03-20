#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceTypeAuBecsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}
