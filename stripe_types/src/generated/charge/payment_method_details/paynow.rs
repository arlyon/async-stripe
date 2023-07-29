#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Paynow {
    /// Reference number associated with this PayNow payment.
    pub reference: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Paynow {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
