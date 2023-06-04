#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NetworkData {
    /// ID from the network that identifies the acquiring financial institution.
    ///
    /// For Visa and Mastercard credit transactions this is as 6 digit code.
    /// For Maestro debit transactions this is a 9 digit code.
    /// Uncommonly, acquiring institution ID is not provided.
    /// When this occurs, the value will be null.
    pub acquiring_institution_id: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NetworkData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
