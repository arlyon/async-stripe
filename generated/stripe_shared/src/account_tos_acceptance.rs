#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountTosAcceptance {
    /// The Unix timestamp marking when the account representative accepted their service agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted their service agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user's service agreement type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<String>,
    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
