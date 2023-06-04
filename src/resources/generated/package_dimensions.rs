#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PackageDimensions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
