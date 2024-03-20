#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClimateRemovalsBeneficiary {
    /// Publicly displayable name for the end beneficiary of carbon removal.
    pub public_name: String,
}
