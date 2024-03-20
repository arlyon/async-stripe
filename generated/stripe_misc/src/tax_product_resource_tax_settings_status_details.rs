#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxSettingsStatusDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}
