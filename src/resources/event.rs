use chrono::{Utc};
use error::{Error, ErrorCode, ErrorType, RequestError, WebhookError};
use resources::{Charge, Invoice, Subscription};
use hmac::{Hmac, Mac, MacResult};
use serde_json as json;
use sha2::Sha256;
use std::str;

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

pub struct Webhook {}

impl Webhook {
    pub fn construct_event(payload: String, sig: String, secret: String) -> Result<Event, WebhookError> {
        let mut headers: Vec<String> = sig.split(",").map(|s| s.trim().to_string()).collect();

        // Prepare the signed payload
        let ref mut timestamp: Vec<String> = headers[0].split("=").map(|s| s.to_string()).collect();
        let signed_payload = format!("{}{}{}", timestamp[1], ".", payload);

        // Get Stripe signature from header
        let ref mut signature: Vec<String> = headers[1].split("=").map(|s| s.to_string()).collect();

        // Compute HMAC with the SHA256 hash function, using endpoing secret as key and signed_payload string as the message
        let mut mac = Hmac::<Sha256>::new(secret.as_bytes());
        mac.input(signed_payload.as_bytes());

        let result = mac.result();

        let bytes_signature = MacResult::from_slice(signature[1].as_bytes());

        // Get current timestamp to compare to signature timestamp
        let current = Utc::now().timestamp();
        let num_timestamp = timestamp[1].parse::<i64>()
            .map_err(|err| WebhookError::BadHeader(err))?;

        if bytes_signature != result {
            return Err(WebhookError::BadSignature);
        }

        if current - num_timestamp > 300 {
            return Err(WebhookError::BadTimestamp(num_timestamp));
        }

        // return Event
        return json::from_str(&payload).map_err(|err| WebhookError::BadParse(err));
    }
}
