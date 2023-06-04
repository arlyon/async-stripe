use crate::{Client, Response};

impl crate::webhook_endpoint::WebhookEndpoint {
    /// Returns a list of your webhook endpoints.
    pub fn list(
        client: &Client,
        params: ListWebhookEndpoint,
    ) -> Response<crate::List<crate::webhook_endpoint::WebhookEndpoint>> {
        client.get_query("/webhook_endpoints", params)
    }
    /// Retrieves the webhook endpoint with the given ID.
    pub fn retrieve(
        client: &Client,
        webhook_endpoint: &crate::webhook_endpoint::WebhookEndpointId,
        params: RetrieveWebhookEndpoint,
    ) -> Response<crate::webhook_endpoint::WebhookEndpoint> {
        client.get_query(
            &format!("/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = webhook_endpoint),
            params,
        )
    }
    /// A webhook endpoint must have a `url` and a list of `enabled_events`.
    ///
    /// You may optionally specify the Boolean `connect` parameter.
    /// If set to true, then a Connect webhook endpoint that notifies the specified `url` about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified `url` only about events from your account is created.
    /// You can also create webhook endpoints in the [webhooks settings](https://dashboard.stripe.com/account/webhooks) section of the Dashboard.
    pub fn create(
        client: &Client,
        params: CreateWebhookEndpoint,
    ) -> Response<crate::webhook_endpoint::WebhookEndpoint> {
        client.send_form("/webhook_endpoints", params, http_types::Method::Post)
    }
    /// Updates the webhook endpoint.
    ///
    /// You may edit the `url`, the list of `enabled_events`, and the status of your endpoint.
    pub fn update(
        client: &Client,
        webhook_endpoint: &crate::webhook_endpoint::WebhookEndpointId,
        params: UpdateWebhookEndpoint,
    ) -> Response<crate::webhook_endpoint::WebhookEndpoint> {
        client.send_form(
            &format!("/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = webhook_endpoint),
            params,
            http_types::Method::Post,
        )
    }
    /// You can also delete webhook endpoints via the [webhook endpoint management](https://dashboard.stripe.com/account/webhooks) page of the Stripe dashboard.
    pub fn delete(
        client: &Client,
        webhook_endpoint: &crate::webhook_endpoint::WebhookEndpointId,
    ) -> Response<crate::webhook_endpoint::DeletedWebhookEndpoint> {
        client.send(
            &format!("/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = webhook_endpoint),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListWebhookEndpoint<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListWebhookEndpoint<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveWebhookEndpoint<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveWebhookEndpoint<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateWebhookEndpoint<'a> {
    /// Events sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<CreateWebhookEndpointApiVersion>,
    /// Whether this endpoint should receive events from connected accounts (`true`), or from your account (`false`).
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<bool>,
    /// An optional description of what the webhook is used for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The list of events to enable for this endpoint.
    ///
    /// You may specify `['*']` to enable all events, except those that require explicit selection.
    pub enabled_events: &'a [CreateWebhookEndpointEnabledEvents],
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The URL of the webhook endpoint.
    pub url: &'a str,
}
impl<'a> CreateWebhookEndpoint<'a> {
    pub fn new(enabled_events: &'a [CreateWebhookEndpointEnabledEvents], url: &'a str) -> Self {
        Self {
            api_version: Default::default(),
            connect: Default::default(),
            description: Default::default(),
            enabled_events,
            expand: Default::default(),
            metadata: Default::default(),
            url,
        }
    }
}
/// Events sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateWebhookEndpointApiVersion {
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
    #[serde(rename = "2019-05-16")]
    V2019_05_16,
    #[serde(rename = "2019-08-14")]
    V2019_08_14,
    #[serde(rename = "2019-09-09")]
    V2019_09_09,
    #[serde(rename = "2019-10-08")]
    V2019_10_08,
    #[serde(rename = "2019-10-17")]
    V2019_10_17,
    #[serde(rename = "2019-11-05")]
    V2019_11_05,
    #[serde(rename = "2019-12-03")]
    V2019_12_03,
    #[serde(rename = "2020-03-02")]
    V2020_03_02,
    #[serde(rename = "2020-08-27")]
    V2020_08_27,
    #[serde(rename = "2022-08-01")]
    V2022_08_01,
}

impl CreateWebhookEndpointApiVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V2011_01_01 => "2011-01-01",
            Self::V2011_06_21 => "2011-06-21",
            Self::V2011_06_28 => "2011-06-28",
            Self::V2011_08_01 => "2011-08-01",
            Self::V2011_09_15 => "2011-09-15",
            Self::V2011_11_17 => "2011-11-17",
            Self::V2012_02_23 => "2012-02-23",
            Self::V2012_03_25 => "2012-03-25",
            Self::V2012_06_18 => "2012-06-18",
            Self::V2012_06_28 => "2012-06-28",
            Self::V2012_07_09 => "2012-07-09",
            Self::V2012_09_24 => "2012-09-24",
            Self::V2012_10_26 => "2012-10-26",
            Self::V2012_11_07 => "2012-11-07",
            Self::V2013_02_11 => "2013-02-11",
            Self::V2013_02_13 => "2013-02-13",
            Self::V2013_07_05 => "2013-07-05",
            Self::V2013_08_12 => "2013-08-12",
            Self::V2013_08_13 => "2013-08-13",
            Self::V2013_10_29 => "2013-10-29",
            Self::V2013_12_03 => "2013-12-03",
            Self::V2014_01_31 => "2014-01-31",
            Self::V2014_03_13 => "2014-03-13",
            Self::V2014_03_28 => "2014-03-28",
            Self::V2014_05_19 => "2014-05-19",
            Self::V2014_06_13 => "2014-06-13",
            Self::V2014_06_17 => "2014-06-17",
            Self::V2014_07_22 => "2014-07-22",
            Self::V2014_07_26 => "2014-07-26",
            Self::V2014_08_04 => "2014-08-04",
            Self::V2014_08_20 => "2014-08-20",
            Self::V2014_09_08 => "2014-09-08",
            Self::V2014_10_07 => "2014-10-07",
            Self::V2014_11_05 => "2014-11-05",
            Self::V2014_11_20 => "2014-11-20",
            Self::V2014_12_08 => "2014-12-08",
            Self::V2014_12_17 => "2014-12-17",
            Self::V2014_12_22 => "2014-12-22",
            Self::V2015_01_11 => "2015-01-11",
            Self::V2015_01_26 => "2015-01-26",
            Self::V2015_02_10 => "2015-02-10",
            Self::V2015_02_16 => "2015-02-16",
            Self::V2015_02_18 => "2015-02-18",
            Self::V2015_03_24 => "2015-03-24",
            Self::V2015_04_07 => "2015-04-07",
            Self::V2015_06_15 => "2015-06-15",
            Self::V2015_07_07 => "2015-07-07",
            Self::V2015_07_13 => "2015-07-13",
            Self::V2015_07_28 => "2015-07-28",
            Self::V2015_08_07 => "2015-08-07",
            Self::V2015_08_19 => "2015-08-19",
            Self::V2015_09_03 => "2015-09-03",
            Self::V2015_09_08 => "2015-09-08",
            Self::V2015_09_23 => "2015-09-23",
            Self::V2015_10_01 => "2015-10-01",
            Self::V2015_10_12 => "2015-10-12",
            Self::V2015_10_16 => "2015-10-16",
            Self::V2016_02_03 => "2016-02-03",
            Self::V2016_02_19 => "2016-02-19",
            Self::V2016_02_22 => "2016-02-22",
            Self::V2016_02_23 => "2016-02-23",
            Self::V2016_02_29 => "2016-02-29",
            Self::V2016_03_07 => "2016-03-07",
            Self::V2016_06_15 => "2016-06-15",
            Self::V2016_07_06 => "2016-07-06",
            Self::V2016_10_19 => "2016-10-19",
            Self::V2017_01_27 => "2017-01-27",
            Self::V2017_02_14 => "2017-02-14",
            Self::V2017_04_06 => "2017-04-06",
            Self::V2017_05_25 => "2017-05-25",
            Self::V2017_06_05 => "2017-06-05",
            Self::V2017_08_15 => "2017-08-15",
            Self::V2017_12_14 => "2017-12-14",
            Self::V2018_01_23 => "2018-01-23",
            Self::V2018_02_05 => "2018-02-05",
            Self::V2018_02_06 => "2018-02-06",
            Self::V2018_02_28 => "2018-02-28",
            Self::V2018_05_21 => "2018-05-21",
            Self::V2018_07_27 => "2018-07-27",
            Self::V2018_08_23 => "2018-08-23",
            Self::V2018_09_06 => "2018-09-06",
            Self::V2018_09_24 => "2018-09-24",
            Self::V2018_10_31 => "2018-10-31",
            Self::V2018_11_08 => "2018-11-08",
            Self::V2019_02_11 => "2019-02-11",
            Self::V2019_02_19 => "2019-02-19",
            Self::V2019_03_14 => "2019-03-14",
            Self::V2019_05_16 => "2019-05-16",
            Self::V2019_08_14 => "2019-08-14",
            Self::V2019_09_09 => "2019-09-09",
            Self::V2019_10_08 => "2019-10-08",
            Self::V2019_10_17 => "2019-10-17",
            Self::V2019_11_05 => "2019-11-05",
            Self::V2019_12_03 => "2019-12-03",
            Self::V2020_03_02 => "2020-03-02",
            Self::V2020_08_27 => "2020-08-27",
            Self::V2022_08_01 => "2022-08-01",
        }
    }
}

impl AsRef<str> for CreateWebhookEndpointApiVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateWebhookEndpointApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of events to enable for this endpoint.
///
/// You may specify `['*']` to enable all events, except those that require explicit selection.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateWebhookEndpointEnabledEvents {
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
    #[serde(rename = "billing_portal.configuration.created")]
    BillingPortalConfigurationCreated,
    #[serde(rename = "billing_portal.configuration.updated")]
    BillingPortalConfigurationUpdated,
    #[serde(rename = "billing_portal.session.created")]
    BillingPortalSessionCreated,
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
    #[serde(rename = "customer.subscription.pending_update_applied")]
    CustomerSubscriptionPendingUpdateApplied,
    #[serde(rename = "customer.subscription.pending_update_expired")]
    CustomerSubscriptionPendingUpdateExpired,
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
    #[serde(rename = "customer_cash_balance_transaction.created")]
    CustomerCashBalanceTransactionCreated,
    #[serde(rename = "file.created")]
    FileCreated,
    #[serde(rename = "financial_connections.account.created")]
    FinancialConnectionsAccountCreated,
    #[serde(rename = "financial_connections.account.deactivated")]
    FinancialConnectionsAccountDeactivated,
    #[serde(rename = "financial_connections.account.disconnected")]
    FinancialConnectionsAccountDisconnected,
    #[serde(rename = "financial_connections.account.reactivated")]
    FinancialConnectionsAccountReactivated,
    #[serde(rename = "financial_connections.account.refreshed_balance")]
    FinancialConnectionsAccountRefreshedBalance,
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
    #[serde(rename = "transfer.reversed")]
    TransferReversed,
    #[serde(rename = "transfer.updated")]
    TransferUpdated,
    #[serde(rename = "treasury.credit_reversal.created")]
    TreasuryCreditReversalCreated,
    #[serde(rename = "treasury.credit_reversal.posted")]
    TreasuryCreditReversalPosted,
    #[serde(rename = "treasury.debit_reversal.completed")]
    TreasuryDebitReversalCompleted,
    #[serde(rename = "treasury.debit_reversal.created")]
    TreasuryDebitReversalCreated,
    #[serde(rename = "treasury.debit_reversal.initial_credit_granted")]
    TreasuryDebitReversalInitialCreditGranted,
    #[serde(rename = "treasury.financial_account.closed")]
    TreasuryFinancialAccountClosed,
    #[serde(rename = "treasury.financial_account.created")]
    TreasuryFinancialAccountCreated,
    #[serde(rename = "treasury.financial_account.features_status_updated")]
    TreasuryFinancialAccountFeaturesStatusUpdated,
    #[serde(rename = "treasury.inbound_transfer.canceled")]
    TreasuryInboundTransferCanceled,
    #[serde(rename = "treasury.inbound_transfer.created")]
    TreasuryInboundTransferCreated,
    #[serde(rename = "treasury.inbound_transfer.failed")]
    TreasuryInboundTransferFailed,
    #[serde(rename = "treasury.inbound_transfer.succeeded")]
    TreasuryInboundTransferSucceeded,
    #[serde(rename = "treasury.outbound_payment.canceled")]
    TreasuryOutboundPaymentCanceled,
    #[serde(rename = "treasury.outbound_payment.created")]
    TreasuryOutboundPaymentCreated,
    #[serde(rename = "treasury.outbound_payment.expected_arrival_date_updated")]
    TreasuryOutboundPaymentExpectedArrivalDateUpdated,
    #[serde(rename = "treasury.outbound_payment.failed")]
    TreasuryOutboundPaymentFailed,
    #[serde(rename = "treasury.outbound_payment.posted")]
    TreasuryOutboundPaymentPosted,
    #[serde(rename = "treasury.outbound_payment.returned")]
    TreasuryOutboundPaymentReturned,
    #[serde(rename = "treasury.outbound_transfer.canceled")]
    TreasuryOutboundTransferCanceled,
    #[serde(rename = "treasury.outbound_transfer.created")]
    TreasuryOutboundTransferCreated,
    #[serde(rename = "treasury.outbound_transfer.expected_arrival_date_updated")]
    TreasuryOutboundTransferExpectedArrivalDateUpdated,
    #[serde(rename = "treasury.outbound_transfer.failed")]
    TreasuryOutboundTransferFailed,
    #[serde(rename = "treasury.outbound_transfer.posted")]
    TreasuryOutboundTransferPosted,
    #[serde(rename = "treasury.outbound_transfer.returned")]
    TreasuryOutboundTransferReturned,
    #[serde(rename = "treasury.received_credit.created")]
    TreasuryReceivedCreditCreated,
    #[serde(rename = "treasury.received_credit.failed")]
    TreasuryReceivedCreditFailed,
    #[serde(rename = "treasury.received_credit.succeeded")]
    TreasuryReceivedCreditSucceeded,
    #[serde(rename = "treasury.received_debit.created")]
    TreasuryReceivedDebitCreated,
}

impl CreateWebhookEndpointEnabledEvents {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::All => "*",
            Self::AccountApplicationAuthorized => "account.application.authorized",
            Self::AccountApplicationDeauthorized => "account.application.deauthorized",
            Self::AccountExternalAccountCreated => "account.external_account.created",
            Self::AccountExternalAccountDeleted => "account.external_account.deleted",
            Self::AccountExternalAccountUpdated => "account.external_account.updated",
            Self::AccountUpdated => "account.updated",
            Self::ApplicationFeeCreated => "application_fee.created",
            Self::ApplicationFeeRefundUpdated => "application_fee.refund.updated",
            Self::ApplicationFeeRefunded => "application_fee.refunded",
            Self::BalanceAvailable => "balance.available",
            Self::BillingPortalConfigurationCreated => "billing_portal.configuration.created",
            Self::BillingPortalConfigurationUpdated => "billing_portal.configuration.updated",
            Self::BillingPortalSessionCreated => "billing_portal.session.created",
            Self::CapabilityUpdated => "capability.updated",
            Self::CashBalanceFundsAvailable => "cash_balance.funds_available",
            Self::ChargeCaptured => "charge.captured",
            Self::ChargeDisputeClosed => "charge.dispute.closed",
            Self::ChargeDisputeCreated => "charge.dispute.created",
            Self::ChargeDisputeFundsReinstated => "charge.dispute.funds_reinstated",
            Self::ChargeDisputeFundsWithdrawn => "charge.dispute.funds_withdrawn",
            Self::ChargeDisputeUpdated => "charge.dispute.updated",
            Self::ChargeExpired => "charge.expired",
            Self::ChargeFailed => "charge.failed",
            Self::ChargePending => "charge.pending",
            Self::ChargeRefundUpdated => "charge.refund.updated",
            Self::ChargeRefunded => "charge.refunded",
            Self::ChargeSucceeded => "charge.succeeded",
            Self::ChargeUpdated => "charge.updated",
            Self::CheckoutSessionAsyncPaymentFailed => "checkout.session.async_payment_failed",
            Self::CheckoutSessionAsyncPaymentSucceeded => {
                "checkout.session.async_payment_succeeded"
            }
            Self::CheckoutSessionCompleted => "checkout.session.completed",
            Self::CheckoutSessionExpired => "checkout.session.expired",
            Self::CouponCreated => "coupon.created",
            Self::CouponDeleted => "coupon.deleted",
            Self::CouponUpdated => "coupon.updated",
            Self::CreditNoteCreated => "credit_note.created",
            Self::CreditNoteUpdated => "credit_note.updated",
            Self::CreditNoteVoided => "credit_note.voided",
            Self::CustomerCreated => "customer.created",
            Self::CustomerDeleted => "customer.deleted",
            Self::CustomerDiscountCreated => "customer.discount.created",
            Self::CustomerDiscountDeleted => "customer.discount.deleted",
            Self::CustomerDiscountUpdated => "customer.discount.updated",
            Self::CustomerSourceCreated => "customer.source.created",
            Self::CustomerSourceDeleted => "customer.source.deleted",
            Self::CustomerSourceExpiring => "customer.source.expiring",
            Self::CustomerSourceUpdated => "customer.source.updated",
            Self::CustomerSubscriptionCreated => "customer.subscription.created",
            Self::CustomerSubscriptionDeleted => "customer.subscription.deleted",
            Self::CustomerSubscriptionPendingUpdateApplied => {
                "customer.subscription.pending_update_applied"
            }
            Self::CustomerSubscriptionPendingUpdateExpired => {
                "customer.subscription.pending_update_expired"
            }
            Self::CustomerSubscriptionTrialWillEnd => "customer.subscription.trial_will_end",
            Self::CustomerSubscriptionUpdated => "customer.subscription.updated",
            Self::CustomerTaxIdCreated => "customer.tax_id.created",
            Self::CustomerTaxIdDeleted => "customer.tax_id.deleted",
            Self::CustomerTaxIdUpdated => "customer.tax_id.updated",
            Self::CustomerUpdated => "customer.updated",
            Self::CustomerCashBalanceTransactionCreated => {
                "customer_cash_balance_transaction.created"
            }
            Self::FileCreated => "file.created",
            Self::FinancialConnectionsAccountCreated => "financial_connections.account.created",
            Self::FinancialConnectionsAccountDeactivated => {
                "financial_connections.account.deactivated"
            }
            Self::FinancialConnectionsAccountDisconnected => {
                "financial_connections.account.disconnected"
            }
            Self::FinancialConnectionsAccountReactivated => {
                "financial_connections.account.reactivated"
            }
            Self::FinancialConnectionsAccountRefreshedBalance => {
                "financial_connections.account.refreshed_balance"
            }
            Self::IdentityVerificationSessionCanceled => "identity.verification_session.canceled",
            Self::IdentityVerificationSessionCreated => "identity.verification_session.created",
            Self::IdentityVerificationSessionProcessing => {
                "identity.verification_session.processing"
            }
            Self::IdentityVerificationSessionRedacted => "identity.verification_session.redacted",
            Self::IdentityVerificationSessionRequiresInput => {
                "identity.verification_session.requires_input"
            }
            Self::IdentityVerificationSessionVerified => "identity.verification_session.verified",
            Self::InvoiceCreated => "invoice.created",
            Self::InvoiceDeleted => "invoice.deleted",
            Self::InvoiceFinalizationFailed => "invoice.finalization_failed",
            Self::InvoiceFinalized => "invoice.finalized",
            Self::InvoiceMarkedUncollectible => "invoice.marked_uncollectible",
            Self::InvoicePaid => "invoice.paid",
            Self::InvoicePaymentActionRequired => "invoice.payment_action_required",
            Self::InvoicePaymentFailed => "invoice.payment_failed",
            Self::InvoicePaymentSucceeded => "invoice.payment_succeeded",
            Self::InvoiceSent => "invoice.sent",
            Self::InvoiceUpcoming => "invoice.upcoming",
            Self::InvoiceUpdated => "invoice.updated",
            Self::InvoiceVoided => "invoice.voided",
            Self::InvoiceitemCreated => "invoiceitem.created",
            Self::InvoiceitemDeleted => "invoiceitem.deleted",
            Self::InvoiceitemUpdated => "invoiceitem.updated",
            Self::IssuingAuthorizationCreated => "issuing_authorization.created",
            Self::IssuingAuthorizationRequest => "issuing_authorization.request",
            Self::IssuingAuthorizationUpdated => "issuing_authorization.updated",
            Self::IssuingCardCreated => "issuing_card.created",
            Self::IssuingCardUpdated => "issuing_card.updated",
            Self::IssuingCardholderCreated => "issuing_cardholder.created",
            Self::IssuingCardholderUpdated => "issuing_cardholder.updated",
            Self::IssuingDisputeClosed => "issuing_dispute.closed",
            Self::IssuingDisputeCreated => "issuing_dispute.created",
            Self::IssuingDisputeFundsReinstated => "issuing_dispute.funds_reinstated",
            Self::IssuingDisputeSubmitted => "issuing_dispute.submitted",
            Self::IssuingDisputeUpdated => "issuing_dispute.updated",
            Self::IssuingTransactionCreated => "issuing_transaction.created",
            Self::IssuingTransactionUpdated => "issuing_transaction.updated",
            Self::MandateUpdated => "mandate.updated",
            Self::OrderCreated => "order.created",
            Self::PaymentIntentAmountCapturableUpdated => {
                "payment_intent.amount_capturable_updated"
            }
            Self::PaymentIntentCanceled => "payment_intent.canceled",
            Self::PaymentIntentCreated => "payment_intent.created",
            Self::PaymentIntentPartiallyFunded => "payment_intent.partially_funded",
            Self::PaymentIntentPaymentFailed => "payment_intent.payment_failed",
            Self::PaymentIntentProcessing => "payment_intent.processing",
            Self::PaymentIntentRequiresAction => "payment_intent.requires_action",
            Self::PaymentIntentSucceeded => "payment_intent.succeeded",
            Self::PaymentLinkCreated => "payment_link.created",
            Self::PaymentLinkUpdated => "payment_link.updated",
            Self::PaymentMethodAttached => "payment_method.attached",
            Self::PaymentMethodAutomaticallyUpdated => "payment_method.automatically_updated",
            Self::PaymentMethodDetached => "payment_method.detached",
            Self::PaymentMethodUpdated => "payment_method.updated",
            Self::PayoutCanceled => "payout.canceled",
            Self::PayoutCreated => "payout.created",
            Self::PayoutFailed => "payout.failed",
            Self::PayoutPaid => "payout.paid",
            Self::PayoutUpdated => "payout.updated",
            Self::PersonCreated => "person.created",
            Self::PersonDeleted => "person.deleted",
            Self::PersonUpdated => "person.updated",
            Self::PlanCreated => "plan.created",
            Self::PlanDeleted => "plan.deleted",
            Self::PlanUpdated => "plan.updated",
            Self::PriceCreated => "price.created",
            Self::PriceDeleted => "price.deleted",
            Self::PriceUpdated => "price.updated",
            Self::ProductCreated => "product.created",
            Self::ProductDeleted => "product.deleted",
            Self::ProductUpdated => "product.updated",
            Self::PromotionCodeCreated => "promotion_code.created",
            Self::PromotionCodeUpdated => "promotion_code.updated",
            Self::QuoteAccepted => "quote.accepted",
            Self::QuoteCanceled => "quote.canceled",
            Self::QuoteCreated => "quote.created",
            Self::QuoteFinalized => "quote.finalized",
            Self::RadarEarlyFraudWarningCreated => "radar.early_fraud_warning.created",
            Self::RadarEarlyFraudWarningUpdated => "radar.early_fraud_warning.updated",
            Self::RecipientCreated => "recipient.created",
            Self::RecipientDeleted => "recipient.deleted",
            Self::RecipientUpdated => "recipient.updated",
            Self::ReportingReportRunFailed => "reporting.report_run.failed",
            Self::ReportingReportRunSucceeded => "reporting.report_run.succeeded",
            Self::ReportingReportTypeUpdated => "reporting.report_type.updated",
            Self::ReviewClosed => "review.closed",
            Self::ReviewOpened => "review.opened",
            Self::SetupIntentCanceled => "setup_intent.canceled",
            Self::SetupIntentCreated => "setup_intent.created",
            Self::SetupIntentRequiresAction => "setup_intent.requires_action",
            Self::SetupIntentSetupFailed => "setup_intent.setup_failed",
            Self::SetupIntentSucceeded => "setup_intent.succeeded",
            Self::SigmaScheduledQueryRunCreated => "sigma.scheduled_query_run.created",
            Self::SkuCreated => "sku.created",
            Self::SkuDeleted => "sku.deleted",
            Self::SkuUpdated => "sku.updated",
            Self::SourceCanceled => "source.canceled",
            Self::SourceChargeable => "source.chargeable",
            Self::SourceFailed => "source.failed",
            Self::SourceMandateNotification => "source.mandate_notification",
            Self::SourceRefundAttributesRequired => "source.refund_attributes_required",
            Self::SourceTransactionCreated => "source.transaction.created",
            Self::SourceTransactionUpdated => "source.transaction.updated",
            Self::SubscriptionScheduleAborted => "subscription_schedule.aborted",
            Self::SubscriptionScheduleCanceled => "subscription_schedule.canceled",
            Self::SubscriptionScheduleCompleted => "subscription_schedule.completed",
            Self::SubscriptionScheduleCreated => "subscription_schedule.created",
            Self::SubscriptionScheduleExpiring => "subscription_schedule.expiring",
            Self::SubscriptionScheduleReleased => "subscription_schedule.released",
            Self::SubscriptionScheduleUpdated => "subscription_schedule.updated",
            Self::TaxRateCreated => "tax_rate.created",
            Self::TaxRateUpdated => "tax_rate.updated",
            Self::TerminalReaderActionFailed => "terminal.reader.action_failed",
            Self::TerminalReaderActionSucceeded => "terminal.reader.action_succeeded",
            Self::TestHelpersTestClockAdvancing => "test_helpers.test_clock.advancing",
            Self::TestHelpersTestClockCreated => "test_helpers.test_clock.created",
            Self::TestHelpersTestClockDeleted => "test_helpers.test_clock.deleted",
            Self::TestHelpersTestClockInternalFailure => "test_helpers.test_clock.internal_failure",
            Self::TestHelpersTestClockReady => "test_helpers.test_clock.ready",
            Self::TopupCanceled => "topup.canceled",
            Self::TopupCreated => "topup.created",
            Self::TopupFailed => "topup.failed",
            Self::TopupReversed => "topup.reversed",
            Self::TopupSucceeded => "topup.succeeded",
            Self::TransferCreated => "transfer.created",
            Self::TransferReversed => "transfer.reversed",
            Self::TransferUpdated => "transfer.updated",
            Self::TreasuryCreditReversalCreated => "treasury.credit_reversal.created",
            Self::TreasuryCreditReversalPosted => "treasury.credit_reversal.posted",
            Self::TreasuryDebitReversalCompleted => "treasury.debit_reversal.completed",
            Self::TreasuryDebitReversalCreated => "treasury.debit_reversal.created",
            Self::TreasuryDebitReversalInitialCreditGranted => {
                "treasury.debit_reversal.initial_credit_granted"
            }
            Self::TreasuryFinancialAccountClosed => "treasury.financial_account.closed",
            Self::TreasuryFinancialAccountCreated => "treasury.financial_account.created",
            Self::TreasuryFinancialAccountFeaturesStatusUpdated => {
                "treasury.financial_account.features_status_updated"
            }
            Self::TreasuryInboundTransferCanceled => "treasury.inbound_transfer.canceled",
            Self::TreasuryInboundTransferCreated => "treasury.inbound_transfer.created",
            Self::TreasuryInboundTransferFailed => "treasury.inbound_transfer.failed",
            Self::TreasuryInboundTransferSucceeded => "treasury.inbound_transfer.succeeded",
            Self::TreasuryOutboundPaymentCanceled => "treasury.outbound_payment.canceled",
            Self::TreasuryOutboundPaymentCreated => "treasury.outbound_payment.created",
            Self::TreasuryOutboundPaymentExpectedArrivalDateUpdated => {
                "treasury.outbound_payment.expected_arrival_date_updated"
            }
            Self::TreasuryOutboundPaymentFailed => "treasury.outbound_payment.failed",
            Self::TreasuryOutboundPaymentPosted => "treasury.outbound_payment.posted",
            Self::TreasuryOutboundPaymentReturned => "treasury.outbound_payment.returned",
            Self::TreasuryOutboundTransferCanceled => "treasury.outbound_transfer.canceled",
            Self::TreasuryOutboundTransferCreated => "treasury.outbound_transfer.created",
            Self::TreasuryOutboundTransferExpectedArrivalDateUpdated => {
                "treasury.outbound_transfer.expected_arrival_date_updated"
            }
            Self::TreasuryOutboundTransferFailed => "treasury.outbound_transfer.failed",
            Self::TreasuryOutboundTransferPosted => "treasury.outbound_transfer.posted",
            Self::TreasuryOutboundTransferReturned => "treasury.outbound_transfer.returned",
            Self::TreasuryReceivedCreditCreated => "treasury.received_credit.created",
            Self::TreasuryReceivedCreditFailed => "treasury.received_credit.failed",
            Self::TreasuryReceivedCreditSucceeded => "treasury.received_credit.succeeded",
            Self::TreasuryReceivedDebitCreated => "treasury.received_debit.created",
        }
    }
}

impl AsRef<str> for CreateWebhookEndpointEnabledEvents {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateWebhookEndpointEnabledEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateWebhookEndpoint<'a> {
    /// An optional description of what the webhook is used for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Disable the webhook endpoint if set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The list of events to enable for this endpoint.
    ///
    /// You may specify `['*']` to enable all events, except those that require explicit selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_events: Option<&'a [UpdateWebhookEndpointEnabledEvents]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The URL of the webhook endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> UpdateWebhookEndpoint<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of events to enable for this endpoint.
///
/// You may specify `['*']` to enable all events, except those that require explicit selection.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateWebhookEndpointEnabledEvents {
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
    #[serde(rename = "billing_portal.configuration.created")]
    BillingPortalConfigurationCreated,
    #[serde(rename = "billing_portal.configuration.updated")]
    BillingPortalConfigurationUpdated,
    #[serde(rename = "billing_portal.session.created")]
    BillingPortalSessionCreated,
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
    #[serde(rename = "customer.subscription.pending_update_applied")]
    CustomerSubscriptionPendingUpdateApplied,
    #[serde(rename = "customer.subscription.pending_update_expired")]
    CustomerSubscriptionPendingUpdateExpired,
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
    #[serde(rename = "customer_cash_balance_transaction.created")]
    CustomerCashBalanceTransactionCreated,
    #[serde(rename = "file.created")]
    FileCreated,
    #[serde(rename = "financial_connections.account.created")]
    FinancialConnectionsAccountCreated,
    #[serde(rename = "financial_connections.account.deactivated")]
    FinancialConnectionsAccountDeactivated,
    #[serde(rename = "financial_connections.account.disconnected")]
    FinancialConnectionsAccountDisconnected,
    #[serde(rename = "financial_connections.account.reactivated")]
    FinancialConnectionsAccountReactivated,
    #[serde(rename = "financial_connections.account.refreshed_balance")]
    FinancialConnectionsAccountRefreshedBalance,
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
    #[serde(rename = "transfer.reversed")]
    TransferReversed,
    #[serde(rename = "transfer.updated")]
    TransferUpdated,
    #[serde(rename = "treasury.credit_reversal.created")]
    TreasuryCreditReversalCreated,
    #[serde(rename = "treasury.credit_reversal.posted")]
    TreasuryCreditReversalPosted,
    #[serde(rename = "treasury.debit_reversal.completed")]
    TreasuryDebitReversalCompleted,
    #[serde(rename = "treasury.debit_reversal.created")]
    TreasuryDebitReversalCreated,
    #[serde(rename = "treasury.debit_reversal.initial_credit_granted")]
    TreasuryDebitReversalInitialCreditGranted,
    #[serde(rename = "treasury.financial_account.closed")]
    TreasuryFinancialAccountClosed,
    #[serde(rename = "treasury.financial_account.created")]
    TreasuryFinancialAccountCreated,
    #[serde(rename = "treasury.financial_account.features_status_updated")]
    TreasuryFinancialAccountFeaturesStatusUpdated,
    #[serde(rename = "treasury.inbound_transfer.canceled")]
    TreasuryInboundTransferCanceled,
    #[serde(rename = "treasury.inbound_transfer.created")]
    TreasuryInboundTransferCreated,
    #[serde(rename = "treasury.inbound_transfer.failed")]
    TreasuryInboundTransferFailed,
    #[serde(rename = "treasury.inbound_transfer.succeeded")]
    TreasuryInboundTransferSucceeded,
    #[serde(rename = "treasury.outbound_payment.canceled")]
    TreasuryOutboundPaymentCanceled,
    #[serde(rename = "treasury.outbound_payment.created")]
    TreasuryOutboundPaymentCreated,
    #[serde(rename = "treasury.outbound_payment.expected_arrival_date_updated")]
    TreasuryOutboundPaymentExpectedArrivalDateUpdated,
    #[serde(rename = "treasury.outbound_payment.failed")]
    TreasuryOutboundPaymentFailed,
    #[serde(rename = "treasury.outbound_payment.posted")]
    TreasuryOutboundPaymentPosted,
    #[serde(rename = "treasury.outbound_payment.returned")]
    TreasuryOutboundPaymentReturned,
    #[serde(rename = "treasury.outbound_transfer.canceled")]
    TreasuryOutboundTransferCanceled,
    #[serde(rename = "treasury.outbound_transfer.created")]
    TreasuryOutboundTransferCreated,
    #[serde(rename = "treasury.outbound_transfer.expected_arrival_date_updated")]
    TreasuryOutboundTransferExpectedArrivalDateUpdated,
    #[serde(rename = "treasury.outbound_transfer.failed")]
    TreasuryOutboundTransferFailed,
    #[serde(rename = "treasury.outbound_transfer.posted")]
    TreasuryOutboundTransferPosted,
    #[serde(rename = "treasury.outbound_transfer.returned")]
    TreasuryOutboundTransferReturned,
    #[serde(rename = "treasury.received_credit.created")]
    TreasuryReceivedCreditCreated,
    #[serde(rename = "treasury.received_credit.failed")]
    TreasuryReceivedCreditFailed,
    #[serde(rename = "treasury.received_credit.succeeded")]
    TreasuryReceivedCreditSucceeded,
    #[serde(rename = "treasury.received_debit.created")]
    TreasuryReceivedDebitCreated,
}

impl UpdateWebhookEndpointEnabledEvents {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::All => "*",
            Self::AccountApplicationAuthorized => "account.application.authorized",
            Self::AccountApplicationDeauthorized => "account.application.deauthorized",
            Self::AccountExternalAccountCreated => "account.external_account.created",
            Self::AccountExternalAccountDeleted => "account.external_account.deleted",
            Self::AccountExternalAccountUpdated => "account.external_account.updated",
            Self::AccountUpdated => "account.updated",
            Self::ApplicationFeeCreated => "application_fee.created",
            Self::ApplicationFeeRefundUpdated => "application_fee.refund.updated",
            Self::ApplicationFeeRefunded => "application_fee.refunded",
            Self::BalanceAvailable => "balance.available",
            Self::BillingPortalConfigurationCreated => "billing_portal.configuration.created",
            Self::BillingPortalConfigurationUpdated => "billing_portal.configuration.updated",
            Self::BillingPortalSessionCreated => "billing_portal.session.created",
            Self::CapabilityUpdated => "capability.updated",
            Self::CashBalanceFundsAvailable => "cash_balance.funds_available",
            Self::ChargeCaptured => "charge.captured",
            Self::ChargeDisputeClosed => "charge.dispute.closed",
            Self::ChargeDisputeCreated => "charge.dispute.created",
            Self::ChargeDisputeFundsReinstated => "charge.dispute.funds_reinstated",
            Self::ChargeDisputeFundsWithdrawn => "charge.dispute.funds_withdrawn",
            Self::ChargeDisputeUpdated => "charge.dispute.updated",
            Self::ChargeExpired => "charge.expired",
            Self::ChargeFailed => "charge.failed",
            Self::ChargePending => "charge.pending",
            Self::ChargeRefundUpdated => "charge.refund.updated",
            Self::ChargeRefunded => "charge.refunded",
            Self::ChargeSucceeded => "charge.succeeded",
            Self::ChargeUpdated => "charge.updated",
            Self::CheckoutSessionAsyncPaymentFailed => "checkout.session.async_payment_failed",
            Self::CheckoutSessionAsyncPaymentSucceeded => {
                "checkout.session.async_payment_succeeded"
            }
            Self::CheckoutSessionCompleted => "checkout.session.completed",
            Self::CheckoutSessionExpired => "checkout.session.expired",
            Self::CouponCreated => "coupon.created",
            Self::CouponDeleted => "coupon.deleted",
            Self::CouponUpdated => "coupon.updated",
            Self::CreditNoteCreated => "credit_note.created",
            Self::CreditNoteUpdated => "credit_note.updated",
            Self::CreditNoteVoided => "credit_note.voided",
            Self::CustomerCreated => "customer.created",
            Self::CustomerDeleted => "customer.deleted",
            Self::CustomerDiscountCreated => "customer.discount.created",
            Self::CustomerDiscountDeleted => "customer.discount.deleted",
            Self::CustomerDiscountUpdated => "customer.discount.updated",
            Self::CustomerSourceCreated => "customer.source.created",
            Self::CustomerSourceDeleted => "customer.source.deleted",
            Self::CustomerSourceExpiring => "customer.source.expiring",
            Self::CustomerSourceUpdated => "customer.source.updated",
            Self::CustomerSubscriptionCreated => "customer.subscription.created",
            Self::CustomerSubscriptionDeleted => "customer.subscription.deleted",
            Self::CustomerSubscriptionPendingUpdateApplied => {
                "customer.subscription.pending_update_applied"
            }
            Self::CustomerSubscriptionPendingUpdateExpired => {
                "customer.subscription.pending_update_expired"
            }
            Self::CustomerSubscriptionTrialWillEnd => "customer.subscription.trial_will_end",
            Self::CustomerSubscriptionUpdated => "customer.subscription.updated",
            Self::CustomerTaxIdCreated => "customer.tax_id.created",
            Self::CustomerTaxIdDeleted => "customer.tax_id.deleted",
            Self::CustomerTaxIdUpdated => "customer.tax_id.updated",
            Self::CustomerUpdated => "customer.updated",
            Self::CustomerCashBalanceTransactionCreated => {
                "customer_cash_balance_transaction.created"
            }
            Self::FileCreated => "file.created",
            Self::FinancialConnectionsAccountCreated => "financial_connections.account.created",
            Self::FinancialConnectionsAccountDeactivated => {
                "financial_connections.account.deactivated"
            }
            Self::FinancialConnectionsAccountDisconnected => {
                "financial_connections.account.disconnected"
            }
            Self::FinancialConnectionsAccountReactivated => {
                "financial_connections.account.reactivated"
            }
            Self::FinancialConnectionsAccountRefreshedBalance => {
                "financial_connections.account.refreshed_balance"
            }
            Self::IdentityVerificationSessionCanceled => "identity.verification_session.canceled",
            Self::IdentityVerificationSessionCreated => "identity.verification_session.created",
            Self::IdentityVerificationSessionProcessing => {
                "identity.verification_session.processing"
            }
            Self::IdentityVerificationSessionRedacted => "identity.verification_session.redacted",
            Self::IdentityVerificationSessionRequiresInput => {
                "identity.verification_session.requires_input"
            }
            Self::IdentityVerificationSessionVerified => "identity.verification_session.verified",
            Self::InvoiceCreated => "invoice.created",
            Self::InvoiceDeleted => "invoice.deleted",
            Self::InvoiceFinalizationFailed => "invoice.finalization_failed",
            Self::InvoiceFinalized => "invoice.finalized",
            Self::InvoiceMarkedUncollectible => "invoice.marked_uncollectible",
            Self::InvoicePaid => "invoice.paid",
            Self::InvoicePaymentActionRequired => "invoice.payment_action_required",
            Self::InvoicePaymentFailed => "invoice.payment_failed",
            Self::InvoicePaymentSucceeded => "invoice.payment_succeeded",
            Self::InvoiceSent => "invoice.sent",
            Self::InvoiceUpcoming => "invoice.upcoming",
            Self::InvoiceUpdated => "invoice.updated",
            Self::InvoiceVoided => "invoice.voided",
            Self::InvoiceitemCreated => "invoiceitem.created",
            Self::InvoiceitemDeleted => "invoiceitem.deleted",
            Self::InvoiceitemUpdated => "invoiceitem.updated",
            Self::IssuingAuthorizationCreated => "issuing_authorization.created",
            Self::IssuingAuthorizationRequest => "issuing_authorization.request",
            Self::IssuingAuthorizationUpdated => "issuing_authorization.updated",
            Self::IssuingCardCreated => "issuing_card.created",
            Self::IssuingCardUpdated => "issuing_card.updated",
            Self::IssuingCardholderCreated => "issuing_cardholder.created",
            Self::IssuingCardholderUpdated => "issuing_cardholder.updated",
            Self::IssuingDisputeClosed => "issuing_dispute.closed",
            Self::IssuingDisputeCreated => "issuing_dispute.created",
            Self::IssuingDisputeFundsReinstated => "issuing_dispute.funds_reinstated",
            Self::IssuingDisputeSubmitted => "issuing_dispute.submitted",
            Self::IssuingDisputeUpdated => "issuing_dispute.updated",
            Self::IssuingTransactionCreated => "issuing_transaction.created",
            Self::IssuingTransactionUpdated => "issuing_transaction.updated",
            Self::MandateUpdated => "mandate.updated",
            Self::OrderCreated => "order.created",
            Self::PaymentIntentAmountCapturableUpdated => {
                "payment_intent.amount_capturable_updated"
            }
            Self::PaymentIntentCanceled => "payment_intent.canceled",
            Self::PaymentIntentCreated => "payment_intent.created",
            Self::PaymentIntentPartiallyFunded => "payment_intent.partially_funded",
            Self::PaymentIntentPaymentFailed => "payment_intent.payment_failed",
            Self::PaymentIntentProcessing => "payment_intent.processing",
            Self::PaymentIntentRequiresAction => "payment_intent.requires_action",
            Self::PaymentIntentSucceeded => "payment_intent.succeeded",
            Self::PaymentLinkCreated => "payment_link.created",
            Self::PaymentLinkUpdated => "payment_link.updated",
            Self::PaymentMethodAttached => "payment_method.attached",
            Self::PaymentMethodAutomaticallyUpdated => "payment_method.automatically_updated",
            Self::PaymentMethodDetached => "payment_method.detached",
            Self::PaymentMethodUpdated => "payment_method.updated",
            Self::PayoutCanceled => "payout.canceled",
            Self::PayoutCreated => "payout.created",
            Self::PayoutFailed => "payout.failed",
            Self::PayoutPaid => "payout.paid",
            Self::PayoutUpdated => "payout.updated",
            Self::PersonCreated => "person.created",
            Self::PersonDeleted => "person.deleted",
            Self::PersonUpdated => "person.updated",
            Self::PlanCreated => "plan.created",
            Self::PlanDeleted => "plan.deleted",
            Self::PlanUpdated => "plan.updated",
            Self::PriceCreated => "price.created",
            Self::PriceDeleted => "price.deleted",
            Self::PriceUpdated => "price.updated",
            Self::ProductCreated => "product.created",
            Self::ProductDeleted => "product.deleted",
            Self::ProductUpdated => "product.updated",
            Self::PromotionCodeCreated => "promotion_code.created",
            Self::PromotionCodeUpdated => "promotion_code.updated",
            Self::QuoteAccepted => "quote.accepted",
            Self::QuoteCanceled => "quote.canceled",
            Self::QuoteCreated => "quote.created",
            Self::QuoteFinalized => "quote.finalized",
            Self::RadarEarlyFraudWarningCreated => "radar.early_fraud_warning.created",
            Self::RadarEarlyFraudWarningUpdated => "radar.early_fraud_warning.updated",
            Self::RecipientCreated => "recipient.created",
            Self::RecipientDeleted => "recipient.deleted",
            Self::RecipientUpdated => "recipient.updated",
            Self::ReportingReportRunFailed => "reporting.report_run.failed",
            Self::ReportingReportRunSucceeded => "reporting.report_run.succeeded",
            Self::ReportingReportTypeUpdated => "reporting.report_type.updated",
            Self::ReviewClosed => "review.closed",
            Self::ReviewOpened => "review.opened",
            Self::SetupIntentCanceled => "setup_intent.canceled",
            Self::SetupIntentCreated => "setup_intent.created",
            Self::SetupIntentRequiresAction => "setup_intent.requires_action",
            Self::SetupIntentSetupFailed => "setup_intent.setup_failed",
            Self::SetupIntentSucceeded => "setup_intent.succeeded",
            Self::SigmaScheduledQueryRunCreated => "sigma.scheduled_query_run.created",
            Self::SkuCreated => "sku.created",
            Self::SkuDeleted => "sku.deleted",
            Self::SkuUpdated => "sku.updated",
            Self::SourceCanceled => "source.canceled",
            Self::SourceChargeable => "source.chargeable",
            Self::SourceFailed => "source.failed",
            Self::SourceMandateNotification => "source.mandate_notification",
            Self::SourceRefundAttributesRequired => "source.refund_attributes_required",
            Self::SourceTransactionCreated => "source.transaction.created",
            Self::SourceTransactionUpdated => "source.transaction.updated",
            Self::SubscriptionScheduleAborted => "subscription_schedule.aborted",
            Self::SubscriptionScheduleCanceled => "subscription_schedule.canceled",
            Self::SubscriptionScheduleCompleted => "subscription_schedule.completed",
            Self::SubscriptionScheduleCreated => "subscription_schedule.created",
            Self::SubscriptionScheduleExpiring => "subscription_schedule.expiring",
            Self::SubscriptionScheduleReleased => "subscription_schedule.released",
            Self::SubscriptionScheduleUpdated => "subscription_schedule.updated",
            Self::TaxRateCreated => "tax_rate.created",
            Self::TaxRateUpdated => "tax_rate.updated",
            Self::TerminalReaderActionFailed => "terminal.reader.action_failed",
            Self::TerminalReaderActionSucceeded => "terminal.reader.action_succeeded",
            Self::TestHelpersTestClockAdvancing => "test_helpers.test_clock.advancing",
            Self::TestHelpersTestClockCreated => "test_helpers.test_clock.created",
            Self::TestHelpersTestClockDeleted => "test_helpers.test_clock.deleted",
            Self::TestHelpersTestClockInternalFailure => "test_helpers.test_clock.internal_failure",
            Self::TestHelpersTestClockReady => "test_helpers.test_clock.ready",
            Self::TopupCanceled => "topup.canceled",
            Self::TopupCreated => "topup.created",
            Self::TopupFailed => "topup.failed",
            Self::TopupReversed => "topup.reversed",
            Self::TopupSucceeded => "topup.succeeded",
            Self::TransferCreated => "transfer.created",
            Self::TransferReversed => "transfer.reversed",
            Self::TransferUpdated => "transfer.updated",
            Self::TreasuryCreditReversalCreated => "treasury.credit_reversal.created",
            Self::TreasuryCreditReversalPosted => "treasury.credit_reversal.posted",
            Self::TreasuryDebitReversalCompleted => "treasury.debit_reversal.completed",
            Self::TreasuryDebitReversalCreated => "treasury.debit_reversal.created",
            Self::TreasuryDebitReversalInitialCreditGranted => {
                "treasury.debit_reversal.initial_credit_granted"
            }
            Self::TreasuryFinancialAccountClosed => "treasury.financial_account.closed",
            Self::TreasuryFinancialAccountCreated => "treasury.financial_account.created",
            Self::TreasuryFinancialAccountFeaturesStatusUpdated => {
                "treasury.financial_account.features_status_updated"
            }
            Self::TreasuryInboundTransferCanceled => "treasury.inbound_transfer.canceled",
            Self::TreasuryInboundTransferCreated => "treasury.inbound_transfer.created",
            Self::TreasuryInboundTransferFailed => "treasury.inbound_transfer.failed",
            Self::TreasuryInboundTransferSucceeded => "treasury.inbound_transfer.succeeded",
            Self::TreasuryOutboundPaymentCanceled => "treasury.outbound_payment.canceled",
            Self::TreasuryOutboundPaymentCreated => "treasury.outbound_payment.created",
            Self::TreasuryOutboundPaymentExpectedArrivalDateUpdated => {
                "treasury.outbound_payment.expected_arrival_date_updated"
            }
            Self::TreasuryOutboundPaymentFailed => "treasury.outbound_payment.failed",
            Self::TreasuryOutboundPaymentPosted => "treasury.outbound_payment.posted",
            Self::TreasuryOutboundPaymentReturned => "treasury.outbound_payment.returned",
            Self::TreasuryOutboundTransferCanceled => "treasury.outbound_transfer.canceled",
            Self::TreasuryOutboundTransferCreated => "treasury.outbound_transfer.created",
            Self::TreasuryOutboundTransferExpectedArrivalDateUpdated => {
                "treasury.outbound_transfer.expected_arrival_date_updated"
            }
            Self::TreasuryOutboundTransferFailed => "treasury.outbound_transfer.failed",
            Self::TreasuryOutboundTransferPosted => "treasury.outbound_transfer.posted",
            Self::TreasuryOutboundTransferReturned => "treasury.outbound_transfer.returned",
            Self::TreasuryReceivedCreditCreated => "treasury.received_credit.created",
            Self::TreasuryReceivedCreditFailed => "treasury.received_credit.failed",
            Self::TreasuryReceivedCreditSucceeded => "treasury.received_credit.succeeded",
            Self::TreasuryReceivedDebitCreated => "treasury.received_debit.created",
        }
    }
}

impl AsRef<str> for UpdateWebhookEndpointEnabledEvents {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateWebhookEndpointEnabledEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}