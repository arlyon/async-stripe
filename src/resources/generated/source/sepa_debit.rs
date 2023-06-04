#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SepaDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
