/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing list items](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarListListItem {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_list_list_item::RadarValueListItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The value of the item.
    pub value: String,
    /// The identifier of the value list this item belongs to.
    pub value_list: String,
}
impl stripe_types::Object for RadarListListItem {
    type Id = stripe_fraud::radar_list_list_item::RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RadarValueListItemId);
#[cfg(feature = "radar_list_list_item")]
mod requests;
#[cfg(feature = "radar_list_list_item")]
pub use requests::*;
