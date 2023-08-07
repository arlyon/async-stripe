use std::collections::HashMap;
use std::str::FromStr;

use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use stripe_types::{ApiVersion, NotificationEvent};

use crate::{EventObject, EventType, WebhookError};

#[derive(Clone, Debug)]
pub struct Event {
    /// The connected account that originated the event.
    pub account: Option<String>,
    /// The Stripe API version used to render `data`.
    ///
    /// *Note: This property is populated only for events on or after October 31, 2014*.
    pub api_version: Option<ApiVersion>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub data: EventData,
    /// Unique identifier for the object.
    pub id: stripe_types::notification_event::EventId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Number of webhooks that have yet to be successfully delivered (i.e., to return a 20x response) to the URLs you've specified.
    pub pending_webhooks: i64,
    /// Information on the API request that instigated the event.
    pub request: Option<stripe_types::NotificationEventRequest>,
    /// Description of the event (e.g., `invoice.created` or `charge.refunded`).
    pub type_: EventType,
}

#[derive(Clone, Debug)]
pub struct EventData {
    /// Object containing the API resource relevant to the event.
    ///
    /// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    pub object: EventObject,
    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    ///
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    pub previous_attributes: Option<serde_json::Value>,
}

pub struct Webhook {
    current_timestamp: i64,
}

impl Webhook {
    /// # Errors
    ///
    /// This function will return a WebhookError if:
    ///  - the provided signature is invalid
    ///  - the provided secret is invalid
    ///  - the signature timestamp is older than 5 minutes
    pub fn construct_event(payload: &str, sig: &str, secret: &str) -> Result<Event, WebhookError> {
        Self { current_timestamp: Utc::now().timestamp() }.do_construct_event(payload, sig, secret)
    }

    fn do_construct_event(
        self,
        payload: &str,
        sig: &str,
        secret: &str,
    ) -> Result<Event, WebhookError> {
        // Get Stripe signature from header
        let signature = Signature::parse(sig)?;
        let signed_payload = format!("{}.{}", signature.t, payload);

        // Compute HMAC with the SHA256 hash function, using endpoint secret as key
        // and signed_payload string as the message.
        let mut mac =
            Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| WebhookError::BadKey)?;
        mac.update(signed_payload.as_bytes());

        let sig = hex::decode(signature.v1).map_err(|_| WebhookError::BadSignature)?;

        mac.verify_slice(sig.as_slice()).map_err(|_| WebhookError::BadSignature)?;

        // Get current timestamp to compare to signature timestamp
        if (self.current_timestamp - signature.t).abs() > 300 {
            return Err(WebhookError::BadTimestamp(signature.t));
        }

        let base_evt: NotificationEvent = serde_json::from_str(payload)?;

        Ok(Event {
            account: base_evt.account,
            api_version: base_evt
                .api_version
                .map(|s| ApiVersion::from_str(&s).unwrap_or(ApiVersion::Unknown)),
            created: base_evt.created,
            data: EventData {
                object: serde_json::from_value(base_evt.data.object)?,
                previous_attributes: base_evt.data.previous_attributes,
            },
            id: base_evt.id,
            livemode: base_evt.livemode,
            pending_webhooks: base_evt.pending_webhooks,
            request: base_evt.request,
            type_: EventType::from_str(&base_evt.type_).unwrap_or(EventType::Unknown),
        })
    }
}

#[derive(Debug)]
struct Signature<'r> {
    t: i64,
    v1: &'r str,
}

impl<'r> Signature<'r> {
    fn parse(raw: &'r str) -> Result<Signature<'r>, WebhookError> {
        let headers: HashMap<&str, &str> = raw
            .split(',')
            .map(|header| {
                let mut key_and_value = header.split('=');
                let key = key_and_value.next();
                let value = key_and_value.next();
                (key, value)
            })
            .filter_map(|(key, value)| match (key, value) {
                (Some(key), Some(value)) => Some((key, value)),
                _ => None,
            })
            .collect();
        let t = headers.get("t").ok_or(WebhookError::BadSignature)?;
        let v1 = headers.get("v1").ok_or(WebhookError::BadSignature)?;
        Ok(Signature { t: t.parse::<i64>().map_err(WebhookError::BadHeader)?, v1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_parse() {
        let raw_signature =
            "t=1492774577,v1=5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd";
        let signature = Signature::parse(raw_signature).unwrap();
        assert_eq!(signature.t, 1492774577);
        assert_eq!(
            signature.v1,
            "5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd"
        );

        let raw_signature_with_test_mode = "t=1492774577,v1=5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd,v0=6ffbb59b2300aae63f272406069a9788598b792a944a07aba816edb039989a39";
        let signature = Signature::parse(raw_signature_with_test_mode).unwrap();
        assert_eq!(signature.t, 1492774577);
        assert_eq!(
            signature.v1,
            "5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd"
        );
    }

    #[test]
    fn test_webhook_construct_event() {
        let payload = r#"{
  "id": "evt_123",
  "object": "event",
  "account": "acct_123",
  "api_version": "2017-05-25",
  "created": 1533204620,
  "data": {
    "object": {
      "id": "ii_123",
      "object": "invoiceitem",
      "amount": 1000,
      "currency": "usd",
      "customer": "cus_123",
      "date": 1533204620,
      "description": "Test Invoice Item",
      "discountable": false,
      "invoice": "in_123",
      "livemode": false,
      "metadata": {},
      "period": {
        "start": 1533204620,
        "end": 1533204620
      },
      "proration": false,
      "quantity": 3
    }
  },
  "livemode": false,
  "pending_webhooks": 1,
  "request": {
    "id": "req_123",
    "idempotency_key": "idempotency-key-123"
  },
  "type": "invoiceitem.created"
}
"#;
        let event_timestamp = 1533204620;
        let secret = "webhook_secret".to_string();
        let signature = format!("t={},v1=82216eca827bcb7b34b8055eb2d2d9e6bc13b9ac39ded14a61e69f70c565f53a,v0=63f3a72374a733066c4be69ed7f8e5ac85c22c9f0a6a612ab9a025a9e4ee7eef", event_timestamp);

        let webhook = Webhook { current_timestamp: event_timestamp };

        let event = webhook
            .do_construct_event(payload, &signature, &secret)
            .expect("Failed to construct event");

        assert_eq!(event.type_, EventType::InvoiceitemCreated);
        assert_eq!(event.id.as_str(), "evt_123",);
        assert_eq!(event.account, "acct_123".parse().ok());
        assert_eq!(event.created, 1533204620);
    }
}
