#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Boleto {
    /// The tax ID of the customer (CPF for individuals consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Boleto {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
