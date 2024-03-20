#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: stripe_shared::UsageRecordSummaryId,
    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub period: stripe_shared::Period,
    /// The ID of the subscription item this summary is describing.
    pub subscription_item: String,
    /// The total usage within this usage period.
    pub total_usage: i64,
}
impl stripe_types::Object for UsageRecordSummary {
    type Id = stripe_shared::UsageRecordSummaryId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(UsageRecordSummaryId, "urs_" | "sis_");
