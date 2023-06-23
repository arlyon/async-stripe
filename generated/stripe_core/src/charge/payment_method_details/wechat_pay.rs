#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct WechatPay {
    /// Uniquely identifies this particular WeChat Pay account.
    ///
    /// You can use this attribute to check whether two WeChat accounts are the same.
    pub fingerprint: Option<String>,
    /// Transaction ID of this particular WeChat Pay transaction.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for WechatPay {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
