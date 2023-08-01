#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Fuel {
    /// The type of fuel that was purchased.
    ///
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The units for `volume_decimal`.
    ///
    /// One of `us_gallon` or `liter`.
    pub unit: String,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: String,
    /// The volume of the fuel that was pumped, represented as a decimal string with at most 12 decimal places.
    pub volume_decimal: Option<String>,
}
