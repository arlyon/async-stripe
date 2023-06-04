#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Details {
    /// Additional fields which are only required for some users.
    pub additional: Vec<String>,
    /// Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Details {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
