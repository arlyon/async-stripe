#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Alipay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Alipay {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
