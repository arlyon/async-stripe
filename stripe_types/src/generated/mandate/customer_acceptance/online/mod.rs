#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Online {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: Option<String>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: Option<String>,
}
