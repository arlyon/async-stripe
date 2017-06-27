use error::Error;
use client::Client;
use resources::{Charge, Invoice};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum EventType {
    #[serde(rename = "invoice.created")] InvoiceCreated,
    #[serde(rename = "invoice.updated")] InvoiceUpdated,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "type")] pub event_type: EventType,
    pub data: EventData,
    // ...
}

#[derive(Debug, Deserialize)]
pub struct EventData {
    pub object: EventObject,
    // previous_attributes: ...
}

#[derive(Debug, Deserialize)]
#[serde(tag = "object")]
pub enum EventObject {
    #[serde(rename = "charge")] Charge(Charge),
    #[serde(rename = "invoice")] Invoice(Invoice),
}
