#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_line_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_line_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
