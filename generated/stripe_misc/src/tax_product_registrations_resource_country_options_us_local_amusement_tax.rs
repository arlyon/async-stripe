#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    pub jurisdiction: String,
}
