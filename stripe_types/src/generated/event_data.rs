#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EventData {
    /// Object containing the API resource relevant to the event.
    ///
    /// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    pub object: EventDataObject,
    /// Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<EventDataPreviousAttributes>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EventData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Object containing the API resource relevant to the event.
///
/// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EventDataObject {}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EventDataObject {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EventDataPreviousAttributes {}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EventDataPreviousAttributes {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
