#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Link {
    /// Account owner's email address.
    pub email: Option<String>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Link {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
