#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Grabpay {
    /// Unique transaction id generated by GrabPay.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Grabpay {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}