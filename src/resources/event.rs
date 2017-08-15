use resources::{Charge, Invoice, Subscription};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum EventType {
    #[serde(rename = "charge.succeeded")]
    ChargeSucceeded,
    #[serde(rename = "customer.subscription.created")]
    CustomerSubscriptionCreated,
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
    #[serde(rename = "invoice.upcoming")]
    InvoiceUpcoming,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: EventType,
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
    #[serde(rename = "charge")]
    Charge(Charge),
    #[serde(rename = "invoice")]
    Invoice(Invoice),
    #[serde(rename = "subscription")]
    Subscription(Subscription),
}
