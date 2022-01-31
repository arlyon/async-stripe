use serde_derive::{Deserialize, Serialize};

use crate::ids::*;
use crate::params::Object;

#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: AccountId,
}



#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    pub id: (),
}



#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFee {
    pub id: ApplicationFeeId,
}



#[cfg(not(feature = "checkout"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSession {
    pub id: CheckoutSessionId,
}



#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectCollectionTransfer {
    pub id: (),
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coupon {
    pub id: CouponId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Discount {
    pub id: DiscountId,
}



#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationFeeRefund {
    pub id: ApplicationFeeRefundId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub id: InvoiceId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceItem {
    pub id: InvoiceItemId,
}



#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorization {
    pub id: IssuingAuthorizationId,
}



#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCard {
    pub id: IssuingCardId,
}



#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholder {
    pub id: IssuingCardholderId,
}



#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDispute {
    pub id: IssuingDisputeId,
}



#[cfg(not(feature = "issuing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransaction {
    pub id: IssuingTransactionId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    pub id: InvoiceLineItemId,
}



#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: OrderId,
}



#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderItem {
    pub id: (),
}



#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderReturn {
    pub id: OrderReturnId,
}



#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: PersonId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plan {
    pub id: PlanId,
}



#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Recipient {
    pub id: RecipientId,
}



#[cfg(not(feature = "fraud"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Review {
    pub id: ReviewId,
}



#[cfg(not(feature = "sigma"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScheduledQueryRun {
    pub id: ScheduledQueryRunId,
}



#[cfg(not(feature = "orders"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sku {
    pub id: SkuId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub id: SubscriptionId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItem {
    pub id: SubscriptionItemId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    pub id: SubscriptionScheduleId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleRevision {
    pub id: (),
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxId {
    pub id: TaxIdId,
}



#[cfg(not(feature = "billing"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxRate {
    pub id: TaxRateId,
}



#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Topup {
    pub id: TopupId,
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

//automatically added back in service of TransferReversal with hash5771848075456844946
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

//automatically added back in service of OrderItem with hash3704032167982539101
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

//automatically added back in service of CheckoutSession with hash-8809814167742878604
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

//automatically added back in service of Application with hash-6459248839763565641
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

//automatically added back in service of Account with hash2972867375275022447
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

//automatically added back in service of ConnectCollectionTransfer with hash-5340590057807454939
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

//automatically added back in service of Subscription with hash6349076382051314398
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

//automatically added back in service of Coupon with hash8592416896964471419
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

//automatically added back in service of ApplicationFee with hash727360216826743751
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

//automatically added back in service of InvoiceLineItem with hash2307922612136106088
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

//automatically added back in service of SubscriptionSchedule with hash-8605940895179834805
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

//automatically added back in service of IssuingCard with hash4501721549196130287
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

//automatically added back in service of SubscriptionItem with hash6670535476668204121
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

//automatically added back in service of SubscriptionScheduleRevision with hash675248495282515059
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

//automatically added back in service of Transfer with hash1469934724607901288
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

//automatically added back in service of IssuingCardholder with hash3919303576008855339
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

//automatically added back in service of TaxRate with hash5859477794856546109
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

//automatically added back in service of Topup with hash-1055834901306071266
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

//automatically added back in service of IssuingTransaction with hash2453258017434230428
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

//automatically added back in service of ScheduledQueryRun with hash-768470941936991008
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

//automatically added back in service of Invoice with hash-1480097582396266853
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

//automatically added back in service of Recipient with hash-2353419236078842127
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

//automatically added back in service of InvoiceItem with hash7950177059610030066
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

//automatically added back in service of Sku with hash-2970364133945343515
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

//automatically added back in service of Discount with hash610163505819025392
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

//automatically added back in service of Review with hash2910031241930626001
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

//automatically added back in service of Plan with hash1272108170975119476
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

//automatically added back in service of TaxId with hash992046258278666322
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

//automatically added back in service of Order with hash-7707031304986798316
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

//automatically added back in service of OrderReturn with hash-3373669299755781116
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

//automatically added back in service of ApplicationFeeRefund with hash-5884094980258879102
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

//automatically added back in service of IssuingAuthorization with hash-5141201543021633229
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

//automatically added back in service of Person with hash3437362498376385340
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

//automatically added back in service of IssuingDispute with hash5967689277921946028
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
