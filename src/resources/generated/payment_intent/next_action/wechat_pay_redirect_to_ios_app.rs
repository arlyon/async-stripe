#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct WechatPayRedirectToIosApp {
    /// An universal link that redirect to WeChat Pay app.
    pub native_url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for WechatPayRedirectToIosApp {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
