#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AchCreditTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_routing_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AchCreditTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
