#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomUnitAmount {
    /// The maximum unit amount the customer can specify for this item.
    pub maximum: Option<i64>,
    /// The minimum unit amount the customer can specify for this item.
    ///
    /// Must be at least the minimum charge amount.
    pub minimum: Option<i64>,
    /// The starting unit amount which can be updated by the customer.
    pub preset: Option<i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomUnitAmount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
