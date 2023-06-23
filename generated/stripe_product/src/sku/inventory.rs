#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Inventory {
    /// The count of inventory available.
    ///
    /// Will be present if and only if `type` is `finite`.
    pub quantity: Option<u64>,
    /// Inventory type.
    ///
    /// Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    #[serde(rename = "type")]
    pub type_: String,
    /// An indicator of the inventory available.
    ///
    /// Possible values are `in_stock`, `limited`, and `out_of_stock`.
    /// Will be present if and only if `type` is `bucket`.
    pub value: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Inventory {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
