#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Link {
    /// Token used for persistent Link logins.
    pub persistent_token: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Link {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
