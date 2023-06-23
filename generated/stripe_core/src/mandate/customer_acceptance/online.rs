#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Online {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: Option<String>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Online {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
