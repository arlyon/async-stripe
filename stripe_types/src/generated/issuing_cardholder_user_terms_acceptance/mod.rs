#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderUserTermsAcceptance {
    /// The Unix timestamp marking when the cardholder accepted the Authorized User Terms.
    ///
    /// Required for Celtic Spend Card users.
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the cardholder accepted the Authorized User Terms.
    ///
    /// Required for Celtic Spend Card users.
    pub ip: Option<String>,
    /// The user agent of the browser from which the cardholder accepted the Authorized User Terms.
    pub user_agent: Option<String>,
}
