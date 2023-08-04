#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarListDeletedListItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_list_list_item::RadarValueListItemId,
}
impl stripe_types::Object for RadarListDeletedListItem {
    type Id = stripe_fraud::radar_list_list_item::RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
