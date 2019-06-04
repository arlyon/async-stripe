// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::WebhookEndpointId;
use crate::params::{Deleted, Expand, List, Object, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "NotificationWebhookEndpoint".
///
/// For more details see [https://stripe.com/docs/api/webhook_endpoints/object](https://stripe.com/docs/api/webhook_endpoints/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookEndpoint {
    /// Unique identifier for the object.
    pub id: WebhookEndpointId,

    /// The API version events are rendered as for this webhook endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<ApiVersion>,

    /// The ID of the associated Connect application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The list of events to enable for this endpoint.
    ///
    /// You may specify `['*']` to enable all events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_events: Option<Vec<EventFilter>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// The endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures).
    ///
    /// Only returned at creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// The status of the webhook.
    ///
    /// It can be `enabled` or `disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WebhookEndpointStatus>,

    /// The URL of the webhook endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WebhookEndpoint {
    /// Returns a list of your webhook endpoints.
    pub fn list(
        client: &Client,
        params: ListWebhookEndpoints<'_>,
    ) -> Response<List<WebhookEndpoint>> {
        client.get_query("/webhook_endpoints", &params)
    }

    /// A webhook endpoint must have a `url` and a list of `enabled_events`.
    ///
    /// You may optionally specify the Boolean `connect` parameter.
    /// If set to true, then a Connect webhook endpoint that notifies the specified `url` about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified `url` only about events from your account is created.
    /// You can also create webhook endpoints in the [webhooks settings](https://dashboard.stripe.com/account/webhooks) section of the Dashboard.
    pub fn create(client: &Client, params: CreateWebhookEndpoint<'_>) -> Response<WebhookEndpoint> {
        client.post_form("/webhook_endpoints", &params)
    }

    /// Retrieves the webhook endpoint with the given ID.
    pub fn retrieve(
        client: &Client,
        id: &WebhookEndpointId,
        expand: &[&str],
    ) -> Response<WebhookEndpoint> {
        client.get_query(&format!("/webhook_endpoints/{}", id), &Expand { expand })
    }

    /// Updates the webhook endpoint.
    ///
    /// You may edit the `url`, the list of `enabled_events`, and the status of your endpoint.
    pub fn update(
        client: &Client,
        id: &WebhookEndpointId,
        params: UpdateWebhookEndpoint<'_>,
    ) -> Response<WebhookEndpoint> {
        client.post_form(&format!("/webhook_endpoints/{}", id), &params)
    }

    /// You can also delete webhook endpoints via the [webhook endpoint management](https://dashboard.stripe.com/account/webhooks) page of the Stripe dashboard.
    pub fn delete(client: &Client, id: &WebhookEndpointId) -> Response<Deleted<WebhookEndpointId>> {
        client.delete(&format!("/webhook_endpoints/{}", id))
    }
}

impl Object for WebhookEndpoint {
    type Id = WebhookEndpointId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "webhook_endpoint"
    }
}

/// The parameters for `WebhookEndpoint::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateWebhookEndpoint<'a> {
    /// Events sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<ApiVersion>,

    /// Whether this endpoint should receive events from connected accounts (`true`), or your account (`false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<bool>,
    pub enabled_events: Vec<EventFilter>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The URL of the webhook endpoint.
    pub url: &'a str,
}

impl<'a> CreateWebhookEndpoint<'a> {
    pub fn new(enabled_events: Vec<EventFilter>, url: &'a str) -> Self {
        CreateWebhookEndpoint {
            api_version: Default::default(),
            connect: Default::default(),
            enabled_events,
            expand: Default::default(),
            url,
        }
    }
}

/// The parameters for `WebhookEndpoint::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListWebhookEndpoints<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a WebhookEndpointId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<WebhookEndpointId>,
}

impl<'a> ListWebhookEndpoints<'a> {
    pub fn new() -> Self {
        ListWebhookEndpoints {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `WebhookEndpoint::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateWebhookEndpoint<'a> {
    /// Disable the webhook endpoint if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_events: Option<Vec<EventFilter>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The URL of the webhook endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}

impl<'a> UpdateWebhookEndpoint<'a> {
    pub fn new() -> Self {
        UpdateWebhookEndpoint {
            disabled: Default::default(),
            enabled_events: Default::default(),
            expand: Default::default(),
            url: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `WebhookEndpoint`'s `api_version` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiVersion {
    #[serde(rename = "2011-01-01")]
    V2011_01_01,
    #[serde(rename = "2011-06-21")]
    V2011_06_21,
    #[serde(rename = "2011-06-28")]
    V2011_06_28,
    #[serde(rename = "2011-08-01")]
    V2011_08_01,
    #[serde(rename = "2011-09-15")]
    V2011_09_15,
    #[serde(rename = "2011-11-17")]
    V2011_11_17,
    #[serde(rename = "2012-02-23")]
    V2012_02_23,
    #[serde(rename = "2012-03-25")]
    V2012_03_25,
    #[serde(rename = "2012-06-18")]
    V2012_06_18,
    #[serde(rename = "2012-06-28")]
    V2012_06_28,
    #[serde(rename = "2012-07-09")]
    V2012_07_09,
    #[serde(rename = "2012-09-24")]
    V2012_09_24,
    #[serde(rename = "2012-10-26")]
    V2012_10_26,
    #[serde(rename = "2012-11-07")]
    V2012_11_07,
    #[serde(rename = "2013-02-11")]
    V2013_02_11,
    #[serde(rename = "2013-02-13")]
    V2013_02_13,
    #[serde(rename = "2013-07-05")]
    V2013_07_05,
    #[serde(rename = "2013-08-12")]
    V2013_08_12,
    #[serde(rename = "2013-08-13")]
    V2013_08_13,
    #[serde(rename = "2013-10-29")]
    V2013_10_29,
    #[serde(rename = "2013-12-03")]
    V2013_12_03,
    #[serde(rename = "2014-01-31")]
    V2014_01_31,
    #[serde(rename = "2014-03-13")]
    V2014_03_13,
    #[serde(rename = "2014-03-28")]
    V2014_03_28,
    #[serde(rename = "2014-05-19")]
    V2014_05_19,
    #[serde(rename = "2014-06-13")]
    V2014_06_13,
    #[serde(rename = "2014-06-17")]
    V2014_06_17,
    #[serde(rename = "2014-07-22")]
    V2014_07_22,
    #[serde(rename = "2014-07-26")]
    V2014_07_26,
    #[serde(rename = "2014-08-04")]
    V2014_08_04,
    #[serde(rename = "2014-08-20")]
    V2014_08_20,
    #[serde(rename = "2014-09-08")]
    V2014_09_08,
    #[serde(rename = "2014-10-07")]
    V2014_10_07,
    #[serde(rename = "2014-11-05")]
    V2014_11_05,
    #[serde(rename = "2014-11-20")]
    V2014_11_20,
    #[serde(rename = "2014-12-08")]
    V2014_12_08,
    #[serde(rename = "2014-12-17")]
    V2014_12_17,
    #[serde(rename = "2014-12-22")]
    V2014_12_22,
    #[serde(rename = "2015-01-11")]
    V2015_01_11,
    #[serde(rename = "2015-01-26")]
    V2015_01_26,
    #[serde(rename = "2015-02-10")]
    V2015_02_10,
    #[serde(rename = "2015-02-16")]
    V2015_02_16,
    #[serde(rename = "2015-02-18")]
    V2015_02_18,
    #[serde(rename = "2015-03-24")]
    V2015_03_24,
    #[serde(rename = "2015-04-07")]
    V2015_04_07,
    #[serde(rename = "2015-06-15")]
    V2015_06_15,
    #[serde(rename = "2015-07-07")]
    V2015_07_07,
    #[serde(rename = "2015-07-13")]
    V2015_07_13,
    #[serde(rename = "2015-07-28")]
    V2015_07_28,
    #[serde(rename = "2015-08-07")]
    V2015_08_07,
    #[serde(rename = "2015-08-19")]
    V2015_08_19,
    #[serde(rename = "2015-09-03")]
    V2015_09_03,
    #[serde(rename = "2015-09-08")]
    V2015_09_08,
    #[serde(rename = "2015-09-23")]
    V2015_09_23,
    #[serde(rename = "2015-10-01")]
    V2015_10_01,
    #[serde(rename = "2015-10-12")]
    V2015_10_12,
    #[serde(rename = "2015-10-16")]
    V2015_10_16,
    #[serde(rename = "2016-02-03")]
    V2016_02_03,
    #[serde(rename = "2016-02-19")]
    V2016_02_19,
    #[serde(rename = "2016-02-22")]
    V2016_02_22,
    #[serde(rename = "2016-02-23")]
    V2016_02_23,
    #[serde(rename = "2016-02-29")]
    V2016_02_29,
    #[serde(rename = "2016-03-07")]
    V2016_03_07,
    #[serde(rename = "2016-06-15")]
    V2016_06_15,
    #[serde(rename = "2016-07-06")]
    V2016_07_06,
    #[serde(rename = "2016-10-19")]
    V2016_10_19,
    #[serde(rename = "2017-01-27")]
    V2017_01_27,
    #[serde(rename = "2017-02-14")]
    V2017_02_14,
    #[serde(rename = "2017-04-06")]
    V2017_04_06,
    #[serde(rename = "2017-05-25")]
    V2017_05_25,
    #[serde(rename = "2017-06-05")]
    V2017_06_05,
    #[serde(rename = "2017-08-15")]
    V2017_08_15,
    #[serde(rename = "2017-12-14")]
    V2017_12_14,
    #[serde(rename = "2018-01-23")]
    V2018_01_23,
    #[serde(rename = "2018-02-05")]
    V2018_02_05,
    #[serde(rename = "2018-02-06")]
    V2018_02_06,
    #[serde(rename = "2018-02-28")]
    V2018_02_28,
    #[serde(rename = "2018-05-21")]
    V2018_05_21,
    #[serde(rename = "2018-07-27")]
    V2018_07_27,
    #[serde(rename = "2018-08-23")]
    V2018_08_23,
    #[serde(rename = "2018-09-06")]
    V2018_09_06,
    #[serde(rename = "2018-09-24")]
    V2018_09_24,
    #[serde(rename = "2018-10-31")]
    V2018_10_31,
    #[serde(rename = "2018-11-08")]
    V2018_11_08,
    #[serde(rename = "2019-02-11")]
    V2019_02_11,
    #[serde(rename = "2019-02-19")]
    V2019_02_19,
    #[serde(rename = "2019-03-14")]
    V2019_03_14,
}

/// An enum representing the possible values of an `WebhookEndpoint`'s `enabled_events` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventFilter {
    #[serde(rename = "*")]
    All,
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
    #[serde(rename = "checkout.session.completed")]
    CheckoutSessionCompleted,
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
    #[serde(rename = "invoice.upcoming")]
    InvoiceUpcoming,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
    #[serde(rename = "invoice.voided")]
    InvoiceVoided,
    #[serde(rename = "invoiceitem.created")]
    InvoiceitemCreated,
    #[serde(rename = "invoiceitem.deleted")]
    InvoiceitemDeleted,
    #[serde(rename = "invoiceitem.updated")]
    InvoiceitemUpdated,
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
    #[serde(rename = "issuing_dispute.created")]
    IssuingDisputeCreated,
    #[serde(rename = "issuing_dispute.updated")]
    IssuingDisputeUpdated,
    #[serde(rename = "issuing_settlement.created")]
    IssuingSettlementCreated,
    #[serde(rename = "issuing_settlement.updated")]
    IssuingSettlementUpdated,
    #[serde(rename = "issuing_transaction.created")]
    IssuingTransactionCreated,
    #[serde(rename = "issuing_transaction.updated")]
    IssuingTransactionUpdated,
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
    #[serde(rename = "payment_intent.amount_capturable_updated")]
    PaymentIntentAmountCapturableUpdated,
    #[serde(rename = "payment_intent.created")]
    PaymentIntentCreated,
    #[serde(rename = "payment_intent.payment_failed")]
    PaymentIntentPaymentFailed,
    #[serde(rename = "payment_intent.succeeded")]
    PaymentIntentSucceeded,
    #[serde(rename = "payment_method.attached")]
    PaymentMethodAttached,
    #[serde(rename = "payment_method.card_automatically_updated")]
    PaymentMethodCardAutomaticallyUpdated,
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
    #[serde(rename = "product.created")]
    ProductCreated,
    #[serde(rename = "product.deleted")]
    ProductDeleted,
    #[serde(rename = "product.updated")]
    ProductUpdated,
    #[serde(rename = "recipient.created")]
    RecipientCreated,
    #[serde(rename = "recipient.deleted")]
    RecipientDeleted,
    #[serde(rename = "recipient.updated")]
    RecipientUpdated,
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
    #[serde(rename = "tax_rate.created")]
    TaxRateCreated,
    #[serde(rename = "tax_rate.updated")]
    TaxRateUpdated,
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
}

/// An enum representing the possible values of an `WebhookEndpoint`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEndpointStatus {
    Disabled,
    Enabled,
}
