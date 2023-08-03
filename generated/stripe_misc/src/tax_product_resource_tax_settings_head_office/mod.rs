#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxSettingsHeadOffice {
    pub address: stripe_types::Address,
}
