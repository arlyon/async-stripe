use std::str::FromStr;

use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use stripe_shared::ApiVersion;
use stripe_shared::event::EventType;

use crate::{EventObject, WebhookError};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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
    pub id: stripe_shared::event::EventId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Number of webhooks that have yet to be successfully delivered (i.e., to return a 20x response) to the URLs you've specified.
    pub pending_webhooks: i64,
    /// Information on the API request that instigated the event.
    pub request: Option<stripe_shared::NotificationEventRequest>,
    /// Description of the event (e.g., `invoice.created` or `charge.refunded`).
    pub type_: EventType,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EventData {
    /// Object containing the API resource relevant to the event.
    ///
    /// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    pub object: EventObject,
    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    ///
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json_opt")
    )]
    pub previous_attributes: Option<miniserde::json::Value>,
}

#[derive(Debug)]
pub struct Webhook {
    current_timestamp: i64,
}

impl Webhook {
    /// Construct an event from a webhook payload, **ignoring the secret**.
    ///
    /// This method is considered insecure and intended for early-stage local testing only.
    /// Use [construct_event](Self::construct_event) for production instead.
    ///
    /// # Errors
    ///
    /// This function will return a WebhookError if the payload could not be parsed
    pub fn insecure(payload: &str) -> Result<Event, WebhookError> {
        Self { current_timestamp: 0 }.parse_payload(payload)
    }

    /// Construct an event from a webhook payload and signature.
    ///
    /// # Errors
    ///
    /// This function will return a WebhookError if:
    ///  - the provided signature is invalid
    ///  - the provided secret is invalid
    ///  - the signature timestamp is older than 5 minutes
    ///  - the payload could not be parsed
    pub fn construct_event(payload: &str, sig: &str, secret: &str) -> Result<Event, WebhookError> {
        Self { current_timestamp: Utc::now().timestamp() }.do_construct_event(payload, sig, secret)
    }

    /// Construct an event from a webhook payload and signature, verifying its signature
    /// using the provided timestamp.
    ///
    /// This is helpful for replaying requests in tests and should be avoided otherwise
    /// in production use.
    ///
    /// # Errors
    ///
    /// This function will return a WebhookError if:
    /// - the provided signature is invalid
    /// - the provided secret is invalid
    /// - the signature timestamp is older than 5 minutes from the provided timestamp
    /// - the payload could not be parsed
    pub fn construct_event_with_timestamp(
        payload: &str,
        sig: &str,
        secret: &str,
        timestamp: i64,
    ) -> Result<Event, WebhookError> {
        Self { current_timestamp: timestamp }.do_construct_event(payload, sig, secret)
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

        self.parse_payload(payload)
    }

    #[tracing::instrument]
    fn parse_payload(self, payload: &str) -> Result<Event, WebhookError> {
        let base_evt: stripe_shared::Event = miniserde::json::from_str(payload)
            .map_err(|_| WebhookError::BadParse("could not deserialize webhook event".into()))?;

        let event_obj =
            EventObject::from_raw_data(base_evt.type_.as_str(), base_evt.data.object)
                .ok_or_else(|| WebhookError::BadParse("could not parse event object".into()))?;

        // Check for API version mismatch
        let api_version = base_evt.api_version.as_ref().and_then(|s| ApiVersion::from_str(s).ok());

        if let Some(event_version) = &api_version {
            if event_version != &stripe_shared::version::VERSION {
                tracing::warn!(
                    event_version=?event_version,
                    sdk_version=?stripe_shared::version::VERSION,
                    "API version mismatch: SDK compiled with {:?}, but event received with {:?}",
                    stripe_shared::version::VERSION,
                    event_version
                );
            }
        }

        Ok(Event {
            account: base_evt.account,
            api_version: base_evt
                .api_version
                .map(|s| ApiVersion::from_str(&s).unwrap_or(ApiVersion::Unknown(s))),
            created: base_evt.created,
            data: EventData {
                object: event_obj,
                previous_attributes: base_evt.data.previous_attributes,
            },
            id: base_evt.id,
            livemode: base_evt.livemode,
            pending_webhooks: base_evt.pending_webhooks,
            request: base_evt.request,
            type_: base_evt.type_,
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
        let mut t: Option<i64> = None;
        let mut v1: Option<&'r str> = None;
        for pair in raw.split(',') {
            let (key, val) = pair.split_once('=').ok_or(WebhookError::BadSignature)?;
            match key {
                "t" => {
                    t = Some(val.parse().map_err(WebhookError::BadHeader)?);
                }
                "v1" => {
                    v1 = Some(val);
                }
                _ => {}
            }
        }
        Ok(Signature {
            t: t.ok_or(WebhookError::BadSignature)?,
            v1: v1.ok_or(WebhookError::BadSignature)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::{AccountExternalAccountCreated, EventType};

    const WEBHOOK_SECRET: &str = "secret";

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

    fn get_mock_stripe_sig(msg: &str, timestamp: i64) -> String {
        let signed_payload = format!("{timestamp}.{msg}");

        let mut mac = Hmac::<Sha256>::new_from_slice(WEBHOOK_SECRET.as_bytes()).unwrap();

        mac.update(signed_payload.as_bytes());
        let result = mac.finalize().into_bytes();
        let v1 = hex::encode(&result[..]);
        format!("t={timestamp},v1={v1}")
    }

    fn mock_webhook_event(event_type: &EventType, data: Value) -> Value {
        json!({
            "id": "evt_123",
            "object": "event",
            "account": "acct_123",
            "api_version": "2017-05-25",
            "created": 1533204620,
            "livemode": false,
            "pending_webhooks": 1,
            "request": {
                "id": "req_123",
                "idempotency_key": "idempotency-key-123"
            },
            "data": {
                "object": data,
            },
            "type": event_type.to_string()
        })
    }

    #[track_caller]
    fn parse_mock_webhook_event(event_type: EventType, data: Value) -> EventObject {
        let now = Utc::now().timestamp();
        let payload = mock_webhook_event(&event_type, data).to_string();
        let sig = get_mock_stripe_sig(&payload, now);

        let webhook = Webhook { current_timestamp: now };
        let parsed = webhook.do_construct_event(&payload, &sig, WEBHOOK_SECRET).unwrap();
        assert_eq!(parsed.type_, event_type);
        parsed.data.object
    }

    #[test]
    #[cfg(feature = "async-stripe-billing")]
    fn test_webhook_construct_event() {
        let object = json!({
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
        });
        let payload = mock_webhook_event(&EventType::InvoiceitemCreated, object);
        let event_timestamp = 1533204620;
        let signature = format!(
            "t={event_timestamp},v1=5a81ebe328da1df19581cbc6c7377920947ffd30b56eebcc7ba9a6938a090965,v0=63f3a72374a733066c4be69ed7f8e5ac85c22c9f0a6a612ab9a025a9e4ee7eef"
        );

        let webhook = Webhook { current_timestamp: event_timestamp };

        let event = webhook
            .do_construct_event(&payload.to_string(), &signature, WEBHOOK_SECRET)
            .expect("Failed to construct event");

        assert_eq!(event.type_, EventType::InvoiceitemCreated);
        assert_eq!(event.id.as_str(), "evt_123",);
        assert_eq!(event.account, "acct_123".parse().ok());
        assert_eq!(event.created, 1533204620);

        let EventObject::InvoiceitemCreated(invoice) = event.data.object else {
            panic!("expected invoice item created");
        };
        assert_eq!(invoice.id.as_str(), "ii_123");
        assert_eq!(invoice.quantity, 3);
    }

    #[cfg(feature = "async-stripe-billing")]
    #[test]
    // https://github.com/arlyon/async-stripe/issues/455
    fn test_billing_portal_session() {
        let object = json!({
            "configuration": "bpc_123",
            "created": 1533204620,
            "customer": "cus_123",
            "id": "bps_123",
            "livemode": false,
            "url": "http://localhost:3000"
        });
        let result = parse_mock_webhook_event(EventType::BillingPortalSessionCreated, object);
        let EventObject::BillingPortalSessionCreated(session) = result else {
            panic!("expected billing portal session");
        };
        assert_eq!(session.url, "http://localhost:3000");
        assert_eq!(session.id.as_str(), "bps_123");
        assert_eq!(session.configuration.id().as_str(), "bpc_123");
    }

    #[test]
    fn deserialize_polymorphic() {
        let object = json!({
            "object": "bank_account",
            "country": "us",
            "currency": "gbp",
            "id": "ba_123",
            "last4": "1234",
            "status": "status",
        });
        let result = parse_mock_webhook_event(EventType::AccountExternalAccountCreated, object);
        let EventObject::AccountExternalAccountCreated(bank_account) = result else {
            panic!("unexpected type parsed");
        };
        let AccountExternalAccountCreated::BankAccount(bank_account) = *bank_account else {
            panic!("unexpected type parsed");
        };
        assert_eq!(bank_account.id.as_str(), "ba_123");
        assert_eq!(bank_account.last4, "1234");
    }
}
