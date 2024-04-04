/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing list items](https://stripe.com/docs/radar/lists#managing-list-items)
///
/// For more details see <<https://stripe.com/docs/api/radar/value_list_items/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarValueListItem {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The value of the item.
    pub value: String,
    /// The identifier of the value list this item belongs to.
    pub value_list: String,
}
impl stripe_types::Object for RadarValueListItem {
    type Id = stripe_fraud::RadarValueListItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(RadarValueListItemId);
