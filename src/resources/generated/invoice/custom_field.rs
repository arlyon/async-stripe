#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomField {
    /// The name of the custom field.
    pub name: String,
    /// The value of the custom field.
    pub value: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomField {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
