use serde_derive::{Deserialize, Serialize};

use crate::ids::*;
use crate::params::Object;

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: AccountId,
}
//automatically added back in service of Account with hash3592802899155702435
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    pub id: (),
}
//automatically added back in service of Application with hash-7924135171204768363
#[cfg(not(feature = "connect"))]
impl Object for Application {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "application"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFee {
    pub id: ApplicationFeeId,
}
//automatically added back in service of ApplicationFee with hash6166226494024039131
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSession {
    pub id: CheckoutSessionId,
}
//automatically added back in service of CheckoutSession with hash1064398556010491948
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectCollectionTransfer {
    pub id: (),
}
//automatically added back in service of Transfer with hash-946360196327450892
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

//automatically added back in service of ConnectCollectionTransfer with hash5040508490302329541
#[cfg(not(feature = "connect"))]
impl Object for ConnectCollectionTransfer {
    type Id = ();
    fn id(&self) -> Self::Id {
        self.id
    }
    fn object(&self) -> &'static str {
        "connect_collection_transfer"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coupon {
    pub id: CouponId,
}
//automatically added back in service of Coupon with hash5062855632676062145
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Discount {
    pub id: DiscountId,
}
//automatically added back in service of Discount with hash-2175254495179855576
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFeeRefund {
    pub id: ApplicationFeeRefundId,
}
//automatically added back in service of ApplicationFeeRefund with hash4715153609552166390
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub id: InvoiceId,
}
//automatically added back in service of Invoice with hash2686597225083832559
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceItem {
    pub id: InvoiceItemId,
}
//automatically added back in service of InvoiceItem with hash595189701795417752
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorization {
    pub id: IssuingAuthorizationId,
}
//automatically added back in service of IssuingAuthorization with hash5455333282758663823
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCard {
    pub id: IssuingCardId,
}
//automatically added back in service of IssuingCard with hash-2725050037589636419
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholder {
    pub id: IssuingCardholderId,
}
//automatically added back in service of IssuingCardholder with hash-3281164070190290591
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDispute {
    pub id: IssuingDisputeId,
}
//automatically added back in service of IssuingDispute with hash-8021036917407596782
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransaction {
    pub id: IssuingTransactionId,
}
//automatically added back in service of IssuingTransaction with hash-6340634987581238962
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    pub id: InvoiceLineItemId,
}
//automatically added back in service of InvoiceLineItem with hash7700006293702222346
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: OrderId,
}
//automatically added back in service of Order with hash6154026638304160162
#[cfg(not(feature = "orders"))]
impl Object for Order {
    type Id = OrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order"
    }
}

#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderItem {
    pub id: (),
}
//automatically added back in service of OrderItem with hash3895749257816968079
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderReturn {
    pub id: OrderReturnId,
}
//automatically added back in service of OrderReturn with hash6629718212312249848
#[cfg(not(feature = "orders"))]
impl Object for OrderReturn {
    type Id = OrderReturnId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order_return"
    }
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: PersonId,
}
//automatically added back in service of Person with hash5620762629573234696
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plan {
    pub id: PlanId,
}
//automatically added back in service of Plan with hash3390798107851349328
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

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Recipient {
    pub id: RecipientId,
}
//automatically added back in service of Recipient with hash1817394076928278497
#[cfg(not(feature = "connect"))]
impl Object for Recipient {
    type Id = RecipientId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "recipient"
    }
}

#[cfg(not(feature = "fraud"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Review {
    pub id: ReviewId,
}
//automatically added back in service of Review with hash-6534407800379077321
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScheduledQueryRun {
    pub id: ScheduledQueryRunId,
}
//automatically added back in service of ScheduledQueryRun with hash5690276177249760270
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sku {
    pub id: SkuId,
}
//automatically added back in service of Sku with hash-7355070974575702771
#[cfg(not(feature = "orders"))]
impl Object for Sku {
    type Id = SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "sku"
    }
}

#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub id: SubscriptionId,
}
//automatically added back in service of Subscription with hash-6326617233248894842
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItem {
    pub id: SubscriptionItemId,
}
//automatically added back in service of SubscriptionItem with hash1114318409582002905
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    pub id: SubscriptionScheduleId,
}
//automatically added back in service of SubscriptionSchedule with hash6075648035649748343
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleRevision {
    pub id: (),
}
//automatically added back in service of SubscriptionScheduleRevision with hash7937448135755504067
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxId {
    pub id: TaxIdId,
}
//automatically added back in service of TaxId with hash4016838953470791134
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxRate {
    pub id: TaxRateId,
}
//automatically added back in service of TaxRate with hash-7355842402776281241
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Topup {
    pub id: TopupId,
}
//automatically added back in service of Topup with hash9015737665482534784
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transfer {
    pub id: TransferId,
}

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferReversal {
    pub id: TransferReversalId,
}
//automatically added back in service of TransferReversal with hash-3912079485920673044
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
