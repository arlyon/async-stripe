#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CardPresent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_cryptogram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_preferred_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_response_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvm_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_customer_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_transaction_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_entry_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_verification_results: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status_information: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CardPresent {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
