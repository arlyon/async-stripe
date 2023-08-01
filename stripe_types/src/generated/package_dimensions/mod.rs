#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PackageDimensions {
    /// Height, in inches.
    pub height: f64,
    /// Length, in inches.
    pub length: f64,
    /// Weight, in ounces.
    pub weight: f64,
    /// Width, in inches.
    pub width: f64,
}
