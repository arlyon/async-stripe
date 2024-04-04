#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedRadarValueList {
    /// Always true for a deleted object
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListId,
}
impl stripe_types::Object for DeletedRadarValueList {
    type Id = stripe_fraud::RadarValueListId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
