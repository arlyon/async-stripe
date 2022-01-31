use serde_derive::{Deserialize, Serialize};

use crate::ids::*;
use crate::params::Object;

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: AccountId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    pub id: (),
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFee {
    pub id: ApplicationFeeId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "checkout"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSession {
    pub id: CheckoutSessionId,
}

#[cfg(not(feature = "checkout"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectCollectionTransfer {
    pub id: (),
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coupon {
    pub id: CouponId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Discount {
    pub id: DiscountId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFeeRefund {
    pub id: ApplicationFeeRefundId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub id: InvoiceId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceItem {
    pub id: InvoiceItemId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorization {
    pub id: IssuingAuthorizationId,
}

#[cfg(not(feature = "issuing"))]


#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCard {
    pub id: IssuingCardId,
}

#[cfg(not(feature = "issuing"))]


#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholder {
    pub id: IssuingCardholderId,
}

#[cfg(not(feature = "issuing"))]


#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDispute {
    pub id: IssuingDisputeId,
}

#[cfg(not(feature = "issuing"))]


#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransaction {
    pub id: IssuingTransactionId,
}

#[cfg(not(feature = "issuing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    pub id: InvoiceLineItemId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: OrderId,
}

#[cfg(not(feature = "orders"))]


#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderItem {
    pub id: (),
}

#[cfg(not(feature = "orders"))]


#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderReturn {
    pub id: OrderReturnId,
}

#[cfg(not(feature = "orders"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: PersonId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plan {
    pub id: PlanId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Recipient {
    pub id: RecipientId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "fraud"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Review {
    pub id: ReviewId,
}

#[cfg(not(feature = "fraud"))]


#[cfg(not(feature = "sigma"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScheduledQueryRun {
    pub id: ScheduledQueryRunId,
}

#[cfg(not(feature = "sigma"))]


#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sku {
    pub id: SkuId,
}

#[cfg(not(feature = "orders"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub id: SubscriptionId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItem {
    pub id: SubscriptionItemId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    pub id: SubscriptionScheduleId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleRevision {
    pub id: (),
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxId {
    pub id: TaxIdId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxRate {
    pub id: TaxRateId,
}

#[cfg(not(feature = "billing"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Topup {
    pub id: TopupId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transfer {
    pub id: TransferId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferReversal {
    pub id: TransferReversalId,
}

#[cfg(not(feature = "connect"))]


#[cfg(not(feature = "webhook-endpoints"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookEndpoint {
    pub id: WebhookEndpointId,
}

#[cfg(not(feature = "webhook-endpoints"))]
impl Object for WebhookEndpoint {
    type Id = WebhookEndpointId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "webhook_endpoint"
    }
}

impl Object for TransferReversal {
    type Id = TransferReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "transfer_reversal"
    }
}

impl Object for OrderItem {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "order_item"
    }
}

impl Object for CheckoutSession {
    type Id = CheckoutSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "checkout_session"
    }
}

impl Object for Application {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "application"
    }
}

impl Object for Account {
    type Id = AccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "account"
    }
}

impl Object for ConnectCollectionTransfer {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "connect_collection_transfer"
    }
}

impl Object for Subscription {
    type Id = SubscriptionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription"
    }
}

impl Object for Coupon {
    type Id = CouponId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "coupon"
    }
}

impl Object for ApplicationFee {
    type Id = ApplicationFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "application_fee"
    }
}

impl Object for InvoiceLineItem {
    type Id = InvoiceLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "line_item"
    }
}

impl Object for SubscriptionSchedule {
    type Id = SubscriptionScheduleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_schedule"
    }
}

impl Object for IssuingCard {
    type Id = IssuingCardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.card"
    }
}

impl Object for SubscriptionItem {
    type Id = SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_item"
    }
}

impl Object for SubscriptionScheduleRevision {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "subscription_schedule_revision"
    }
}

impl Object for Transfer {
    type Id = TransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "transfer"
    }
}

impl Object for IssuingCardholder {
    type Id = IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.cardholder"
    }
}

impl Object for TaxRate {
    type Id = TaxRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_rate"
    }
}

impl Object for Topup {
    type Id = TopupId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "topup"
    }
}

impl Object for IssuingTransaction {
    type Id = IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.transaction"
    }
}

impl Object for ScheduledQueryRun {
    type Id = ScheduledQueryRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "scheduled_query_run"
    }
}

impl Object for Invoice {
    type Id = InvoiceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoice"
    }
}

impl Object for Recipient {
    type Id = RecipientId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "recipient"
    }
}

impl Object for InvoiceItem {
    type Id = InvoiceItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoiceitem"
    }
}

impl Object for Sku {
    type Id = SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "sku"
    }
}

impl Object for Discount {
    type Id = DiscountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "discount"
    }
}

impl Object for Review {
    type Id = ReviewId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "review"
    }
}

impl Object for Plan {
    type Id = PlanId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "plan"
    }
}

impl Object for TaxId {
    type Id = TaxIdId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_id"
    }
}

impl Object for Order {
    type Id = OrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order"
    }
}

impl Object for OrderReturn {
    type Id = OrderReturnId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order_return"
    }
}

impl Object for ApplicationFeeRefund {
    type Id = ApplicationFeeRefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "fee_refund"
    }
}

impl Object for IssuingAuthorization {
    type Id = IssuingAuthorizationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.authorization"
    }
}

impl Object for Person {
    type Id = PersonId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "person"
    }
}

impl Object for IssuingDispute {
    type Id = IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.dispute"
    }
}
