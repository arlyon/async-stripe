#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OnlineAcceptance {
    /// The customer accepts the mandate from this IP address.
    pub ip_address: Option<String>,
    /// The customer accepts the mandate using the user agent of the browser.
    pub user_agent: Option<String>,
}
