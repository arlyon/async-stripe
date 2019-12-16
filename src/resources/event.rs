use crate::error::WebhookError;
use crate::ids::EventId;
use crate::resources::*;

use chrono::Utc;
#[cfg(feature = "webhook-events")]
use hmac::{Hmac, Mac};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "webhook-events")]
use sha2::Sha256;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum EventType {
    #[serde(rename = "account.updated")]
    AccountUpdated,
    #[serde(rename = "account.application.deauthorized")]
    AccountApplicationDeauthorized,
    #[serde(rename = "account.external_account.created")]
    AccountExternalAccountCreated,
    #[serde(rename = "account.external_account.deleted")]
    AccountExternalAccountDeleted,
    #[serde(rename = "account.external_account.updated")]
    AccountExternalAccountUpdated,
    #[serde(rename = "application_fee.created")]
    ApplicationFeeCreated,
    #[serde(rename = "application_fee.refunded")]
    ApplicationFeeRefunded,
    #[serde(rename = "application_fee.refund.updated")]
    ApplicationFeeRefundUpdated,
    #[serde(rename = "balance.available")]
    BalanceAvailable,
    #[serde(rename = "charge.captured")]
    ChargeCaptured,
    #[serde(rename = "charge.failed")]
    ChargeFailed,
    #[serde(rename = "charge.pending")]
    ChargePending,
    #[serde(rename = "charge.refunded")]
    ChargeRefunded,
    #[serde(rename = "charge.succeeded")]
    ChargeSucceeded,
    #[serde(rename = "charge.updated")]
    ChargeUpdated,
    #[serde(rename = "charge.dispute.closed")]
    ChargeDisputeClosed,
    #[serde(rename = "charge.dispute.created")]
    ChargeDisputeCreated,
    #[serde(rename = "charge.dispute.funds_reinstated")]
    ChargeDisputeFundsReinstated,
    #[serde(rename = "charge.dispute.funds_withdrawn")]
    ChargeDisputeFundsWithdrawn,
    #[serde(rename = "charge.dispute.updated")]
    ChargeDisputeUpdated,
    #[serde(rename = "charge.refund.updated")]
    ChargeRefundUpdated,
    #[serde(rename = "checkout.session.completed")]
    CheckoutSessionCompleted,
    #[serde(rename = "coupon.created")]
    CouponCreated,
    #[serde(rename = "coupon.deleted")]
    CouponDeleted,
    #[serde(rename = "coupon.updated")]
    CouponUpdated,
    #[serde(rename = "customer.created")]
    CustomerCreated,
    #[serde(rename = "customer.deleted")]
    CustomerDeleted,
    #[serde(rename = "customer.updated")]
    CustomerUpdated,
    #[serde(rename = "customer.discount.created")]
    CustomerDiscountCreated,
    #[serde(rename = "customer.discount.deleted")]
    CustomerDiscountDeleted,
    #[serde(rename = "customer.discount.updated")]
    CustomerDiscountUpdated,
    #[serde(rename = "customer.source.created")]
    CustomerSourceCreated,
    #[serde(rename = "customer.source.deleted")]
    CustomerSourceDeleted,
    #[serde(rename = "customer.source.updated")]
    CustomerSourceUpdated,
    #[serde(rename = "customer.subscription.created")]
    CustomerSubscriptionCreated,
    #[serde(rename = "customer.subscription.deleted")]
    CustomerSubscriptionDeleted,
    #[serde(rename = "customer.subscription.trial_will_end")]
    CustomerSubscriptionTrialWillEnd,
    #[serde(rename = "customer.subscription.updated")]
    CustomerSubscriptionUpdated,
    #[serde(rename = "file.created")]
    FileCreated,
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[serde(rename = "invoice.deleted")]
    InvoiceDeleted,
    #[serde(rename = "invoice.finalized")]
    InvoiceFinalized,
    #[serde(rename = "invoice.marked_uncollectible")]
    InvoiceMarkedUncollectible,
    #[serde(rename = "invoice.payment_action_required")]
    InvoicePaymentActionRequired,
    #[serde(rename = "invoice.payment_failed")]
    InvoicePaymentFailed,
    #[serde(rename = "invoice.payment_succeeded")]
    InvoicePaymentSucceeded,
    #[serde(rename = "invoice.sent")]
    InvoiceSent,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
    #[serde(rename = "invoice.upcoming")]
    InvoiceUpcoming,
    #[serde(rename = "invoice.voided")]
    InvoiceVoided,
    #[serde(rename = "invoiceitem.created")]
    InvoiceItemCreated,
    #[serde(rename = "invoiceitem.deleted")]
    InvoiceItemDeleted,
    #[serde(rename = "invoiceitem.updated")]
    InvoiceItemUpdated,
    #[serde(rename = "order.created")]
    OrderCreated,
    #[serde(rename = "order.payment_failed")]
    OrderPaymentFailed,
    #[serde(rename = "order.payment_succeeded")]
    OrderPaymentSucceeded,
    #[serde(rename = "order.updated")]
    OrderUpdated,
    #[serde(rename = "order_return.updated")]
    OrderReturnUpdated,
    #[serde(rename = "payment_intent.amount_capturable_updated")]
    PaymentIntentAmountCapturableUpdated,
    #[serde(rename = "payment_intent.created")]
    PaymentIntentCreated,
    #[serde(rename = "payment_intent.payment_failed")]
    PaymentIntentPaymentFailed,
    #[serde(rename = "payment_intent.requires_capture")]
    PaymentIntentRequiresCapture,
    #[serde(rename = "payment_intent.succeeded")]
    PaymentIntentSucceeded,
    #[serde(rename = "payout.canceled")]
    PayoutCanceled,
    #[serde(rename = "payout.created")]
    PayoutCreated,
    #[serde(rename = "payout.failed")]
    PayoutFailed,
    #[serde(rename = "payout.paid")]
    PayoutPaid,
    #[serde(rename = "payout.updated")]
    PayoutUpdated,
    #[serde(rename = "plan.created")]
    PlanCreated,
    #[serde(rename = "plan.deleted")]
    PlanDeleted,
    #[serde(rename = "plan.updated")]
    PlanUpdated,
    #[serde(rename = "product.created")]
    ProductCreated,
    #[serde(rename = "product.deleted")]
    ProductDeleted,
    #[serde(rename = "product.updated")]
    ProductUpdated,
    #[serde(rename = "review.closed")]
    ReviewClosed,
    #[serde(rename = "review.opened")]
    ReviewOpened,
    #[serde(rename = "sigma.scheduled_query_run.created")]
    SigmaScheduledQueryRunCreated,
    #[serde(rename = "sku.created")]
    SkuCreated,
    #[serde(rename = "sku.deleted")]
    SkuDeleted,
    #[serde(rename = "sku.updated")]
    SkuUpdated,
    #[serde(rename = "source.canceled")]
    SourceCanceled,
    #[serde(rename = "source.chargeable")]
    Sourcechargeable,
    #[serde(rename = "source.failed")]
    SourceFailed,
    #[serde(rename = "source.transaction.created")]
    SourceTransactionCreated,
    #[serde(rename = "transfer.created")]
    TransferCreated,
    #[serde(rename = "transfer.reversed")]
    TransferReversed,
    #[serde(rename = "transfer.updated")]
    TransferUpdated,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: EventId,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub data: EventData,
    // ...
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventData {
    pub object: EventObject,
    // previous_attributes: ...
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum EventObject {
    Account(Account),
    ApplicationFee(ApplicationFee),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(ApplicationFeeRefund),
    Balance(Balance),
    BankAccount(BankAccount),
    Charge(Charge),
    Customer(Customer),
    Dispute(Dispute),
    #[serde(rename = "checkout.session")]
    CheckoutSession(CheckoutSession),
    File(File),
    Invoice(Invoice),
    #[serde(rename = "invoiceitem")]
    InvoiceItem(InvoiceItem),
    Order(Order),
    OrderReturn(OrderReturn),
    PaymentIntent(PaymentIntent),
    Payout(Payout),
    Plan(Plan),
    Product(Product),
    Refund(Refund),
    Review(Review),
    Sku(Sku),
    Subscription(Subscription),
    Transfer(Transfer),
}

#[cfg(feature = "webhook-events")]
pub struct Webhook {
    current_timestamp: i64,
}

#[cfg(feature = "webhook-events")]
impl Webhook {
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
        let signature = Signature::parse(&sig)?;
        let signed_payload = format!("{}{}{}", signature.t, ".", payload);

        // Compute HMAC with the SHA256 hash function, using endpoing secret as key
        // and signed_payload string as the message.
        let mut mac =
            Hmac::<Sha256>::new_varkey(secret.as_bytes()).map_err(|_| WebhookError::BadKey)?;
        mac.input(signed_payload.as_bytes());
        let mac_result = mac.result();
        let hex = to_hex(mac_result.code().as_slice());
        if hex != signature.v1 {
            return Err(WebhookError::BadSignature);
        }

        // Get current timestamp to compare to signature timestamp
        if (self.current_timestamp - signature.t).abs() > 300 {
            return Err(WebhookError::BadTimestamp(signature.t));
        }

        serde_json::from_str(&payload).map_err(WebhookError::BadParse)
    }
}

// TODO: If there is a lightweight hex crate, we should just rely on that instead.
fn to_hex(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"0123456789abcdef";

    let mut v = Vec::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
    }

    unsafe { String::from_utf8_unchecked(v) }
}

#[cfg(feature = "webhook-events")]
#[derive(Debug)]
struct Signature<'r> {
    t: i64,
    v1: &'r str,
    v0: Option<&'r str>,
}

#[cfg(feature = "webhook-events")]
impl<'r> Signature<'r> {
    fn parse(raw: &'r str) -> Result<Signature<'r>, WebhookError> {
        use std::collections::HashMap;
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
        let v0 = headers.get("v0").map(|r| *r);
        Ok(Signature { t: t.parse::<i64>().map_err(WebhookError::BadHeader)?, v1, v0 })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "webhook-events")]
    #[test]
    fn test_signature_parse() {
        use super::Signature;

        let raw_signature =
            "t=1492774577,v1=5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd";
        let signature = Signature::parse(raw_signature).unwrap();
        assert_eq!(signature.t, 1492774577);
        assert_eq!(
            signature.v1,
            "5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd"
        );
        assert_eq!(signature.v0, None);

        let raw_signature_with_test_mode = "t=1492774577,v1=5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd,v0=6ffbb59b2300aae63f272406069a9788598b792a944a07aba816edb039989a39";
        let signature = Signature::parse(raw_signature_with_test_mode).unwrap();
        assert_eq!(signature.t, 1492774577);
        assert_eq!(
            signature.v1,
            "5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd"
        );
        assert_eq!(
            signature.v0,
            Some("6ffbb59b2300aae63f272406069a9788598b792a944a07aba816edb039989a39")
        );
    }

    #[cfg(feature = "webhook-events")]
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
      "plan": null,
      "proration": false,
      "quantity": null,
      "subscription": null
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
        let signature = format!("t={},v1=f0bdba6d4eacbd8ad8a3bbadd7248e633ec1477f7899c124c51b39405fa36613,v0=63f3a72374a733066c4be69ed7f8e5ac85c22c9f0a6a612ab9a025a9e4ee7eef", event_timestamp);

        let webhook = super::Webhook { current_timestamp: event_timestamp };

        let event = webhook
            .do_construct_event(payload, &signature, &secret)
            .expect("Failed to construct event");

        assert_eq!(event.event_type, super::EventType::InvoiceItemCreated);
        assert_eq!(event.id.to_string(), "evt_123");
    }
}
