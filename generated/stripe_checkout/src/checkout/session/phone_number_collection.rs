#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PhoneNumberCollection {
    /// Indicates whether phone number collection is enabled for the session.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PhoneNumberCollection {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
