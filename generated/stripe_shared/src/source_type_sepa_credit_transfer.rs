#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceTypeSepaCreditTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_iban: Option<String>,
}
