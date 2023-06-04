#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ApplePay {}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApplePay {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
