#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Multibanco {
    /// Entity number associated with this Multibanco payment.
    pub entity: Option<String>,
    /// Reference number associated with this Multibanco payment.
    pub reference: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Multibanco {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
