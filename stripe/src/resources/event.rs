use crate::error::WebhookError;
use crate::resources::*;
use chrono::Utc;
#[cfg(feature = "webhooks")]
use hmac::{Hmac, Mac};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "webhooks")]
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
    #[serde(rename = "invoice.payment_failed")]
    InvoicePaymentFailed,
    #[serde(rename = "invoice.payment_succeeded")]
    InvoicePaymentSucceeded,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
    #[serde(rename = "invoice.upcoming")]
    InvoiceUpcoming,
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
    Dispute(Dispute),
    File(File),
    Invoice(Invoice),
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
    Transaction(Transaction),
    Transfer(Transfer),
}

#[cfg(feature = "webhooks")]
pub struct Webhook {}

#[cfg(feature = "webhooks")]
impl Webhook {
    pub fn construct_event(
        payload: String,
        sig: String,
        secret: String,
    ) -> Result<Event, WebhookError> {
        let headers: Vec<String> = sig.split(',').map(|s| s.trim().to_string()).collect();

        // Prepare the signed payload
        let timestamp: Vec<String> = headers[0].split('=').map(|s| s.to_string()).collect();
        let signed_payload = format!("{}{}{}", timestamp[1], ".", payload);

        // Get Stripe signature from header
        let signature: Vec<String> = headers[1].split('=').map(|s| s.to_string()).collect();

        // Compute HMAC with the SHA256 hash function, using endpoing secret as key
        // and signed_payload string as the message.
        let mut mac =
            Hmac::<Sha256>::new_varkey(secret.as_bytes()).map_err(|_| WebhookError::BadKey)?;
        mac.input(signed_payload.as_bytes());
        if !mac.result().is_equal(signature[1].as_bytes()) {
            return Err(WebhookError::BadSignature);
        }

        // Get current timestamp to compare to signature timestamp
        let current = Utc::now().timestamp();
        let num_timestamp = timestamp[1].parse::<i64>().map_err(WebhookError::BadHeader)?;
        if current - num_timestamp > 300 {
            return Err(WebhookError::BadTimestamp(num_timestamp));
        }

        serde_json::from_str(&payload).map_err(WebhookError::BadParse)
    }
}
