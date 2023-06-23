#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AppliesTo {
    /// A list of product IDs this coupon applies to.
    pub products: Vec<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AppliesTo {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
