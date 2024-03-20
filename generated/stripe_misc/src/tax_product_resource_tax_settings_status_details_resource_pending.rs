#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePending {
    /// The list of missing fields that are required to perform calculations.
    /// It includes the entry `head_office` when the status is `pending`.
    /// It is recommended to set the optional values even if they aren't listed as required for calculating taxes.
    /// Calculations can fail if missing fields aren't explicitly provided on every call.
    pub missing_fields: Option<Vec<String>>,
}
