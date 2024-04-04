#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NotificationEventData {
    /// Object containing the API resource relevant to the event.
    /// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    pub object: serde_json::Value,
    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<serde_json::Value>,
}
