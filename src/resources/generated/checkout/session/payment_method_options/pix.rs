#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Pix {
    /// The number of seconds after which Pix payment will expire.
    pub expires_after_seconds: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Pix {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
