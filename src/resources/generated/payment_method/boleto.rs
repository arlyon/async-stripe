#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Boleto {
    /// Uniquely identifies the customer tax id (CNPJ or CPF).
    pub tax_id: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Boleto {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
