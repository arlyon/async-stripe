#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Company {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
