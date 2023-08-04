#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarListDeletedList {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_list_list::RadarValueListId,
}
impl stripe_types::Object for RadarListDeletedList {
    type Id = stripe_fraud::radar_list_list::RadarValueListId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
