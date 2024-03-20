#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedRadarValueListItem {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListItemId,
}
impl stripe_types::Object for DeletedRadarValueListItem {
    type Id = stripe_fraud::RadarValueListItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
