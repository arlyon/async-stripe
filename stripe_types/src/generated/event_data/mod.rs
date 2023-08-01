#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EventData {
    /// Object containing the API resource relevant to the event.
    ///
    /// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    pub object: EventDataObject,
    /// Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<EventDataPreviousAttributes>,
}
/// Object containing the API resource relevant to the event.
///
/// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EventDataObject {}
/// Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EventDataPreviousAttributes {}
