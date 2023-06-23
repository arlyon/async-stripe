#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Receipt {
    /// The description of the item.
    ///
    /// The maximum length of this field is 26 characters.
    pub description: Option<String>,
    /// The quantity of the item.
    pub quantity: Option<f64>,
    /// The total for this line item in cents.
    pub total: Option<i64>,
    /// The unit cost of the item in cents.
    pub unit_cost: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Receipt {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
