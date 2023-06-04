#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Bancontact {}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Bancontact {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
