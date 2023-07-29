#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Company {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Company {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
