use crate::ids::*;
use crate::params::Object;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Account {
    pub id: AccountId,
}

#[cfg(not(feature = "connect"))]
impl Object for Account {
    type Id = AccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "account"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Application {
    pub id: ApplicationId,
}

#[cfg(not(feature = "connect"))]
impl Object for Application {
    type Id = ApplicationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "application"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationFee {
    pub id: ApplicationFeeId,
}

#[cfg(not(feature = "connect"))]
impl Object for ApplicationFee {
    type Id = ApplicationFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "application_fee"
    }
}

#[cfg(not(feature = "checkout"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSession {
    pub id: CheckoutSessionId,
}

#[cfg(not(feature = "checkout"))]
impl Object for CheckoutSession {
    type Id = CheckoutSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "checkout_session"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectCollectionTransfer {
    pub id: ConnectCollectionTransferId,
}

#[cfg(not(feature = "connect"))]
impl Object for ConnectCollectionTransfer {
    type Id = ConnectCollectionTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "connect_collection_transfer"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Coupon {
    pub id: CouponId,
}

#[cfg(not(feature = "billing"))]
impl Object for Coupon {
    type Id = CouponId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "coupon"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Discount {
    pub id: DiscountId,
}

#[cfg(not(feature = "billing"))]
impl Object for Discount {
    type Id = DiscountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "discount"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationFeeRefund {
    pub id: ApplicationFeeRefundId,
}

#[cfg(not(feature = "connect"))]
impl Object for ApplicationFeeRefund {
    type Id = ApplicationFeeRefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "fee_refund"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Invoice {
    pub id: InvoiceId,
}

#[cfg(not(feature = "billing"))]
impl Object for Invoice {
    type Id = InvoiceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoice"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceItem {
    pub id: InvoiceItemId,
}

#[cfg(not(feature = "billing"))]
impl Object for InvoiceItem {
    type Id = InvoiceItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoiceitem"
    }
}

#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorization {
    pub id: IssuingAuthorizationId,
}

#[cfg(not(feature = "issuing"))]
impl Object for IssuingAuthorization {
    type Id = IssuingAuthorizationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.authorization"
    }
}

#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCard {
    pub id: IssuingCardId,
}

#[cfg(not(feature = "issuing"))]
impl Object for IssuingCard {
    type Id = IssuingCardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.card"
    }
}

#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholder {
    pub id: IssuingCardholderId,
}

#[cfg(not(feature = "issuing"))]
impl Object for IssuingCardholder {
    type Id = IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.cardholder"
    }
}

#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingDispute {
    pub id: IssuingDisputeId,
}

#[cfg(not(feature = "issuing"))]
impl Object for IssuingDispute {
    type Id = IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.dispute"
    }
}

#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransaction {
    pub id: IssuingTransactionId,
}

#[cfg(not(feature = "issuing"))]
impl Object for IssuingTransaction {
    type Id = IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.transaction"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    pub id: InvoiceLineItemId,
}

#[cfg(not(feature = "billing"))]
impl Object for InvoiceLineItem {
    type Id = InvoiceLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "line_item"
    }
}

#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Order {
    pub id: (),
}

#[cfg(not(feature = "orders"))]
impl Object for Order {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "order"
    }
}

#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderItem {
    pub id: (),
}

#[cfg(not(feature = "orders"))]
impl Object for OrderItem {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "order_item"
    }
}

#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderReturn {
    pub id: (),
}

#[cfg(not(feature = "orders"))]
impl Object for OrderReturn {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "order_return"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Person {
    pub id: PersonId,
}

#[cfg(not(feature = "connect"))]
impl Object for Person {
    type Id = PersonId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "person"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Plan {
    pub id: PlanId,
}

#[cfg(not(feature = "billing"))]
impl Object for Plan {
    type Id = PlanId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "plan"
    }
}

#[cfg(not(feature = "fraud"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Review {
    pub id: ReviewId,
}

#[cfg(not(feature = "fraud"))]
impl Object for Review {
    type Id = ReviewId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "review"
    }
}

#[cfg(not(feature = "sigma"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ScheduledQueryRun {
    pub id: ScheduledQueryRunId,
}

#[cfg(not(feature = "sigma"))]
impl Object for ScheduledQueryRun {
    type Id = ScheduledQueryRunId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "scheduled_query_run"
    }
}

#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Sku {
    pub id: (),
}

#[cfg(not(feature = "orders"))]
impl Object for Sku {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "sku"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Subscription {
    pub id: SubscriptionId,
}

#[cfg(not(feature = "billing"))]
impl Object for Subscription {
    type Id = SubscriptionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItem {
    pub id: SubscriptionItemId,
}

#[cfg(not(feature = "billing"))]
impl Object for SubscriptionItem {
    type Id = SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_item"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    pub id: SubscriptionScheduleId,
}

#[cfg(not(feature = "billing"))]
impl Object for SubscriptionSchedule {
    type Id = SubscriptionScheduleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_schedule"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionScheduleRevision {
    pub id: (),
}

#[cfg(not(feature = "billing"))]
impl Object for SubscriptionScheduleRevision {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "subscription_schedule_revision"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxId {
    pub id: TaxIdId,
}

#[cfg(not(feature = "billing"))]
impl Object for TaxId {
    type Id = TaxIdId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_id"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxRate {
    pub id: TaxRateId,
}

#[cfg(not(feature = "billing"))]
impl Object for TaxRate {
    type Id = TaxRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_rate"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Topup {
    pub id: TopupId,
}

#[cfg(not(feature = "connect"))]
impl Object for Topup {
    type Id = TopupId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "topup"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Transfer {
    pub id: TransferId,
}

#[cfg(not(feature = "connect"))]
impl Object for Transfer {
    type Id = TransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "transfer"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransferReversal {
    pub id: TransferReversalId,
}

#[cfg(not(feature = "connect"))]
impl Object for TransferReversal {
    type Id = TransferReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "transfer_reversal"
    }
}

#[cfg(not(feature = "webhook-endpoints"))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
