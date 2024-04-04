#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsBoleto {
    /// The tax ID of the customer (CPF for individuals consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
