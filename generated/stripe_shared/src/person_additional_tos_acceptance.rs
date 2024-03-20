#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PersonAdditionalTosAcceptance {
    /// The Unix timestamp marking when the legal guardian accepted the service agreement.
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the legal guardian accepted the service agreement.
    pub ip: Option<String>,
    /// The user agent of the browser from which the legal guardian accepted the service agreement.
    pub user_agent: Option<String>,
}
