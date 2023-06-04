#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Lodging {
    /// The time of checking into the lodging.
    pub check_in_at: Option<i64>,
    /// The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Lodging {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
