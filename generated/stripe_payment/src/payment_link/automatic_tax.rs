#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
