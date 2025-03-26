use std::collections::HashMap;

use chrono::Utc;
#[cfg(feature = "webhook-events")]
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "webhook-events")]
use sha2::Sha256;
use smart_default::SmartDefault;

use crate::error::WebhookError;
use crate::resources::*;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, SmartDefault)]
pub enum EventType {
    #[serde(rename = "account.application.authorized")]
    AccountApplicationAuthorized,
    #[serde(rename = "account.application.deauthorized")]
    AccountApplicationDeauthorized,
    #[serde(rename = "account.external_account.created")]
    AccountExternalAccountCreated,
    #[serde(rename = "account.external_account.deleted")]
    AccountExternalAccountDeleted,
    #[serde(rename = "account.external_account.updated")]
    AccountExternalAccountUpdated,
    #[serde(rename = "account.updated")]
    AccountUpdated,
    #[serde(rename = "application_fee.created")]
    ApplicationFeeCreated,
    #[serde(rename = "application_fee.refund.updated")]
    ApplicationFeeRefundUpdated,
    #[serde(rename = "application_fee.refunded")]
    ApplicationFeeRefunded,
    #[serde(rename = "balance.available")]
    BalanceAvailable,
    #[serde(rename = "billing_portal.configuration.created")]
    BillingPortalConfigurationCreated,
    #[serde(rename = "billing_portal.configuration.updated")]
    BillingPortalConfigurationUpdated,
    #[serde(rename = "capability.updated")]
    CapabilityUpdated,
    #[serde(rename = "cash_balance.funds_available")]
    CashBalanceFundsAvailable,
    #[serde(rename = "charge.captured")]
    ChargeCaptured,
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
    #[serde(rename = "charge.expired")]
    ChargeExpired,
    #[serde(rename = "charge.failed")]
    ChargeFailed,
    #[serde(rename = "charge.pending")]
    ChargePending,
    #[serde(rename = "charge.refund.updated")]
    ChargeRefundUpdated,
    #[serde(rename = "charge.refunded")]
    ChargeRefunded,
    #[serde(rename = "charge.succeeded")]
    ChargeSucceeded,
    #[serde(rename = "charge.updated")]
    ChargeUpdated,
    #[serde(rename = "checkout.session.async_payment_failed")]
    CheckoutSessionAsyncPaymentFailed,
    #[serde(rename = "checkout.session.async_payment_succeeded")]
    CheckoutSessionAsyncPaymentSucceeded,
    #[serde(rename = "checkout.session.completed")]
    CheckoutSessionCompleted,
    #[serde(rename = "checkout.session.expired")]
    CheckoutSessionExpired,
    #[serde(rename = "coupon.created")]
    CouponCreated,
    #[serde(rename = "coupon.deleted")]
    CouponDeleted,
    #[serde(rename = "coupon.updated")]
    CouponUpdated,
    #[serde(rename = "credit_note.created")]
    CreditNoteCreated,
    #[serde(rename = "credit_note.updated")]
    CreditNoteUpdated,
    #[serde(rename = "credit_note.voided")]
    CreditNoteVoided,
    #[serde(rename = "customer.created")]
    CustomerCreated,
    #[serde(rename = "customer.deleted")]
    CustomerDeleted,
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
    #[serde(rename = "customer.source.expiring")]
    CustomerSourceExpiring,
    #[serde(rename = "customer.source.updated")]
    CustomerSourceUpdated,
    #[serde(rename = "customer.subscription.created")]
    CustomerSubscriptionCreated,
    #[serde(rename = "customer.subscription.deleted")]
    CustomerSubscriptionDeleted,
    #[serde(rename = "customer.subscription.paused")]
    CustomerSubscriptionPaused,
    #[serde(rename = "customer.subscription.pending_update_applied")]
    CustomerSubscriptionPendingUpdateApplied,
    #[serde(rename = "customer.subscription.pending_update_expired")]
    CustomerSubscriptionPendingUpdateExpired,
    #[serde(rename = "customer.subscription.resumed")]
    CustomerSubscriptionResumed,
    #[serde(rename = "customer.subscription.trial_will_end")]
    CustomerSubscriptionTrialWillEnd,
    #[serde(rename = "customer.subscription.updated")]
    CustomerSubscriptionUpdated,
    #[serde(rename = "customer.tax_id.created")]
    CustomerTaxIdCreated,
    #[serde(rename = "customer.tax_id.deleted")]
    CustomerTaxIdDeleted,
    #[serde(rename = "customer.tax_id.updated")]
    CustomerTaxIdUpdated,
    #[serde(rename = "customer.updated")]
    CustomerUpdated,
    #[serde(rename = "file.created")]
    FileCreated,
    #[serde(rename = "identity.verification_session.canceled")]
    IdentityVerificationSessionCanceled,
    #[serde(rename = "identity.verification_session.created")]
    IdentityVerificationSessionCreated,
    #[serde(rename = "identity.verification_session.processing")]
    IdentityVerificationSessionProcessing,
    #[serde(rename = "identity.verification_session.redacted")]
    IdentityVerificationSessionRedacted,
    #[serde(rename = "identity.verification_session.requires_input")]
    IdentityVerificationSessionRequiresInput,
    #[serde(rename = "identity.verification_session.verified")]
    IdentityVerificationSessionVerified,
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[serde(rename = "invoice.deleted")]
    InvoiceDeleted,
    #[serde(rename = "invoice.finalization_failed")]
    InvoiceFinalizationFailed,
    #[serde(rename = "invoice.finalized")]
    InvoiceFinalized,
    #[serde(rename = "invoice.marked_uncollectible")]
    InvoiceMarkedUncollectible,
    #[serde(rename = "invoice.paid")]
    InvoicePaid,
    #[serde(rename = "invoice.payment_action_required")]
    InvoicePaymentActionRequired,
    #[serde(rename = "invoice.payment_failed")]
    InvoicePaymentFailed,
    #[serde(rename = "invoice.payment_succeeded")]
    InvoicePaymentSucceeded,
    #[serde(rename = "invoice.sent")]
    InvoiceSent,
    #[serde(rename = "invoice.upcoming")]
    InvoiceUpcoming,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
    #[serde(rename = "invoice.voided")]
    InvoiceVoided,
    #[serde(rename = "invoiceitem.created")]
    InvoiceItemCreated,
    #[serde(rename = "invoiceitem.deleted")]
    InvoiceItemDeleted,
    #[serde(rename = "invoiceitem.updated")]
    InvoiceItemUpdated,
    #[serde(rename = "issuing_authorization.created")]
    IssuingAuthorizationCreated,
    #[serde(rename = "issuing_authorization.request")]
    IssuingAuthorizationRequest,
    #[serde(rename = "issuing_authorization.updated")]
    IssuingAuthorizationUpdated,
    #[serde(rename = "issuing_card.created")]
    IssuingCardCreated,
    #[serde(rename = "issuing_card.updated")]
    IssuingCardUpdated,
    #[serde(rename = "issuing_cardholder.created")]
    IssuingCardholderCreated,
    #[serde(rename = "issuing_cardholder.updated")]
    IssuingCardholderUpdated,
    #[serde(rename = "issuing_dispute.closed")]
    IssuingDisputeClosed,
    #[serde(rename = "issuing_dispute.created")]
    IssuingDisputeCreated,
    #[serde(rename = "issuing_dispute.funds_reinstated")]
    IssuingDisputeFundsReinstated,
    #[serde(rename = "issuing_dispute.submitted")]
    IssuingDisputeSubmitted,
    #[serde(rename = "issuing_dispute.updated")]
    IssuingDisputeUpdated,
    #[serde(rename = "issuing_transaction.created")]
    IssuingTransactionCreated,
    #[serde(rename = "issuing_transaction.updated")]
    IssuingTransactionUpdated,
    #[serde(rename = "mandate.updated")]
    MandateUpdated,
    #[serde(rename = "order.created")]
    OrderCreated,
    #[serde(rename = "order.payment_failed")]
    OrderPaymentFailed,
    #[serde(rename = "order.payment_succeeded")]
    OrderPaymentSucceeded,
    #[serde(rename = "order.updated")]
    OrderUpdated,
    #[serde(rename = "order_return.created")]
    OrderReturnCreated,
    #[serde(rename = "order_return.updated")]
    OrderReturnUpdated,
    #[serde(rename = "payment_intent.amount_capturable_updated")]
    PaymentIntentAmountCapturableUpdated,
    #[serde(rename = "payment_intent.canceled")]
    PaymentIntentCanceled,
    #[serde(rename = "payment_intent.created")]
    PaymentIntentCreated,
    #[serde(rename = "payment_intent.partially_funded")]
    PaymentIntentPartiallyFunded,
    #[serde(rename = "payment_intent.payment_failed")]
    PaymentIntentPaymentFailed,
    #[serde(rename = "payment_intent.processing")]
    PaymentIntentProcessing,
    #[serde(rename = "payment_intent.requires_action")]
    PaymentIntentRequiresAction,
    #[serde(rename = "payment_intent.requires_capture")]
    PaymentIntentRequiresCapture,
    #[serde(rename = "payment_intent.succeeded")]
    PaymentIntentSucceeded,
    #[serde(rename = "payment_link.created")]
    PaymentLinkCreated,
    #[serde(rename = "payment_link.updated")]
    PaymentLinkUpdated,
    #[serde(rename = "payment_method.attached")]
    PaymentMethodAttached,
    #[serde(rename = "payment_method.automatically_updated")]
    PaymentMethodAutomaticallyUpdated,
    #[serde(rename = "payment_method.detached")]
    PaymentMethodDetached,
    #[serde(rename = "payment_method.updated")]
    PaymentMethodUpdated,
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
    #[serde(rename = "person.created")]
    PersonCreated,
    #[serde(rename = "person.deleted")]
    PersonDeleted,
    #[serde(rename = "person.updated")]
    PersonUpdated,
    #[serde(rename = "plan.created")]
    PlanCreated,
    #[serde(rename = "plan.deleted")]
    PlanDeleted,
    #[serde(rename = "plan.updated")]
    PlanUpdated,
    #[serde(rename = "price.created")]
    PriceCreated,
    #[serde(rename = "price.deleted")]
    PriceDeleted,
    #[serde(rename = "price.updated")]
    PriceUpdated,
    #[serde(rename = "product.created")]
    ProductCreated,
    #[serde(rename = "product.deleted")]
    ProductDeleted,
    #[serde(rename = "product.updated")]
    ProductUpdated,
    #[serde(rename = "promotion_code.created")]
    PromotionCodeCreated,
    #[serde(rename = "promotion_code.updated")]
    PromotionCodeUpdated,
    #[serde(rename = "quote.accepted")]
    QuoteAccepted,
    #[serde(rename = "quote.canceled")]
    QuoteCanceled,
    #[serde(rename = "quote.created")]
    QuoteCreated,
    #[serde(rename = "quote.finalized")]
    QuoteFinalized,
    #[serde(rename = "radar.early_fraud_warning.created")]
    RadarEarlyFraudWarningCreated,
    #[serde(rename = "radar.early_fraud_warning.updated")]
    RadarEarlyFraudWarningUpdated,
    #[serde(rename = "recipient.created")]
    RecipientCreated,
    #[serde(rename = "recipient.deleted")]
    RecipientDeleted,
    #[serde(rename = "recipient.updated")]
    RecipientUpdated,
    #[serde(rename = "refund.created")]
    RefundCreated,
    #[serde(rename = "refund.updated")]
    RefundUpdated,
    #[serde(rename = "reporting.report_run.failed")]
    ReportingReportRunFailed,
    #[serde(rename = "reporting.report_run.succeeded")]
    ReportingReportRunSucceeded,
    #[serde(rename = "reporting.report_type.updated")]
    ReportingReportTypeUpdated,
    #[serde(rename = "review.closed")]
    ReviewClosed,
    #[serde(rename = "review.opened")]
    ReviewOpened,
    #[serde(rename = "setup_intent.canceled")]
    SetupIntentCanceled,
    #[serde(rename = "setup_intent.created")]
    SetupIntentCreated,
    #[serde(rename = "setup_intent.requires_action")]
    SetupIntentRequiresAction,
    #[serde(rename = "setup_intent.setup_failed")]
    SetupIntentSetupFailed,
    #[serde(rename = "setup_intent.succeeded")]
    SetupIntentSucceeded,
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
    SourceChargeable,
    #[serde(rename = "source.failed")]
    SourceFailed,
    #[serde(rename = "source.mandate_notification")]
    SourceMandateNotification,
    #[serde(rename = "source.refund_attributes_required")]
    SourceRefundAttributesRequired,
    #[serde(rename = "source.transaction.created")]
    SourceTransactionCreated,
    #[serde(rename = "source.transaction.updated")]
    SourceTransactionUpdated,
    #[serde(rename = "subscription_schedule.aborted")]
    SubscriptionScheduleAborted,
    #[serde(rename = "subscription_schedule.canceled")]
    SubscriptionScheduleCanceled,
    #[serde(rename = "subscription_schedule.completed")]
    SubscriptionScheduleCompleted,
    #[serde(rename = "subscription_schedule.created")]
    SubscriptionScheduleCreated,
    #[serde(rename = "subscription_schedule.expiring")]
    SubscriptionScheduleExpiring,
    #[serde(rename = "subscription_schedule.released")]
    SubscriptionScheduleReleased,
    #[serde(rename = "subscription_schedule.updated")]
    SubscriptionScheduleUpdated,
    #[serde(rename = "tax_rate.created")]
    TaxRateCreated,
    #[serde(rename = "tax_rate.updated")]
    TaxRateUpdated,
    #[serde(rename = "terminal.reader.action_failed")]
    TerminalReaderActionFailed,
    #[serde(rename = "terminal.reader.action_succeeded")]
    TerminalReaderActionSucceeded,
    #[serde(rename = "test_helpers.test_clock.advancing")]
    TestHelpersTestClockAdvancing,
    #[serde(rename = "test_helpers.test_clock.created")]
    TestHelpersTestClockCreated,
    #[serde(rename = "test_helpers.test_clock.deleted")]
    TestHelpersTestClockDeleted,
    #[serde(rename = "test_helpers.test_clock.internal_failure")]
    TestHelpersTestClockInternalFailure,
    #[serde(rename = "test_helpers.test_clock.ready")]
    TestHelpersTestClockReady,
    #[serde(rename = "topup.canceled")]
    TopupCanceled,
    #[serde(rename = "topup.created")]
    TopupCreated,
    #[serde(rename = "topup.failed")]
    TopupFailed,
    #[serde(rename = "topup.reversed")]
    TopupReversed,
    #[serde(rename = "topup.succeeded")]
    TopupSucceeded,
    #[serde(rename = "transfer.created")]
    TransferCreated,
    #[serde(rename = "transfer.failed")]
    TransferFailed,
    #[serde(rename = "transfer.paid")]
    TransferPaid,
    #[serde(rename = "transfer.reversed")]
    TransferReversed,
    #[serde(rename = "transfer.updated")]
    TransferUpdated,
    #[serde(other)]
    #[default]
    Unknown,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

/// The resource representing a Stripe "NotificationEventData".
///
/// note: this is a manual override of the generated code;
///       see notification_event_data.rs for the (broken) codegen
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct NotificationEventData {
    pub object: EventObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<HashMap<String, Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum EventObject {
    Account(Account),
    #[serde(rename = "capability")]
    AccountCapabilities(AccountCapabilities),
    Application(Application),
    ApplicationFee(ApplicationFee),
    #[serde(rename = "fee_refund")]
    ApplicationFeeRefund(ApplicationFeeRefund),
    Balance(Balance),
    BankAccount(BankAccount),
    #[serde(rename = "billing_portal.configuration")]
    BillingPortalConfiguration(BillingPortalConfiguration),
    Card(Card),
    Charge(Charge),
    #[serde(rename = "checkout.session")]
    CheckoutSession(CheckoutSession),
    Coupon(Coupon),
    Customer(Customer),
    Discount(Discount),
    Dispute(Dispute),
    File(File),
    Invoice(Invoice),
    #[serde(rename = "invoiceitem")]
    InvoiceItem(InvoiceItem),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(IssuingAuthorization),
    #[serde(rename = "issuing.card")]
    IssuingCard(IssuingCard),
    #[serde(rename = "issuing.cardholder")]
    IssuingCardholder(IssuingCardholder),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(IssuingTransaction),
    Mandate(Mandate),
    PaymentIntent(PaymentIntent),
    PaymentLink(PaymentLink),
    PaymentMethod(PaymentMethod),
    Payout(Payout),
    Person(Person),
    Plan(Plan),
    Price(Price),
    Product(Product),
    PromotionCode(PromotionCode),
    Quote(Quote),
    Refund(Refund),
    Review(Review),
    SetupIntent(SetupIntent),
    Subscription(Subscription),
    SubscriptionSchedule(SubscriptionSchedule),
    TaxId(TaxId),
    TaxRate(TaxRate),
    #[serde(rename = "test_helpers.test_clock")]
    TestHelpersTestClock(TestHelpersTestClock),
    Topup(Topup),
    Transfer(Transfer),
}

impl Default for EventObject {
    fn default() -> Self {
        EventObject::Account(Account::default())
    }
}

#[cfg(feature = "webhook-events")]
pub struct Webhook {
    current_timestamp: i64,
}

#[cfg(feature = "webhook-events")]
impl Webhook {
    /// Construct an event from a webhook payload and signature.
    ///
    /// # Errors
    ///
    /// This function will return a WebhookError if:
    ///  - the provided signature is invalid
    ///  - the provided secret is invalid
    ///  - the signature timestamp is older than 5 minutes
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

        // Compute HMAC with the SHA256 hash function, using endpoing secret as key
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

        Ok(serde_json::from_str(payload)?)
    }
}

#[cfg(feature = "webhook-events")]
#[derive(Debug)]
struct Signature<'r> {
    t: i64,
    v1: &'r str,
}

#[cfg(feature = "webhook-events")]
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
    #[cfg(feature = "webhook-events")]
    #[test]
    fn test_event_type_display() {
        use super::EventType;

        // Can catch issues like quotes being added to the string
        assert_eq!(
            EventType::CustomerSubscriptionCreated.to_string(),
            "customer.subscription.created"
        );
    }

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

        let raw_signature_with_test_mode = "t=1492774577,v1=5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd,v0=6ffbb59b2300aae63f272406069a9788598b792a944a07aba816edb039989a39";
        let signature = Signature::parse(raw_signature_with_test_mode).unwrap();
        assert_eq!(signature.t, 1492774577);
        assert_eq!(
            signature.v1,
            "5257a869e7ecebeda32affa62cdca3fa51cad7e77a0e56ff536d0ce8e108d8bd"
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

        let webhook = super::Webhook { current_timestamp: event_timestamp };

        let event = webhook
            .do_construct_event(payload, &signature, &secret)
            .expect("Failed to construct event");

        assert_eq!(event.type_, super::EventType::InvoiceItemCreated);
        assert_eq!(event.id, "evt_123".parse::<crate::EventId>().unwrap());
        assert_eq!(event.account, "acct_123".parse().ok());
        assert_eq!(event.created, 1533204620);
    }
}
