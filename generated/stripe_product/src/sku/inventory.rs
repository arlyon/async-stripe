#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
