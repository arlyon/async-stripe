#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AchCreditTransferData {
    /// Customer data associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<String>,
    /// Bank account fingerprint associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Last 4 digits of the account number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Routing number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AchCreditTransferData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
