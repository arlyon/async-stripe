#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct WechatPayRedirectToAndroidApp {
    /// app_id is the APP ID registered on WeChat open platform.
    pub app_id: String,
    /// nonce_str is a random string.
    pub nonce_str: String,
    /// package is static value.
    pub package: String,
    /// an unique merchant ID assigned by WeChat Pay.
    pub partner_id: String,
    /// an unique trading ID assigned by WeChat Pay.
    pub prepay_id: String,
    /// A signature.
    pub sign: String,
    /// Specifies the current time in epoch format.
    pub timestamp: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for WechatPayRedirectToAndroidApp {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
