#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomUnitAmount {
    /// The maximum unit amount the customer can specify for this item.
    pub maximum: Option<i64>,
    /// The minimum unit amount the customer can specify for this item.
    /// Must be at least the minimum charge amount.
    pub minimum: Option<i64>,
    /// The starting unit amount which can be updated by the customer.
    pub preset: Option<i64>,
}
