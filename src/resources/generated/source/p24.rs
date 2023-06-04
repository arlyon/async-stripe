#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct P24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for P24 {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
