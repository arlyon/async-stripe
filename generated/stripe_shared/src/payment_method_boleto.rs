#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodBoleto {
    /// Uniquely identifies the customer tax id (CNPJ or CPF)
    pub tax_id: String,
}
