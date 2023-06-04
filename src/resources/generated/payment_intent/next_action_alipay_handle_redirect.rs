#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextActionAlipayHandleRedirect {
    /// The native data to be used with Alipay SDK you must redirect your customer to in order to authenticate the payment in an Android App.
    pub native_data: Option<String>,
    /// The native URL you must redirect your customer to in order to authenticate the payment in an iOS App.
    pub native_url: Option<String>,
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate the payment.
    pub url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextActionAlipayHandleRedirect {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
