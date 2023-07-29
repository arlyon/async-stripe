#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextActionOxxoDisplayDetails {
    /// The timestamp after which the OXXO voucher expires.
    pub expires_after: Option<stripe_types::Timestamp>,
    /// The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher.
    pub hosted_voucher_url: Option<String>,
    /// OXXO reference number.
    pub number: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextActionOxxoDisplayDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
