#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Details {
    /// Funds that are available for use.
    pub available: Vec<stripe_core::money::Money>,
}
