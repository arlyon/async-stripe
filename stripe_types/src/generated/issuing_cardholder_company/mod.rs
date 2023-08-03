#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderCompany {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
