use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// You can also delete webhook endpoints via the [webhook endpoint management](https://dashboard.stripe.com/account/webhooks) page of the Stripe dashboard.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DeleteWebhookEndpoint {
    webhook_endpoint: stripe_misc::WebhookEndpointId,
}
impl DeleteWebhookEndpoint {
    /// Construct a new `DeleteWebhookEndpoint`.
    pub fn new(webhook_endpoint: impl Into<stripe_misc::WebhookEndpointId>) -> Self {
        Self { webhook_endpoint: webhook_endpoint.into() }
    }
}
impl DeleteWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteWebhookEndpoint {
    type Output = stripe_misc::DeletedWebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        let webhook_endpoint = &self.webhook_endpoint;
        RequestBuilder::new(StripeMethod::Delete, format!("/webhook_endpoints/{webhook_endpoint}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListWebhookEndpointBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of your webhook endpoints.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListWebhookEndpoint {
    inner: ListWebhookEndpointBuilder,
}
impl ListWebhookEndpoint {
    /// Construct a new `ListWebhookEndpoint`.
    pub fn new() -> Self {
        Self { inner: ListWebhookEndpointBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListWebhookEndpoint {
    fn default() -> Self {
        Self::new()
    }
}
impl ListWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::WebhookEndpoint>> {
        stripe_client_core::ListPaginator::new_list("/webhook_endpoints", &self.inner)
    }
}

impl StripeRequest for ListWebhookEndpoint {
    type Output = stripe_types::List<stripe_misc::WebhookEndpoint>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/webhook_endpoints").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveWebhookEndpointBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the webhook endpoint with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveWebhookEndpoint {
    inner: RetrieveWebhookEndpointBuilder,
    webhook_endpoint: stripe_misc::WebhookEndpointId,
}
impl RetrieveWebhookEndpoint {
    /// Construct a new `RetrieveWebhookEndpoint`.
    pub fn new(webhook_endpoint: impl Into<stripe_misc::WebhookEndpointId>) -> Self {
        Self {
            webhook_endpoint: webhook_endpoint.into(),
            inner: RetrieveWebhookEndpointBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveWebhookEndpoint {
    type Output = stripe_misc::WebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        let webhook_endpoint = &self.webhook_endpoint;
        RequestBuilder::new(StripeMethod::Get, format!("/webhook_endpoints/{webhook_endpoint}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<stripe_shared::ApiVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    enabled_events: Vec<CreateWebhookEndpointEnabledEvents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    url: String,
}
impl CreateWebhookEndpointBuilder {
    fn new(
        enabled_events: impl Into<Vec<CreateWebhookEndpointEnabledEvents>>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            api_version: None,
            connect: None,
            description: None,
            enabled_events: enabled_events.into(),
            expand: None,
            metadata: None,
            url: url.into(),
        }
    }
}
/// The list of events to enable for this endpoint.
/// You may specify `['*']` to enable all events, except those that require explicit selection.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateWebhookEndpointEnabledEvents {
    All,
    AccountApplicationAuthorized,
    AccountApplicationDeauthorized,
    AccountExternalAccountCreated,
    AccountExternalAccountDeleted,
    AccountExternalAccountUpdated,
    AccountUpdated,
    ApplicationFeeCreated,
    ApplicationFeeRefundUpdated,
    ApplicationFeeRefunded,
    BalanceAvailable,
    BillingPortalConfigurationCreated,
    BillingPortalConfigurationUpdated,
    BillingPortalSessionCreated,
    CapabilityUpdated,
    CashBalanceFundsAvailable,
    ChargeCaptured,
    ChargeDisputeClosed,
    ChargeDisputeCreated,
    ChargeDisputeFundsReinstated,
    ChargeDisputeFundsWithdrawn,
    ChargeDisputeUpdated,
    ChargeExpired,
    ChargeFailed,
    ChargePending,
    ChargeRefundUpdated,
    ChargeRefunded,
    ChargeSucceeded,
    ChargeUpdated,
    CheckoutSessionAsyncPaymentFailed,
    CheckoutSessionAsyncPaymentSucceeded,
    CheckoutSessionCompleted,
    CheckoutSessionExpired,
    ClimateOrderCanceled,
    ClimateOrderCreated,
    ClimateOrderDelayed,
    ClimateOrderDelivered,
    ClimateOrderProductSubstituted,
    ClimateProductCreated,
    ClimateProductPricingUpdated,
    CouponCreated,
    CouponDeleted,
    CouponUpdated,
    CreditNoteCreated,
    CreditNoteUpdated,
    CreditNoteVoided,
    CustomerCreated,
    CustomerDeleted,
    CustomerDiscountCreated,
    CustomerDiscountDeleted,
    CustomerDiscountUpdated,
    CustomerSourceCreated,
    CustomerSourceDeleted,
    CustomerSourceExpiring,
    CustomerSourceUpdated,
    CustomerSubscriptionCreated,
    CustomerSubscriptionDeleted,
    CustomerSubscriptionPaused,
    CustomerSubscriptionPendingUpdateApplied,
    CustomerSubscriptionPendingUpdateExpired,
    CustomerSubscriptionResumed,
    CustomerSubscriptionTrialWillEnd,
    CustomerSubscriptionUpdated,
    CustomerTaxIdCreated,
    CustomerTaxIdDeleted,
    CustomerTaxIdUpdated,
    CustomerUpdated,
    CustomerCashBalanceTransactionCreated,
    EntitlementsActiveEntitlementSummaryUpdated,
    FileCreated,
    FinancialConnectionsAccountCreated,
    FinancialConnectionsAccountDeactivated,
    FinancialConnectionsAccountDisconnected,
    FinancialConnectionsAccountReactivated,
    FinancialConnectionsAccountRefreshedBalance,
    FinancialConnectionsAccountRefreshedOwnership,
    FinancialConnectionsAccountRefreshedTransactions,
    IdentityVerificationSessionCanceled,
    IdentityVerificationSessionCreated,
    IdentityVerificationSessionProcessing,
    IdentityVerificationSessionRedacted,
    IdentityVerificationSessionRequiresInput,
    IdentityVerificationSessionVerified,
    InvoiceCreated,
    InvoiceDeleted,
    InvoiceFinalizationFailed,
    InvoiceFinalized,
    InvoiceMarkedUncollectible,
    InvoicePaid,
    InvoicePaymentActionRequired,
    InvoicePaymentFailed,
    InvoicePaymentSucceeded,
    InvoiceSent,
    InvoiceUpcoming,
    InvoiceUpdated,
    InvoiceVoided,
    InvoiceitemCreated,
    InvoiceitemDeleted,
    IssuingAuthorizationCreated,
    IssuingAuthorizationRequest,
    IssuingAuthorizationUpdated,
    IssuingCardCreated,
    IssuingCardUpdated,
    IssuingCardholderCreated,
    IssuingCardholderUpdated,
    IssuingDisputeClosed,
    IssuingDisputeCreated,
    IssuingDisputeFundsReinstated,
    IssuingDisputeSubmitted,
    IssuingDisputeUpdated,
    IssuingTokenCreated,
    IssuingTokenUpdated,
    IssuingTransactionCreated,
    IssuingTransactionUpdated,
    MandateUpdated,
    PaymentIntentAmountCapturableUpdated,
    PaymentIntentCanceled,
    PaymentIntentCreated,
    PaymentIntentPartiallyFunded,
    PaymentIntentPaymentFailed,
    PaymentIntentProcessing,
    PaymentIntentRequiresAction,
    PaymentIntentSucceeded,
    PaymentLinkCreated,
    PaymentLinkUpdated,
    PaymentMethodAttached,
    PaymentMethodAutomaticallyUpdated,
    PaymentMethodDetached,
    PaymentMethodUpdated,
    PayoutCanceled,
    PayoutCreated,
    PayoutFailed,
    PayoutPaid,
    PayoutReconciliationCompleted,
    PayoutUpdated,
    PersonCreated,
    PersonDeleted,
    PersonUpdated,
    PlanCreated,
    PlanDeleted,
    PlanUpdated,
    PriceCreated,
    PriceDeleted,
    PriceUpdated,
    ProductCreated,
    ProductDeleted,
    ProductUpdated,
    PromotionCodeCreated,
    PromotionCodeUpdated,
    QuoteAccepted,
    QuoteCanceled,
    QuoteCreated,
    QuoteFinalized,
    RadarEarlyFraudWarningCreated,
    RadarEarlyFraudWarningUpdated,
    RefundCreated,
    RefundUpdated,
    ReportingReportRunFailed,
    ReportingReportRunSucceeded,
    ReportingReportTypeUpdated,
    ReviewClosed,
    ReviewOpened,
    SetupIntentCanceled,
    SetupIntentCreated,
    SetupIntentRequiresAction,
    SetupIntentSetupFailed,
    SetupIntentSucceeded,
    SigmaScheduledQueryRunCreated,
    SourceCanceled,
    SourceChargeable,
    SourceFailed,
    SourceMandateNotification,
    SourceRefundAttributesRequired,
    SourceTransactionCreated,
    SourceTransactionUpdated,
    SubscriptionScheduleAborted,
    SubscriptionScheduleCanceled,
    SubscriptionScheduleCompleted,
    SubscriptionScheduleCreated,
    SubscriptionScheduleExpiring,
    SubscriptionScheduleReleased,
    SubscriptionScheduleUpdated,
    TaxSettingsUpdated,
    TaxRateCreated,
    TaxRateUpdated,
    TerminalReaderActionFailed,
    TerminalReaderActionSucceeded,
    TestHelpersTestClockAdvancing,
    TestHelpersTestClockCreated,
    TestHelpersTestClockDeleted,
    TestHelpersTestClockInternalFailure,
    TestHelpersTestClockReady,
    TopupCanceled,
    TopupCreated,
    TopupFailed,
    TopupReversed,
    TopupSucceeded,
    TransferCreated,
    TransferReversed,
    TransferUpdated,
    TreasuryCreditReversalCreated,
    TreasuryCreditReversalPosted,
    TreasuryDebitReversalCompleted,
    TreasuryDebitReversalCreated,
    TreasuryDebitReversalInitialCreditGranted,
    TreasuryFinancialAccountClosed,
    TreasuryFinancialAccountCreated,
    TreasuryFinancialAccountFeaturesStatusUpdated,
    TreasuryInboundTransferCanceled,
    TreasuryInboundTransferCreated,
    TreasuryInboundTransferFailed,
    TreasuryInboundTransferSucceeded,
    TreasuryOutboundPaymentCanceled,
    TreasuryOutboundPaymentCreated,
    TreasuryOutboundPaymentExpectedArrivalDateUpdated,
    TreasuryOutboundPaymentFailed,
    TreasuryOutboundPaymentPosted,
    TreasuryOutboundPaymentReturned,
    TreasuryOutboundTransferCanceled,
    TreasuryOutboundTransferCreated,
    TreasuryOutboundTransferExpectedArrivalDateUpdated,
    TreasuryOutboundTransferFailed,
    TreasuryOutboundTransferPosted,
    TreasuryOutboundTransferReturned,
    TreasuryReceivedCreditCreated,
    TreasuryReceivedCreditFailed,
    TreasuryReceivedCreditSucceeded,
    TreasuryReceivedDebitCreated,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateWebhookEndpointEnabledEvents {
    pub fn as_str(self) -> &'static str {
        use CreateWebhookEndpointEnabledEvents::*;
        match self {
            All => "*",
            AccountApplicationAuthorized => "account.application.authorized",
            AccountApplicationDeauthorized => "account.application.deauthorized",
            AccountExternalAccountCreated => "account.external_account.created",
            AccountExternalAccountDeleted => "account.external_account.deleted",
            AccountExternalAccountUpdated => "account.external_account.updated",
            AccountUpdated => "account.updated",
            ApplicationFeeCreated => "application_fee.created",
            ApplicationFeeRefundUpdated => "application_fee.refund.updated",
            ApplicationFeeRefunded => "application_fee.refunded",
            BalanceAvailable => "balance.available",
            BillingPortalConfigurationCreated => "billing_portal.configuration.created",
            BillingPortalConfigurationUpdated => "billing_portal.configuration.updated",
            BillingPortalSessionCreated => "billing_portal.session.created",
            CapabilityUpdated => "capability.updated",
            CashBalanceFundsAvailable => "cash_balance.funds_available",
            ChargeCaptured => "charge.captured",
            ChargeDisputeClosed => "charge.dispute.closed",
            ChargeDisputeCreated => "charge.dispute.created",
            ChargeDisputeFundsReinstated => "charge.dispute.funds_reinstated",
            ChargeDisputeFundsWithdrawn => "charge.dispute.funds_withdrawn",
            ChargeDisputeUpdated => "charge.dispute.updated",
            ChargeExpired => "charge.expired",
            ChargeFailed => "charge.failed",
            ChargePending => "charge.pending",
            ChargeRefundUpdated => "charge.refund.updated",
            ChargeRefunded => "charge.refunded",
            ChargeSucceeded => "charge.succeeded",
            ChargeUpdated => "charge.updated",
            CheckoutSessionAsyncPaymentFailed => "checkout.session.async_payment_failed",
            CheckoutSessionAsyncPaymentSucceeded => "checkout.session.async_payment_succeeded",
            CheckoutSessionCompleted => "checkout.session.completed",
            CheckoutSessionExpired => "checkout.session.expired",
            ClimateOrderCanceled => "climate.order.canceled",
            ClimateOrderCreated => "climate.order.created",
            ClimateOrderDelayed => "climate.order.delayed",
            ClimateOrderDelivered => "climate.order.delivered",
            ClimateOrderProductSubstituted => "climate.order.product_substituted",
            ClimateProductCreated => "climate.product.created",
            ClimateProductPricingUpdated => "climate.product.pricing_updated",
            CouponCreated => "coupon.created",
            CouponDeleted => "coupon.deleted",
            CouponUpdated => "coupon.updated",
            CreditNoteCreated => "credit_note.created",
            CreditNoteUpdated => "credit_note.updated",
            CreditNoteVoided => "credit_note.voided",
            CustomerCreated => "customer.created",
            CustomerDeleted => "customer.deleted",
            CustomerDiscountCreated => "customer.discount.created",
            CustomerDiscountDeleted => "customer.discount.deleted",
            CustomerDiscountUpdated => "customer.discount.updated",
            CustomerSourceCreated => "customer.source.created",
            CustomerSourceDeleted => "customer.source.deleted",
            CustomerSourceExpiring => "customer.source.expiring",
            CustomerSourceUpdated => "customer.source.updated",
            CustomerSubscriptionCreated => "customer.subscription.created",
            CustomerSubscriptionDeleted => "customer.subscription.deleted",
            CustomerSubscriptionPaused => "customer.subscription.paused",
            CustomerSubscriptionPendingUpdateApplied => {
                "customer.subscription.pending_update_applied"
            }
            CustomerSubscriptionPendingUpdateExpired => {
                "customer.subscription.pending_update_expired"
            }
            CustomerSubscriptionResumed => "customer.subscription.resumed",
            CustomerSubscriptionTrialWillEnd => "customer.subscription.trial_will_end",
            CustomerSubscriptionUpdated => "customer.subscription.updated",
            CustomerTaxIdCreated => "customer.tax_id.created",
            CustomerTaxIdDeleted => "customer.tax_id.deleted",
            CustomerTaxIdUpdated => "customer.tax_id.updated",
            CustomerUpdated => "customer.updated",
            CustomerCashBalanceTransactionCreated => "customer_cash_balance_transaction.created",
            EntitlementsActiveEntitlementSummaryUpdated => {
                "entitlements.active_entitlement_summary.updated"
            }
            FileCreated => "file.created",
            FinancialConnectionsAccountCreated => "financial_connections.account.created",
            FinancialConnectionsAccountDeactivated => "financial_connections.account.deactivated",
            FinancialConnectionsAccountDisconnected => "financial_connections.account.disconnected",
            FinancialConnectionsAccountReactivated => "financial_connections.account.reactivated",
            FinancialConnectionsAccountRefreshedBalance => {
                "financial_connections.account.refreshed_balance"
            }
            FinancialConnectionsAccountRefreshedOwnership => {
                "financial_connections.account.refreshed_ownership"
            }
            FinancialConnectionsAccountRefreshedTransactions => {
                "financial_connections.account.refreshed_transactions"
            }
            IdentityVerificationSessionCanceled => "identity.verification_session.canceled",
            IdentityVerificationSessionCreated => "identity.verification_session.created",
            IdentityVerificationSessionProcessing => "identity.verification_session.processing",
            IdentityVerificationSessionRedacted => "identity.verification_session.redacted",
            IdentityVerificationSessionRequiresInput => {
                "identity.verification_session.requires_input"
            }
            IdentityVerificationSessionVerified => "identity.verification_session.verified",
            InvoiceCreated => "invoice.created",
            InvoiceDeleted => "invoice.deleted",
            InvoiceFinalizationFailed => "invoice.finalization_failed",
            InvoiceFinalized => "invoice.finalized",
            InvoiceMarkedUncollectible => "invoice.marked_uncollectible",
            InvoicePaid => "invoice.paid",
            InvoicePaymentActionRequired => "invoice.payment_action_required",
            InvoicePaymentFailed => "invoice.payment_failed",
            InvoicePaymentSucceeded => "invoice.payment_succeeded",
            InvoiceSent => "invoice.sent",
            InvoiceUpcoming => "invoice.upcoming",
            InvoiceUpdated => "invoice.updated",
            InvoiceVoided => "invoice.voided",
            InvoiceitemCreated => "invoiceitem.created",
            InvoiceitemDeleted => "invoiceitem.deleted",
            IssuingAuthorizationCreated => "issuing_authorization.created",
            IssuingAuthorizationRequest => "issuing_authorization.request",
            IssuingAuthorizationUpdated => "issuing_authorization.updated",
            IssuingCardCreated => "issuing_card.created",
            IssuingCardUpdated => "issuing_card.updated",
            IssuingCardholderCreated => "issuing_cardholder.created",
            IssuingCardholderUpdated => "issuing_cardholder.updated",
            IssuingDisputeClosed => "issuing_dispute.closed",
            IssuingDisputeCreated => "issuing_dispute.created",
            IssuingDisputeFundsReinstated => "issuing_dispute.funds_reinstated",
            IssuingDisputeSubmitted => "issuing_dispute.submitted",
            IssuingDisputeUpdated => "issuing_dispute.updated",
            IssuingTokenCreated => "issuing_token.created",
            IssuingTokenUpdated => "issuing_token.updated",
            IssuingTransactionCreated => "issuing_transaction.created",
            IssuingTransactionUpdated => "issuing_transaction.updated",
            MandateUpdated => "mandate.updated",
            PaymentIntentAmountCapturableUpdated => "payment_intent.amount_capturable_updated",
            PaymentIntentCanceled => "payment_intent.canceled",
            PaymentIntentCreated => "payment_intent.created",
            PaymentIntentPartiallyFunded => "payment_intent.partially_funded",
            PaymentIntentPaymentFailed => "payment_intent.payment_failed",
            PaymentIntentProcessing => "payment_intent.processing",
            PaymentIntentRequiresAction => "payment_intent.requires_action",
            PaymentIntentSucceeded => "payment_intent.succeeded",
            PaymentLinkCreated => "payment_link.created",
            PaymentLinkUpdated => "payment_link.updated",
            PaymentMethodAttached => "payment_method.attached",
            PaymentMethodAutomaticallyUpdated => "payment_method.automatically_updated",
            PaymentMethodDetached => "payment_method.detached",
            PaymentMethodUpdated => "payment_method.updated",
            PayoutCanceled => "payout.canceled",
            PayoutCreated => "payout.created",
            PayoutFailed => "payout.failed",
            PayoutPaid => "payout.paid",
            PayoutReconciliationCompleted => "payout.reconciliation_completed",
            PayoutUpdated => "payout.updated",
            PersonCreated => "person.created",
            PersonDeleted => "person.deleted",
            PersonUpdated => "person.updated",
            PlanCreated => "plan.created",
            PlanDeleted => "plan.deleted",
            PlanUpdated => "plan.updated",
            PriceCreated => "price.created",
            PriceDeleted => "price.deleted",
            PriceUpdated => "price.updated",
            ProductCreated => "product.created",
            ProductDeleted => "product.deleted",
            ProductUpdated => "product.updated",
            PromotionCodeCreated => "promotion_code.created",
            PromotionCodeUpdated => "promotion_code.updated",
            QuoteAccepted => "quote.accepted",
            QuoteCanceled => "quote.canceled",
            QuoteCreated => "quote.created",
            QuoteFinalized => "quote.finalized",
            RadarEarlyFraudWarningCreated => "radar.early_fraud_warning.created",
            RadarEarlyFraudWarningUpdated => "radar.early_fraud_warning.updated",
            RefundCreated => "refund.created",
            RefundUpdated => "refund.updated",
            ReportingReportRunFailed => "reporting.report_run.failed",
            ReportingReportRunSucceeded => "reporting.report_run.succeeded",
            ReportingReportTypeUpdated => "reporting.report_type.updated",
            ReviewClosed => "review.closed",
            ReviewOpened => "review.opened",
            SetupIntentCanceled => "setup_intent.canceled",
            SetupIntentCreated => "setup_intent.created",
            SetupIntentRequiresAction => "setup_intent.requires_action",
            SetupIntentSetupFailed => "setup_intent.setup_failed",
            SetupIntentSucceeded => "setup_intent.succeeded",
            SigmaScheduledQueryRunCreated => "sigma.scheduled_query_run.created",
            SourceCanceled => "source.canceled",
            SourceChargeable => "source.chargeable",
            SourceFailed => "source.failed",
            SourceMandateNotification => "source.mandate_notification",
            SourceRefundAttributesRequired => "source.refund_attributes_required",
            SourceTransactionCreated => "source.transaction.created",
            SourceTransactionUpdated => "source.transaction.updated",
            SubscriptionScheduleAborted => "subscription_schedule.aborted",
            SubscriptionScheduleCanceled => "subscription_schedule.canceled",
            SubscriptionScheduleCompleted => "subscription_schedule.completed",
            SubscriptionScheduleCreated => "subscription_schedule.created",
            SubscriptionScheduleExpiring => "subscription_schedule.expiring",
            SubscriptionScheduleReleased => "subscription_schedule.released",
            SubscriptionScheduleUpdated => "subscription_schedule.updated",
            TaxSettingsUpdated => "tax.settings.updated",
            TaxRateCreated => "tax_rate.created",
            TaxRateUpdated => "tax_rate.updated",
            TerminalReaderActionFailed => "terminal.reader.action_failed",
            TerminalReaderActionSucceeded => "terminal.reader.action_succeeded",
            TestHelpersTestClockAdvancing => "test_helpers.test_clock.advancing",
            TestHelpersTestClockCreated => "test_helpers.test_clock.created",
            TestHelpersTestClockDeleted => "test_helpers.test_clock.deleted",
            TestHelpersTestClockInternalFailure => "test_helpers.test_clock.internal_failure",
            TestHelpersTestClockReady => "test_helpers.test_clock.ready",
            TopupCanceled => "topup.canceled",
            TopupCreated => "topup.created",
            TopupFailed => "topup.failed",
            TopupReversed => "topup.reversed",
            TopupSucceeded => "topup.succeeded",
            TransferCreated => "transfer.created",
            TransferReversed => "transfer.reversed",
            TransferUpdated => "transfer.updated",
            TreasuryCreditReversalCreated => "treasury.credit_reversal.created",
            TreasuryCreditReversalPosted => "treasury.credit_reversal.posted",
            TreasuryDebitReversalCompleted => "treasury.debit_reversal.completed",
            TreasuryDebitReversalCreated => "treasury.debit_reversal.created",
            TreasuryDebitReversalInitialCreditGranted => {
                "treasury.debit_reversal.initial_credit_granted"
            }
            TreasuryFinancialAccountClosed => "treasury.financial_account.closed",
            TreasuryFinancialAccountCreated => "treasury.financial_account.created",
            TreasuryFinancialAccountFeaturesStatusUpdated => {
                "treasury.financial_account.features_status_updated"
            }
            TreasuryInboundTransferCanceled => "treasury.inbound_transfer.canceled",
            TreasuryInboundTransferCreated => "treasury.inbound_transfer.created",
            TreasuryInboundTransferFailed => "treasury.inbound_transfer.failed",
            TreasuryInboundTransferSucceeded => "treasury.inbound_transfer.succeeded",
            TreasuryOutboundPaymentCanceled => "treasury.outbound_payment.canceled",
            TreasuryOutboundPaymentCreated => "treasury.outbound_payment.created",
            TreasuryOutboundPaymentExpectedArrivalDateUpdated => {
                "treasury.outbound_payment.expected_arrival_date_updated"
            }
            TreasuryOutboundPaymentFailed => "treasury.outbound_payment.failed",
            TreasuryOutboundPaymentPosted => "treasury.outbound_payment.posted",
            TreasuryOutboundPaymentReturned => "treasury.outbound_payment.returned",
            TreasuryOutboundTransferCanceled => "treasury.outbound_transfer.canceled",
            TreasuryOutboundTransferCreated => "treasury.outbound_transfer.created",
            TreasuryOutboundTransferExpectedArrivalDateUpdated => {
                "treasury.outbound_transfer.expected_arrival_date_updated"
            }
            TreasuryOutboundTransferFailed => "treasury.outbound_transfer.failed",
            TreasuryOutboundTransferPosted => "treasury.outbound_transfer.posted",
            TreasuryOutboundTransferReturned => "treasury.outbound_transfer.returned",
            TreasuryReceivedCreditCreated => "treasury.received_credit.created",
            TreasuryReceivedCreditFailed => "treasury.received_credit.failed",
            TreasuryReceivedCreditSucceeded => "treasury.received_credit.succeeded",
            TreasuryReceivedDebitCreated => "treasury.received_debit.created",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateWebhookEndpointEnabledEvents {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateWebhookEndpointEnabledEvents::*;
        match s {
            "*" => Ok(All),
            "account.application.authorized" => Ok(AccountApplicationAuthorized),
            "account.application.deauthorized" => Ok(AccountApplicationDeauthorized),
            "account.external_account.created" => Ok(AccountExternalAccountCreated),
            "account.external_account.deleted" => Ok(AccountExternalAccountDeleted),
            "account.external_account.updated" => Ok(AccountExternalAccountUpdated),
            "account.updated" => Ok(AccountUpdated),
            "application_fee.created" => Ok(ApplicationFeeCreated),
            "application_fee.refund.updated" => Ok(ApplicationFeeRefundUpdated),
            "application_fee.refunded" => Ok(ApplicationFeeRefunded),
            "balance.available" => Ok(BalanceAvailable),
            "billing_portal.configuration.created" => Ok(BillingPortalConfigurationCreated),
            "billing_portal.configuration.updated" => Ok(BillingPortalConfigurationUpdated),
            "billing_portal.session.created" => Ok(BillingPortalSessionCreated),
            "capability.updated" => Ok(CapabilityUpdated),
            "cash_balance.funds_available" => Ok(CashBalanceFundsAvailable),
            "charge.captured" => Ok(ChargeCaptured),
            "charge.dispute.closed" => Ok(ChargeDisputeClosed),
            "charge.dispute.created" => Ok(ChargeDisputeCreated),
            "charge.dispute.funds_reinstated" => Ok(ChargeDisputeFundsReinstated),
            "charge.dispute.funds_withdrawn" => Ok(ChargeDisputeFundsWithdrawn),
            "charge.dispute.updated" => Ok(ChargeDisputeUpdated),
            "charge.expired" => Ok(ChargeExpired),
            "charge.failed" => Ok(ChargeFailed),
            "charge.pending" => Ok(ChargePending),
            "charge.refund.updated" => Ok(ChargeRefundUpdated),
            "charge.refunded" => Ok(ChargeRefunded),
            "charge.succeeded" => Ok(ChargeSucceeded),
            "charge.updated" => Ok(ChargeUpdated),
            "checkout.session.async_payment_failed" => Ok(CheckoutSessionAsyncPaymentFailed),
            "checkout.session.async_payment_succeeded" => Ok(CheckoutSessionAsyncPaymentSucceeded),
            "checkout.session.completed" => Ok(CheckoutSessionCompleted),
            "checkout.session.expired" => Ok(CheckoutSessionExpired),
            "climate.order.canceled" => Ok(ClimateOrderCanceled),
            "climate.order.created" => Ok(ClimateOrderCreated),
            "climate.order.delayed" => Ok(ClimateOrderDelayed),
            "climate.order.delivered" => Ok(ClimateOrderDelivered),
            "climate.order.product_substituted" => Ok(ClimateOrderProductSubstituted),
            "climate.product.created" => Ok(ClimateProductCreated),
            "climate.product.pricing_updated" => Ok(ClimateProductPricingUpdated),
            "coupon.created" => Ok(CouponCreated),
            "coupon.deleted" => Ok(CouponDeleted),
            "coupon.updated" => Ok(CouponUpdated),
            "credit_note.created" => Ok(CreditNoteCreated),
            "credit_note.updated" => Ok(CreditNoteUpdated),
            "credit_note.voided" => Ok(CreditNoteVoided),
            "customer.created" => Ok(CustomerCreated),
            "customer.deleted" => Ok(CustomerDeleted),
            "customer.discount.created" => Ok(CustomerDiscountCreated),
            "customer.discount.deleted" => Ok(CustomerDiscountDeleted),
            "customer.discount.updated" => Ok(CustomerDiscountUpdated),
            "customer.source.created" => Ok(CustomerSourceCreated),
            "customer.source.deleted" => Ok(CustomerSourceDeleted),
            "customer.source.expiring" => Ok(CustomerSourceExpiring),
            "customer.source.updated" => Ok(CustomerSourceUpdated),
            "customer.subscription.created" => Ok(CustomerSubscriptionCreated),
            "customer.subscription.deleted" => Ok(CustomerSubscriptionDeleted),
            "customer.subscription.paused" => Ok(CustomerSubscriptionPaused),
            "customer.subscription.pending_update_applied" => {
                Ok(CustomerSubscriptionPendingUpdateApplied)
            }
            "customer.subscription.pending_update_expired" => {
                Ok(CustomerSubscriptionPendingUpdateExpired)
            }
            "customer.subscription.resumed" => Ok(CustomerSubscriptionResumed),
            "customer.subscription.trial_will_end" => Ok(CustomerSubscriptionTrialWillEnd),
            "customer.subscription.updated" => Ok(CustomerSubscriptionUpdated),
            "customer.tax_id.created" => Ok(CustomerTaxIdCreated),
            "customer.tax_id.deleted" => Ok(CustomerTaxIdDeleted),
            "customer.tax_id.updated" => Ok(CustomerTaxIdUpdated),
            "customer.updated" => Ok(CustomerUpdated),
            "customer_cash_balance_transaction.created" => {
                Ok(CustomerCashBalanceTransactionCreated)
            }
            "entitlements.active_entitlement_summary.updated" => {
                Ok(EntitlementsActiveEntitlementSummaryUpdated)
            }
            "file.created" => Ok(FileCreated),
            "financial_connections.account.created" => Ok(FinancialConnectionsAccountCreated),
            "financial_connections.account.deactivated" => {
                Ok(FinancialConnectionsAccountDeactivated)
            }
            "financial_connections.account.disconnected" => {
                Ok(FinancialConnectionsAccountDisconnected)
            }
            "financial_connections.account.reactivated" => {
                Ok(FinancialConnectionsAccountReactivated)
            }
            "financial_connections.account.refreshed_balance" => {
                Ok(FinancialConnectionsAccountRefreshedBalance)
            }
            "financial_connections.account.refreshed_ownership" => {
                Ok(FinancialConnectionsAccountRefreshedOwnership)
            }
            "financial_connections.account.refreshed_transactions" => {
                Ok(FinancialConnectionsAccountRefreshedTransactions)
            }
            "identity.verification_session.canceled" => Ok(IdentityVerificationSessionCanceled),
            "identity.verification_session.created" => Ok(IdentityVerificationSessionCreated),
            "identity.verification_session.processing" => Ok(IdentityVerificationSessionProcessing),
            "identity.verification_session.redacted" => Ok(IdentityVerificationSessionRedacted),
            "identity.verification_session.requires_input" => {
                Ok(IdentityVerificationSessionRequiresInput)
            }
            "identity.verification_session.verified" => Ok(IdentityVerificationSessionVerified),
            "invoice.created" => Ok(InvoiceCreated),
            "invoice.deleted" => Ok(InvoiceDeleted),
            "invoice.finalization_failed" => Ok(InvoiceFinalizationFailed),
            "invoice.finalized" => Ok(InvoiceFinalized),
            "invoice.marked_uncollectible" => Ok(InvoiceMarkedUncollectible),
            "invoice.paid" => Ok(InvoicePaid),
            "invoice.payment_action_required" => Ok(InvoicePaymentActionRequired),
            "invoice.payment_failed" => Ok(InvoicePaymentFailed),
            "invoice.payment_succeeded" => Ok(InvoicePaymentSucceeded),
            "invoice.sent" => Ok(InvoiceSent),
            "invoice.upcoming" => Ok(InvoiceUpcoming),
            "invoice.updated" => Ok(InvoiceUpdated),
            "invoice.voided" => Ok(InvoiceVoided),
            "invoiceitem.created" => Ok(InvoiceitemCreated),
            "invoiceitem.deleted" => Ok(InvoiceitemDeleted),
            "issuing_authorization.created" => Ok(IssuingAuthorizationCreated),
            "issuing_authorization.request" => Ok(IssuingAuthorizationRequest),
            "issuing_authorization.updated" => Ok(IssuingAuthorizationUpdated),
            "issuing_card.created" => Ok(IssuingCardCreated),
            "issuing_card.updated" => Ok(IssuingCardUpdated),
            "issuing_cardholder.created" => Ok(IssuingCardholderCreated),
            "issuing_cardholder.updated" => Ok(IssuingCardholderUpdated),
            "issuing_dispute.closed" => Ok(IssuingDisputeClosed),
            "issuing_dispute.created" => Ok(IssuingDisputeCreated),
            "issuing_dispute.funds_reinstated" => Ok(IssuingDisputeFundsReinstated),
            "issuing_dispute.submitted" => Ok(IssuingDisputeSubmitted),
            "issuing_dispute.updated" => Ok(IssuingDisputeUpdated),
            "issuing_token.created" => Ok(IssuingTokenCreated),
            "issuing_token.updated" => Ok(IssuingTokenUpdated),
            "issuing_transaction.created" => Ok(IssuingTransactionCreated),
            "issuing_transaction.updated" => Ok(IssuingTransactionUpdated),
            "mandate.updated" => Ok(MandateUpdated),
            "payment_intent.amount_capturable_updated" => Ok(PaymentIntentAmountCapturableUpdated),
            "payment_intent.canceled" => Ok(PaymentIntentCanceled),
            "payment_intent.created" => Ok(PaymentIntentCreated),
            "payment_intent.partially_funded" => Ok(PaymentIntentPartiallyFunded),
            "payment_intent.payment_failed" => Ok(PaymentIntentPaymentFailed),
            "payment_intent.processing" => Ok(PaymentIntentProcessing),
            "payment_intent.requires_action" => Ok(PaymentIntentRequiresAction),
            "payment_intent.succeeded" => Ok(PaymentIntentSucceeded),
            "payment_link.created" => Ok(PaymentLinkCreated),
            "payment_link.updated" => Ok(PaymentLinkUpdated),
            "payment_method.attached" => Ok(PaymentMethodAttached),
            "payment_method.automatically_updated" => Ok(PaymentMethodAutomaticallyUpdated),
            "payment_method.detached" => Ok(PaymentMethodDetached),
            "payment_method.updated" => Ok(PaymentMethodUpdated),
            "payout.canceled" => Ok(PayoutCanceled),
            "payout.created" => Ok(PayoutCreated),
            "payout.failed" => Ok(PayoutFailed),
            "payout.paid" => Ok(PayoutPaid),
            "payout.reconciliation_completed" => Ok(PayoutReconciliationCompleted),
            "payout.updated" => Ok(PayoutUpdated),
            "person.created" => Ok(PersonCreated),
            "person.deleted" => Ok(PersonDeleted),
            "person.updated" => Ok(PersonUpdated),
            "plan.created" => Ok(PlanCreated),
            "plan.deleted" => Ok(PlanDeleted),
            "plan.updated" => Ok(PlanUpdated),
            "price.created" => Ok(PriceCreated),
            "price.deleted" => Ok(PriceDeleted),
            "price.updated" => Ok(PriceUpdated),
            "product.created" => Ok(ProductCreated),
            "product.deleted" => Ok(ProductDeleted),
            "product.updated" => Ok(ProductUpdated),
            "promotion_code.created" => Ok(PromotionCodeCreated),
            "promotion_code.updated" => Ok(PromotionCodeUpdated),
            "quote.accepted" => Ok(QuoteAccepted),
            "quote.canceled" => Ok(QuoteCanceled),
            "quote.created" => Ok(QuoteCreated),
            "quote.finalized" => Ok(QuoteFinalized),
            "radar.early_fraud_warning.created" => Ok(RadarEarlyFraudWarningCreated),
            "radar.early_fraud_warning.updated" => Ok(RadarEarlyFraudWarningUpdated),
            "refund.created" => Ok(RefundCreated),
            "refund.updated" => Ok(RefundUpdated),
            "reporting.report_run.failed" => Ok(ReportingReportRunFailed),
            "reporting.report_run.succeeded" => Ok(ReportingReportRunSucceeded),
            "reporting.report_type.updated" => Ok(ReportingReportTypeUpdated),
            "review.closed" => Ok(ReviewClosed),
            "review.opened" => Ok(ReviewOpened),
            "setup_intent.canceled" => Ok(SetupIntentCanceled),
            "setup_intent.created" => Ok(SetupIntentCreated),
            "setup_intent.requires_action" => Ok(SetupIntentRequiresAction),
            "setup_intent.setup_failed" => Ok(SetupIntentSetupFailed),
            "setup_intent.succeeded" => Ok(SetupIntentSucceeded),
            "sigma.scheduled_query_run.created" => Ok(SigmaScheduledQueryRunCreated),
            "source.canceled" => Ok(SourceCanceled),
            "source.chargeable" => Ok(SourceChargeable),
            "source.failed" => Ok(SourceFailed),
            "source.mandate_notification" => Ok(SourceMandateNotification),
            "source.refund_attributes_required" => Ok(SourceRefundAttributesRequired),
            "source.transaction.created" => Ok(SourceTransactionCreated),
            "source.transaction.updated" => Ok(SourceTransactionUpdated),
            "subscription_schedule.aborted" => Ok(SubscriptionScheduleAborted),
            "subscription_schedule.canceled" => Ok(SubscriptionScheduleCanceled),
            "subscription_schedule.completed" => Ok(SubscriptionScheduleCompleted),
            "subscription_schedule.created" => Ok(SubscriptionScheduleCreated),
            "subscription_schedule.expiring" => Ok(SubscriptionScheduleExpiring),
            "subscription_schedule.released" => Ok(SubscriptionScheduleReleased),
            "subscription_schedule.updated" => Ok(SubscriptionScheduleUpdated),
            "tax.settings.updated" => Ok(TaxSettingsUpdated),
            "tax_rate.created" => Ok(TaxRateCreated),
            "tax_rate.updated" => Ok(TaxRateUpdated),
            "terminal.reader.action_failed" => Ok(TerminalReaderActionFailed),
            "terminal.reader.action_succeeded" => Ok(TerminalReaderActionSucceeded),
            "test_helpers.test_clock.advancing" => Ok(TestHelpersTestClockAdvancing),
            "test_helpers.test_clock.created" => Ok(TestHelpersTestClockCreated),
            "test_helpers.test_clock.deleted" => Ok(TestHelpersTestClockDeleted),
            "test_helpers.test_clock.internal_failure" => Ok(TestHelpersTestClockInternalFailure),
            "test_helpers.test_clock.ready" => Ok(TestHelpersTestClockReady),
            "topup.canceled" => Ok(TopupCanceled),
            "topup.created" => Ok(TopupCreated),
            "topup.failed" => Ok(TopupFailed),
            "topup.reversed" => Ok(TopupReversed),
            "topup.succeeded" => Ok(TopupSucceeded),
            "transfer.created" => Ok(TransferCreated),
            "transfer.reversed" => Ok(TransferReversed),
            "transfer.updated" => Ok(TransferUpdated),
            "treasury.credit_reversal.created" => Ok(TreasuryCreditReversalCreated),
            "treasury.credit_reversal.posted" => Ok(TreasuryCreditReversalPosted),
            "treasury.debit_reversal.completed" => Ok(TreasuryDebitReversalCompleted),
            "treasury.debit_reversal.created" => Ok(TreasuryDebitReversalCreated),
            "treasury.debit_reversal.initial_credit_granted" => {
                Ok(TreasuryDebitReversalInitialCreditGranted)
            }
            "treasury.financial_account.closed" => Ok(TreasuryFinancialAccountClosed),
            "treasury.financial_account.created" => Ok(TreasuryFinancialAccountCreated),
            "treasury.financial_account.features_status_updated" => {
                Ok(TreasuryFinancialAccountFeaturesStatusUpdated)
            }
            "treasury.inbound_transfer.canceled" => Ok(TreasuryInboundTransferCanceled),
            "treasury.inbound_transfer.created" => Ok(TreasuryInboundTransferCreated),
            "treasury.inbound_transfer.failed" => Ok(TreasuryInboundTransferFailed),
            "treasury.inbound_transfer.succeeded" => Ok(TreasuryInboundTransferSucceeded),
            "treasury.outbound_payment.canceled" => Ok(TreasuryOutboundPaymentCanceled),
            "treasury.outbound_payment.created" => Ok(TreasuryOutboundPaymentCreated),
            "treasury.outbound_payment.expected_arrival_date_updated" => {
                Ok(TreasuryOutboundPaymentExpectedArrivalDateUpdated)
            }
            "treasury.outbound_payment.failed" => Ok(TreasuryOutboundPaymentFailed),
            "treasury.outbound_payment.posted" => Ok(TreasuryOutboundPaymentPosted),
            "treasury.outbound_payment.returned" => Ok(TreasuryOutboundPaymentReturned),
            "treasury.outbound_transfer.canceled" => Ok(TreasuryOutboundTransferCanceled),
            "treasury.outbound_transfer.created" => Ok(TreasuryOutboundTransferCreated),
            "treasury.outbound_transfer.expected_arrival_date_updated" => {
                Ok(TreasuryOutboundTransferExpectedArrivalDateUpdated)
            }
            "treasury.outbound_transfer.failed" => Ok(TreasuryOutboundTransferFailed),
            "treasury.outbound_transfer.posted" => Ok(TreasuryOutboundTransferPosted),
            "treasury.outbound_transfer.returned" => Ok(TreasuryOutboundTransferReturned),
            "treasury.received_credit.created" => Ok(TreasuryReceivedCreditCreated),
            "treasury.received_credit.failed" => Ok(TreasuryReceivedCreditFailed),
            "treasury.received_credit.succeeded" => Ok(TreasuryReceivedCreditSucceeded),
            "treasury.received_debit.created" => Ok(TreasuryReceivedDebitCreated),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateWebhookEndpointEnabledEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateWebhookEndpointEnabledEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateWebhookEndpointEnabledEvents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateWebhookEndpointEnabledEvents {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// A webhook endpoint must have a `url` and a list of `enabled_events`.
/// You may optionally specify the Boolean `connect` parameter.
/// If set to true, then a Connect webhook endpoint that notifies the specified `url` about events from all connected accounts is created; otherwise an account webhook endpoint that notifies the specified `url` only about events from your account is created.
/// You can also create webhook endpoints in the [webhooks settings](https://dashboard.stripe.com/account/webhooks) section of the Dashboard.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateWebhookEndpoint {
    inner: CreateWebhookEndpointBuilder,
}
impl CreateWebhookEndpoint {
    /// Construct a new `CreateWebhookEndpoint`.
    pub fn new(
        enabled_events: impl Into<Vec<CreateWebhookEndpointEnabledEvents>>,
        url: impl Into<String>,
    ) -> Self {
        Self { inner: CreateWebhookEndpointBuilder::new(enabled_events.into(), url.into()) }
    }
    /// Events sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version.
    pub fn api_version(mut self, api_version: impl Into<stripe_shared::ApiVersion>) -> Self {
        self.inner.api_version = Some(api_version.into());
        self
    }
    /// Whether this endpoint should receive events from connected accounts (`true`), or from your account (`false`).
    /// Defaults to `false`.
    pub fn connect(mut self, connect: impl Into<bool>) -> Self {
        self.inner.connect = Some(connect.into());
        self
    }
    /// An optional description of what the webhook is used for.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CreateWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateWebhookEndpoint {
    type Output = stripe_misc::WebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/webhook_endpoints").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdateWebhookEndpointBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_events: Option<Vec<UpdateWebhookEndpointEnabledEvents>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}
impl UpdateWebhookEndpointBuilder {
    fn new() -> Self {
        Self {
            description: None,
            disabled: None,
            enabled_events: None,
            expand: None,
            metadata: None,
            url: None,
        }
    }
}
/// The list of events to enable for this endpoint.
/// You may specify `['*']` to enable all events, except those that require explicit selection.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateWebhookEndpointEnabledEvents {
    All,
    AccountApplicationAuthorized,
    AccountApplicationDeauthorized,
    AccountExternalAccountCreated,
    AccountExternalAccountDeleted,
    AccountExternalAccountUpdated,
    AccountUpdated,
    ApplicationFeeCreated,
    ApplicationFeeRefundUpdated,
    ApplicationFeeRefunded,
    BalanceAvailable,
    BillingPortalConfigurationCreated,
    BillingPortalConfigurationUpdated,
    BillingPortalSessionCreated,
    CapabilityUpdated,
    CashBalanceFundsAvailable,
    ChargeCaptured,
    ChargeDisputeClosed,
    ChargeDisputeCreated,
    ChargeDisputeFundsReinstated,
    ChargeDisputeFundsWithdrawn,
    ChargeDisputeUpdated,
    ChargeExpired,
    ChargeFailed,
    ChargePending,
    ChargeRefundUpdated,
    ChargeRefunded,
    ChargeSucceeded,
    ChargeUpdated,
    CheckoutSessionAsyncPaymentFailed,
    CheckoutSessionAsyncPaymentSucceeded,
    CheckoutSessionCompleted,
    CheckoutSessionExpired,
    ClimateOrderCanceled,
    ClimateOrderCreated,
    ClimateOrderDelayed,
    ClimateOrderDelivered,
    ClimateOrderProductSubstituted,
    ClimateProductCreated,
    ClimateProductPricingUpdated,
    CouponCreated,
    CouponDeleted,
    CouponUpdated,
    CreditNoteCreated,
    CreditNoteUpdated,
    CreditNoteVoided,
    CustomerCreated,
    CustomerDeleted,
    CustomerDiscountCreated,
    CustomerDiscountDeleted,
    CustomerDiscountUpdated,
    CustomerSourceCreated,
    CustomerSourceDeleted,
    CustomerSourceExpiring,
    CustomerSourceUpdated,
    CustomerSubscriptionCreated,
    CustomerSubscriptionDeleted,
    CustomerSubscriptionPaused,
    CustomerSubscriptionPendingUpdateApplied,
    CustomerSubscriptionPendingUpdateExpired,
    CustomerSubscriptionResumed,
    CustomerSubscriptionTrialWillEnd,
    CustomerSubscriptionUpdated,
    CustomerTaxIdCreated,
    CustomerTaxIdDeleted,
    CustomerTaxIdUpdated,
    CustomerUpdated,
    CustomerCashBalanceTransactionCreated,
    EntitlementsActiveEntitlementSummaryUpdated,
    FileCreated,
    FinancialConnectionsAccountCreated,
    FinancialConnectionsAccountDeactivated,
    FinancialConnectionsAccountDisconnected,
    FinancialConnectionsAccountReactivated,
    FinancialConnectionsAccountRefreshedBalance,
    FinancialConnectionsAccountRefreshedOwnership,
    FinancialConnectionsAccountRefreshedTransactions,
    IdentityVerificationSessionCanceled,
    IdentityVerificationSessionCreated,
    IdentityVerificationSessionProcessing,
    IdentityVerificationSessionRedacted,
    IdentityVerificationSessionRequiresInput,
    IdentityVerificationSessionVerified,
    InvoiceCreated,
    InvoiceDeleted,
    InvoiceFinalizationFailed,
    InvoiceFinalized,
    InvoiceMarkedUncollectible,
    InvoicePaid,
    InvoicePaymentActionRequired,
    InvoicePaymentFailed,
    InvoicePaymentSucceeded,
    InvoiceSent,
    InvoiceUpcoming,
    InvoiceUpdated,
    InvoiceVoided,
    InvoiceitemCreated,
    InvoiceitemDeleted,
    IssuingAuthorizationCreated,
    IssuingAuthorizationRequest,
    IssuingAuthorizationUpdated,
    IssuingCardCreated,
    IssuingCardUpdated,
    IssuingCardholderCreated,
    IssuingCardholderUpdated,
    IssuingDisputeClosed,
    IssuingDisputeCreated,
    IssuingDisputeFundsReinstated,
    IssuingDisputeSubmitted,
    IssuingDisputeUpdated,
    IssuingTokenCreated,
    IssuingTokenUpdated,
    IssuingTransactionCreated,
    IssuingTransactionUpdated,
    MandateUpdated,
    PaymentIntentAmountCapturableUpdated,
    PaymentIntentCanceled,
    PaymentIntentCreated,
    PaymentIntentPartiallyFunded,
    PaymentIntentPaymentFailed,
    PaymentIntentProcessing,
    PaymentIntentRequiresAction,
    PaymentIntentSucceeded,
    PaymentLinkCreated,
    PaymentLinkUpdated,
    PaymentMethodAttached,
    PaymentMethodAutomaticallyUpdated,
    PaymentMethodDetached,
    PaymentMethodUpdated,
    PayoutCanceled,
    PayoutCreated,
    PayoutFailed,
    PayoutPaid,
    PayoutReconciliationCompleted,
    PayoutUpdated,
    PersonCreated,
    PersonDeleted,
    PersonUpdated,
    PlanCreated,
    PlanDeleted,
    PlanUpdated,
    PriceCreated,
    PriceDeleted,
    PriceUpdated,
    ProductCreated,
    ProductDeleted,
    ProductUpdated,
    PromotionCodeCreated,
    PromotionCodeUpdated,
    QuoteAccepted,
    QuoteCanceled,
    QuoteCreated,
    QuoteFinalized,
    RadarEarlyFraudWarningCreated,
    RadarEarlyFraudWarningUpdated,
    RefundCreated,
    RefundUpdated,
    ReportingReportRunFailed,
    ReportingReportRunSucceeded,
    ReportingReportTypeUpdated,
    ReviewClosed,
    ReviewOpened,
    SetupIntentCanceled,
    SetupIntentCreated,
    SetupIntentRequiresAction,
    SetupIntentSetupFailed,
    SetupIntentSucceeded,
    SigmaScheduledQueryRunCreated,
    SourceCanceled,
    SourceChargeable,
    SourceFailed,
    SourceMandateNotification,
    SourceRefundAttributesRequired,
    SourceTransactionCreated,
    SourceTransactionUpdated,
    SubscriptionScheduleAborted,
    SubscriptionScheduleCanceled,
    SubscriptionScheduleCompleted,
    SubscriptionScheduleCreated,
    SubscriptionScheduleExpiring,
    SubscriptionScheduleReleased,
    SubscriptionScheduleUpdated,
    TaxSettingsUpdated,
    TaxRateCreated,
    TaxRateUpdated,
    TerminalReaderActionFailed,
    TerminalReaderActionSucceeded,
    TestHelpersTestClockAdvancing,
    TestHelpersTestClockCreated,
    TestHelpersTestClockDeleted,
    TestHelpersTestClockInternalFailure,
    TestHelpersTestClockReady,
    TopupCanceled,
    TopupCreated,
    TopupFailed,
    TopupReversed,
    TopupSucceeded,
    TransferCreated,
    TransferReversed,
    TransferUpdated,
    TreasuryCreditReversalCreated,
    TreasuryCreditReversalPosted,
    TreasuryDebitReversalCompleted,
    TreasuryDebitReversalCreated,
    TreasuryDebitReversalInitialCreditGranted,
    TreasuryFinancialAccountClosed,
    TreasuryFinancialAccountCreated,
    TreasuryFinancialAccountFeaturesStatusUpdated,
    TreasuryInboundTransferCanceled,
    TreasuryInboundTransferCreated,
    TreasuryInboundTransferFailed,
    TreasuryInboundTransferSucceeded,
    TreasuryOutboundPaymentCanceled,
    TreasuryOutboundPaymentCreated,
    TreasuryOutboundPaymentExpectedArrivalDateUpdated,
    TreasuryOutboundPaymentFailed,
    TreasuryOutboundPaymentPosted,
    TreasuryOutboundPaymentReturned,
    TreasuryOutboundTransferCanceled,
    TreasuryOutboundTransferCreated,
    TreasuryOutboundTransferExpectedArrivalDateUpdated,
    TreasuryOutboundTransferFailed,
    TreasuryOutboundTransferPosted,
    TreasuryOutboundTransferReturned,
    TreasuryReceivedCreditCreated,
    TreasuryReceivedCreditFailed,
    TreasuryReceivedCreditSucceeded,
    TreasuryReceivedDebitCreated,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl UpdateWebhookEndpointEnabledEvents {
    pub fn as_str(self) -> &'static str {
        use UpdateWebhookEndpointEnabledEvents::*;
        match self {
            All => "*",
            AccountApplicationAuthorized => "account.application.authorized",
            AccountApplicationDeauthorized => "account.application.deauthorized",
            AccountExternalAccountCreated => "account.external_account.created",
            AccountExternalAccountDeleted => "account.external_account.deleted",
            AccountExternalAccountUpdated => "account.external_account.updated",
            AccountUpdated => "account.updated",
            ApplicationFeeCreated => "application_fee.created",
            ApplicationFeeRefundUpdated => "application_fee.refund.updated",
            ApplicationFeeRefunded => "application_fee.refunded",
            BalanceAvailable => "balance.available",
            BillingPortalConfigurationCreated => "billing_portal.configuration.created",
            BillingPortalConfigurationUpdated => "billing_portal.configuration.updated",
            BillingPortalSessionCreated => "billing_portal.session.created",
            CapabilityUpdated => "capability.updated",
            CashBalanceFundsAvailable => "cash_balance.funds_available",
            ChargeCaptured => "charge.captured",
            ChargeDisputeClosed => "charge.dispute.closed",
            ChargeDisputeCreated => "charge.dispute.created",
            ChargeDisputeFundsReinstated => "charge.dispute.funds_reinstated",
            ChargeDisputeFundsWithdrawn => "charge.dispute.funds_withdrawn",
            ChargeDisputeUpdated => "charge.dispute.updated",
            ChargeExpired => "charge.expired",
            ChargeFailed => "charge.failed",
            ChargePending => "charge.pending",
            ChargeRefundUpdated => "charge.refund.updated",
            ChargeRefunded => "charge.refunded",
            ChargeSucceeded => "charge.succeeded",
            ChargeUpdated => "charge.updated",
            CheckoutSessionAsyncPaymentFailed => "checkout.session.async_payment_failed",
            CheckoutSessionAsyncPaymentSucceeded => "checkout.session.async_payment_succeeded",
            CheckoutSessionCompleted => "checkout.session.completed",
            CheckoutSessionExpired => "checkout.session.expired",
            ClimateOrderCanceled => "climate.order.canceled",
            ClimateOrderCreated => "climate.order.created",
            ClimateOrderDelayed => "climate.order.delayed",
            ClimateOrderDelivered => "climate.order.delivered",
            ClimateOrderProductSubstituted => "climate.order.product_substituted",
            ClimateProductCreated => "climate.product.created",
            ClimateProductPricingUpdated => "climate.product.pricing_updated",
            CouponCreated => "coupon.created",
            CouponDeleted => "coupon.deleted",
            CouponUpdated => "coupon.updated",
            CreditNoteCreated => "credit_note.created",
            CreditNoteUpdated => "credit_note.updated",
            CreditNoteVoided => "credit_note.voided",
            CustomerCreated => "customer.created",
            CustomerDeleted => "customer.deleted",
            CustomerDiscountCreated => "customer.discount.created",
            CustomerDiscountDeleted => "customer.discount.deleted",
            CustomerDiscountUpdated => "customer.discount.updated",
            CustomerSourceCreated => "customer.source.created",
            CustomerSourceDeleted => "customer.source.deleted",
            CustomerSourceExpiring => "customer.source.expiring",
            CustomerSourceUpdated => "customer.source.updated",
            CustomerSubscriptionCreated => "customer.subscription.created",
            CustomerSubscriptionDeleted => "customer.subscription.deleted",
            CustomerSubscriptionPaused => "customer.subscription.paused",
            CustomerSubscriptionPendingUpdateApplied => {
                "customer.subscription.pending_update_applied"
            }
            CustomerSubscriptionPendingUpdateExpired => {
                "customer.subscription.pending_update_expired"
            }
            CustomerSubscriptionResumed => "customer.subscription.resumed",
            CustomerSubscriptionTrialWillEnd => "customer.subscription.trial_will_end",
            CustomerSubscriptionUpdated => "customer.subscription.updated",
            CustomerTaxIdCreated => "customer.tax_id.created",
            CustomerTaxIdDeleted => "customer.tax_id.deleted",
            CustomerTaxIdUpdated => "customer.tax_id.updated",
            CustomerUpdated => "customer.updated",
            CustomerCashBalanceTransactionCreated => "customer_cash_balance_transaction.created",
            EntitlementsActiveEntitlementSummaryUpdated => {
                "entitlements.active_entitlement_summary.updated"
            }
            FileCreated => "file.created",
            FinancialConnectionsAccountCreated => "financial_connections.account.created",
            FinancialConnectionsAccountDeactivated => "financial_connections.account.deactivated",
            FinancialConnectionsAccountDisconnected => "financial_connections.account.disconnected",
            FinancialConnectionsAccountReactivated => "financial_connections.account.reactivated",
            FinancialConnectionsAccountRefreshedBalance => {
                "financial_connections.account.refreshed_balance"
            }
            FinancialConnectionsAccountRefreshedOwnership => {
                "financial_connections.account.refreshed_ownership"
            }
            FinancialConnectionsAccountRefreshedTransactions => {
                "financial_connections.account.refreshed_transactions"
            }
            IdentityVerificationSessionCanceled => "identity.verification_session.canceled",
            IdentityVerificationSessionCreated => "identity.verification_session.created",
            IdentityVerificationSessionProcessing => "identity.verification_session.processing",
            IdentityVerificationSessionRedacted => "identity.verification_session.redacted",
            IdentityVerificationSessionRequiresInput => {
                "identity.verification_session.requires_input"
            }
            IdentityVerificationSessionVerified => "identity.verification_session.verified",
            InvoiceCreated => "invoice.created",
            InvoiceDeleted => "invoice.deleted",
            InvoiceFinalizationFailed => "invoice.finalization_failed",
            InvoiceFinalized => "invoice.finalized",
            InvoiceMarkedUncollectible => "invoice.marked_uncollectible",
            InvoicePaid => "invoice.paid",
            InvoicePaymentActionRequired => "invoice.payment_action_required",
            InvoicePaymentFailed => "invoice.payment_failed",
            InvoicePaymentSucceeded => "invoice.payment_succeeded",
            InvoiceSent => "invoice.sent",
            InvoiceUpcoming => "invoice.upcoming",
            InvoiceUpdated => "invoice.updated",
            InvoiceVoided => "invoice.voided",
            InvoiceitemCreated => "invoiceitem.created",
            InvoiceitemDeleted => "invoiceitem.deleted",
            IssuingAuthorizationCreated => "issuing_authorization.created",
            IssuingAuthorizationRequest => "issuing_authorization.request",
            IssuingAuthorizationUpdated => "issuing_authorization.updated",
            IssuingCardCreated => "issuing_card.created",
            IssuingCardUpdated => "issuing_card.updated",
            IssuingCardholderCreated => "issuing_cardholder.created",
            IssuingCardholderUpdated => "issuing_cardholder.updated",
            IssuingDisputeClosed => "issuing_dispute.closed",
            IssuingDisputeCreated => "issuing_dispute.created",
            IssuingDisputeFundsReinstated => "issuing_dispute.funds_reinstated",
            IssuingDisputeSubmitted => "issuing_dispute.submitted",
            IssuingDisputeUpdated => "issuing_dispute.updated",
            IssuingTokenCreated => "issuing_token.created",
            IssuingTokenUpdated => "issuing_token.updated",
            IssuingTransactionCreated => "issuing_transaction.created",
            IssuingTransactionUpdated => "issuing_transaction.updated",
            MandateUpdated => "mandate.updated",
            PaymentIntentAmountCapturableUpdated => "payment_intent.amount_capturable_updated",
            PaymentIntentCanceled => "payment_intent.canceled",
            PaymentIntentCreated => "payment_intent.created",
            PaymentIntentPartiallyFunded => "payment_intent.partially_funded",
            PaymentIntentPaymentFailed => "payment_intent.payment_failed",
            PaymentIntentProcessing => "payment_intent.processing",
            PaymentIntentRequiresAction => "payment_intent.requires_action",
            PaymentIntentSucceeded => "payment_intent.succeeded",
            PaymentLinkCreated => "payment_link.created",
            PaymentLinkUpdated => "payment_link.updated",
            PaymentMethodAttached => "payment_method.attached",
            PaymentMethodAutomaticallyUpdated => "payment_method.automatically_updated",
            PaymentMethodDetached => "payment_method.detached",
            PaymentMethodUpdated => "payment_method.updated",
            PayoutCanceled => "payout.canceled",
            PayoutCreated => "payout.created",
            PayoutFailed => "payout.failed",
            PayoutPaid => "payout.paid",
            PayoutReconciliationCompleted => "payout.reconciliation_completed",
            PayoutUpdated => "payout.updated",
            PersonCreated => "person.created",
            PersonDeleted => "person.deleted",
            PersonUpdated => "person.updated",
            PlanCreated => "plan.created",
            PlanDeleted => "plan.deleted",
            PlanUpdated => "plan.updated",
            PriceCreated => "price.created",
            PriceDeleted => "price.deleted",
            PriceUpdated => "price.updated",
            ProductCreated => "product.created",
            ProductDeleted => "product.deleted",
            ProductUpdated => "product.updated",
            PromotionCodeCreated => "promotion_code.created",
            PromotionCodeUpdated => "promotion_code.updated",
            QuoteAccepted => "quote.accepted",
            QuoteCanceled => "quote.canceled",
            QuoteCreated => "quote.created",
            QuoteFinalized => "quote.finalized",
            RadarEarlyFraudWarningCreated => "radar.early_fraud_warning.created",
            RadarEarlyFraudWarningUpdated => "radar.early_fraud_warning.updated",
            RefundCreated => "refund.created",
            RefundUpdated => "refund.updated",
            ReportingReportRunFailed => "reporting.report_run.failed",
            ReportingReportRunSucceeded => "reporting.report_run.succeeded",
            ReportingReportTypeUpdated => "reporting.report_type.updated",
            ReviewClosed => "review.closed",
            ReviewOpened => "review.opened",
            SetupIntentCanceled => "setup_intent.canceled",
            SetupIntentCreated => "setup_intent.created",
            SetupIntentRequiresAction => "setup_intent.requires_action",
            SetupIntentSetupFailed => "setup_intent.setup_failed",
            SetupIntentSucceeded => "setup_intent.succeeded",
            SigmaScheduledQueryRunCreated => "sigma.scheduled_query_run.created",
            SourceCanceled => "source.canceled",
            SourceChargeable => "source.chargeable",
            SourceFailed => "source.failed",
            SourceMandateNotification => "source.mandate_notification",
            SourceRefundAttributesRequired => "source.refund_attributes_required",
            SourceTransactionCreated => "source.transaction.created",
            SourceTransactionUpdated => "source.transaction.updated",
            SubscriptionScheduleAborted => "subscription_schedule.aborted",
            SubscriptionScheduleCanceled => "subscription_schedule.canceled",
            SubscriptionScheduleCompleted => "subscription_schedule.completed",
            SubscriptionScheduleCreated => "subscription_schedule.created",
            SubscriptionScheduleExpiring => "subscription_schedule.expiring",
            SubscriptionScheduleReleased => "subscription_schedule.released",
            SubscriptionScheduleUpdated => "subscription_schedule.updated",
            TaxSettingsUpdated => "tax.settings.updated",
            TaxRateCreated => "tax_rate.created",
            TaxRateUpdated => "tax_rate.updated",
            TerminalReaderActionFailed => "terminal.reader.action_failed",
            TerminalReaderActionSucceeded => "terminal.reader.action_succeeded",
            TestHelpersTestClockAdvancing => "test_helpers.test_clock.advancing",
            TestHelpersTestClockCreated => "test_helpers.test_clock.created",
            TestHelpersTestClockDeleted => "test_helpers.test_clock.deleted",
            TestHelpersTestClockInternalFailure => "test_helpers.test_clock.internal_failure",
            TestHelpersTestClockReady => "test_helpers.test_clock.ready",
            TopupCanceled => "topup.canceled",
            TopupCreated => "topup.created",
            TopupFailed => "topup.failed",
            TopupReversed => "topup.reversed",
            TopupSucceeded => "topup.succeeded",
            TransferCreated => "transfer.created",
            TransferReversed => "transfer.reversed",
            TransferUpdated => "transfer.updated",
            TreasuryCreditReversalCreated => "treasury.credit_reversal.created",
            TreasuryCreditReversalPosted => "treasury.credit_reversal.posted",
            TreasuryDebitReversalCompleted => "treasury.debit_reversal.completed",
            TreasuryDebitReversalCreated => "treasury.debit_reversal.created",
            TreasuryDebitReversalInitialCreditGranted => {
                "treasury.debit_reversal.initial_credit_granted"
            }
            TreasuryFinancialAccountClosed => "treasury.financial_account.closed",
            TreasuryFinancialAccountCreated => "treasury.financial_account.created",
            TreasuryFinancialAccountFeaturesStatusUpdated => {
                "treasury.financial_account.features_status_updated"
            }
            TreasuryInboundTransferCanceled => "treasury.inbound_transfer.canceled",
            TreasuryInboundTransferCreated => "treasury.inbound_transfer.created",
            TreasuryInboundTransferFailed => "treasury.inbound_transfer.failed",
            TreasuryInboundTransferSucceeded => "treasury.inbound_transfer.succeeded",
            TreasuryOutboundPaymentCanceled => "treasury.outbound_payment.canceled",
            TreasuryOutboundPaymentCreated => "treasury.outbound_payment.created",
            TreasuryOutboundPaymentExpectedArrivalDateUpdated => {
                "treasury.outbound_payment.expected_arrival_date_updated"
            }
            TreasuryOutboundPaymentFailed => "treasury.outbound_payment.failed",
            TreasuryOutboundPaymentPosted => "treasury.outbound_payment.posted",
            TreasuryOutboundPaymentReturned => "treasury.outbound_payment.returned",
            TreasuryOutboundTransferCanceled => "treasury.outbound_transfer.canceled",
            TreasuryOutboundTransferCreated => "treasury.outbound_transfer.created",
            TreasuryOutboundTransferExpectedArrivalDateUpdated => {
                "treasury.outbound_transfer.expected_arrival_date_updated"
            }
            TreasuryOutboundTransferFailed => "treasury.outbound_transfer.failed",
            TreasuryOutboundTransferPosted => "treasury.outbound_transfer.posted",
            TreasuryOutboundTransferReturned => "treasury.outbound_transfer.returned",
            TreasuryReceivedCreditCreated => "treasury.received_credit.created",
            TreasuryReceivedCreditFailed => "treasury.received_credit.failed",
            TreasuryReceivedCreditSucceeded => "treasury.received_credit.succeeded",
            TreasuryReceivedDebitCreated => "treasury.received_debit.created",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateWebhookEndpointEnabledEvents {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateWebhookEndpointEnabledEvents::*;
        match s {
            "*" => Ok(All),
            "account.application.authorized" => Ok(AccountApplicationAuthorized),
            "account.application.deauthorized" => Ok(AccountApplicationDeauthorized),
            "account.external_account.created" => Ok(AccountExternalAccountCreated),
            "account.external_account.deleted" => Ok(AccountExternalAccountDeleted),
            "account.external_account.updated" => Ok(AccountExternalAccountUpdated),
            "account.updated" => Ok(AccountUpdated),
            "application_fee.created" => Ok(ApplicationFeeCreated),
            "application_fee.refund.updated" => Ok(ApplicationFeeRefundUpdated),
            "application_fee.refunded" => Ok(ApplicationFeeRefunded),
            "balance.available" => Ok(BalanceAvailable),
            "billing_portal.configuration.created" => Ok(BillingPortalConfigurationCreated),
            "billing_portal.configuration.updated" => Ok(BillingPortalConfigurationUpdated),
            "billing_portal.session.created" => Ok(BillingPortalSessionCreated),
            "capability.updated" => Ok(CapabilityUpdated),
            "cash_balance.funds_available" => Ok(CashBalanceFundsAvailable),
            "charge.captured" => Ok(ChargeCaptured),
            "charge.dispute.closed" => Ok(ChargeDisputeClosed),
            "charge.dispute.created" => Ok(ChargeDisputeCreated),
            "charge.dispute.funds_reinstated" => Ok(ChargeDisputeFundsReinstated),
            "charge.dispute.funds_withdrawn" => Ok(ChargeDisputeFundsWithdrawn),
            "charge.dispute.updated" => Ok(ChargeDisputeUpdated),
            "charge.expired" => Ok(ChargeExpired),
            "charge.failed" => Ok(ChargeFailed),
            "charge.pending" => Ok(ChargePending),
            "charge.refund.updated" => Ok(ChargeRefundUpdated),
            "charge.refunded" => Ok(ChargeRefunded),
            "charge.succeeded" => Ok(ChargeSucceeded),
            "charge.updated" => Ok(ChargeUpdated),
            "checkout.session.async_payment_failed" => Ok(CheckoutSessionAsyncPaymentFailed),
            "checkout.session.async_payment_succeeded" => Ok(CheckoutSessionAsyncPaymentSucceeded),
            "checkout.session.completed" => Ok(CheckoutSessionCompleted),
            "checkout.session.expired" => Ok(CheckoutSessionExpired),
            "climate.order.canceled" => Ok(ClimateOrderCanceled),
            "climate.order.created" => Ok(ClimateOrderCreated),
            "climate.order.delayed" => Ok(ClimateOrderDelayed),
            "climate.order.delivered" => Ok(ClimateOrderDelivered),
            "climate.order.product_substituted" => Ok(ClimateOrderProductSubstituted),
            "climate.product.created" => Ok(ClimateProductCreated),
            "climate.product.pricing_updated" => Ok(ClimateProductPricingUpdated),
            "coupon.created" => Ok(CouponCreated),
            "coupon.deleted" => Ok(CouponDeleted),
            "coupon.updated" => Ok(CouponUpdated),
            "credit_note.created" => Ok(CreditNoteCreated),
            "credit_note.updated" => Ok(CreditNoteUpdated),
            "credit_note.voided" => Ok(CreditNoteVoided),
            "customer.created" => Ok(CustomerCreated),
            "customer.deleted" => Ok(CustomerDeleted),
            "customer.discount.created" => Ok(CustomerDiscountCreated),
            "customer.discount.deleted" => Ok(CustomerDiscountDeleted),
            "customer.discount.updated" => Ok(CustomerDiscountUpdated),
            "customer.source.created" => Ok(CustomerSourceCreated),
            "customer.source.deleted" => Ok(CustomerSourceDeleted),
            "customer.source.expiring" => Ok(CustomerSourceExpiring),
            "customer.source.updated" => Ok(CustomerSourceUpdated),
            "customer.subscription.created" => Ok(CustomerSubscriptionCreated),
            "customer.subscription.deleted" => Ok(CustomerSubscriptionDeleted),
            "customer.subscription.paused" => Ok(CustomerSubscriptionPaused),
            "customer.subscription.pending_update_applied" => {
                Ok(CustomerSubscriptionPendingUpdateApplied)
            }
            "customer.subscription.pending_update_expired" => {
                Ok(CustomerSubscriptionPendingUpdateExpired)
            }
            "customer.subscription.resumed" => Ok(CustomerSubscriptionResumed),
            "customer.subscription.trial_will_end" => Ok(CustomerSubscriptionTrialWillEnd),
            "customer.subscription.updated" => Ok(CustomerSubscriptionUpdated),
            "customer.tax_id.created" => Ok(CustomerTaxIdCreated),
            "customer.tax_id.deleted" => Ok(CustomerTaxIdDeleted),
            "customer.tax_id.updated" => Ok(CustomerTaxIdUpdated),
            "customer.updated" => Ok(CustomerUpdated),
            "customer_cash_balance_transaction.created" => {
                Ok(CustomerCashBalanceTransactionCreated)
            }
            "entitlements.active_entitlement_summary.updated" => {
                Ok(EntitlementsActiveEntitlementSummaryUpdated)
            }
            "file.created" => Ok(FileCreated),
            "financial_connections.account.created" => Ok(FinancialConnectionsAccountCreated),
            "financial_connections.account.deactivated" => {
                Ok(FinancialConnectionsAccountDeactivated)
            }
            "financial_connections.account.disconnected" => {
                Ok(FinancialConnectionsAccountDisconnected)
            }
            "financial_connections.account.reactivated" => {
                Ok(FinancialConnectionsAccountReactivated)
            }
            "financial_connections.account.refreshed_balance" => {
                Ok(FinancialConnectionsAccountRefreshedBalance)
            }
            "financial_connections.account.refreshed_ownership" => {
                Ok(FinancialConnectionsAccountRefreshedOwnership)
            }
            "financial_connections.account.refreshed_transactions" => {
                Ok(FinancialConnectionsAccountRefreshedTransactions)
            }
            "identity.verification_session.canceled" => Ok(IdentityVerificationSessionCanceled),
            "identity.verification_session.created" => Ok(IdentityVerificationSessionCreated),
            "identity.verification_session.processing" => Ok(IdentityVerificationSessionProcessing),
            "identity.verification_session.redacted" => Ok(IdentityVerificationSessionRedacted),
            "identity.verification_session.requires_input" => {
                Ok(IdentityVerificationSessionRequiresInput)
            }
            "identity.verification_session.verified" => Ok(IdentityVerificationSessionVerified),
            "invoice.created" => Ok(InvoiceCreated),
            "invoice.deleted" => Ok(InvoiceDeleted),
            "invoice.finalization_failed" => Ok(InvoiceFinalizationFailed),
            "invoice.finalized" => Ok(InvoiceFinalized),
            "invoice.marked_uncollectible" => Ok(InvoiceMarkedUncollectible),
            "invoice.paid" => Ok(InvoicePaid),
            "invoice.payment_action_required" => Ok(InvoicePaymentActionRequired),
            "invoice.payment_failed" => Ok(InvoicePaymentFailed),
            "invoice.payment_succeeded" => Ok(InvoicePaymentSucceeded),
            "invoice.sent" => Ok(InvoiceSent),
            "invoice.upcoming" => Ok(InvoiceUpcoming),
            "invoice.updated" => Ok(InvoiceUpdated),
            "invoice.voided" => Ok(InvoiceVoided),
            "invoiceitem.created" => Ok(InvoiceitemCreated),
            "invoiceitem.deleted" => Ok(InvoiceitemDeleted),
            "issuing_authorization.created" => Ok(IssuingAuthorizationCreated),
            "issuing_authorization.request" => Ok(IssuingAuthorizationRequest),
            "issuing_authorization.updated" => Ok(IssuingAuthorizationUpdated),
            "issuing_card.created" => Ok(IssuingCardCreated),
            "issuing_card.updated" => Ok(IssuingCardUpdated),
            "issuing_cardholder.created" => Ok(IssuingCardholderCreated),
            "issuing_cardholder.updated" => Ok(IssuingCardholderUpdated),
            "issuing_dispute.closed" => Ok(IssuingDisputeClosed),
            "issuing_dispute.created" => Ok(IssuingDisputeCreated),
            "issuing_dispute.funds_reinstated" => Ok(IssuingDisputeFundsReinstated),
            "issuing_dispute.submitted" => Ok(IssuingDisputeSubmitted),
            "issuing_dispute.updated" => Ok(IssuingDisputeUpdated),
            "issuing_token.created" => Ok(IssuingTokenCreated),
            "issuing_token.updated" => Ok(IssuingTokenUpdated),
            "issuing_transaction.created" => Ok(IssuingTransactionCreated),
            "issuing_transaction.updated" => Ok(IssuingTransactionUpdated),
            "mandate.updated" => Ok(MandateUpdated),
            "payment_intent.amount_capturable_updated" => Ok(PaymentIntentAmountCapturableUpdated),
            "payment_intent.canceled" => Ok(PaymentIntentCanceled),
            "payment_intent.created" => Ok(PaymentIntentCreated),
            "payment_intent.partially_funded" => Ok(PaymentIntentPartiallyFunded),
            "payment_intent.payment_failed" => Ok(PaymentIntentPaymentFailed),
            "payment_intent.processing" => Ok(PaymentIntentProcessing),
            "payment_intent.requires_action" => Ok(PaymentIntentRequiresAction),
            "payment_intent.succeeded" => Ok(PaymentIntentSucceeded),
            "payment_link.created" => Ok(PaymentLinkCreated),
            "payment_link.updated" => Ok(PaymentLinkUpdated),
            "payment_method.attached" => Ok(PaymentMethodAttached),
            "payment_method.automatically_updated" => Ok(PaymentMethodAutomaticallyUpdated),
            "payment_method.detached" => Ok(PaymentMethodDetached),
            "payment_method.updated" => Ok(PaymentMethodUpdated),
            "payout.canceled" => Ok(PayoutCanceled),
            "payout.created" => Ok(PayoutCreated),
            "payout.failed" => Ok(PayoutFailed),
            "payout.paid" => Ok(PayoutPaid),
            "payout.reconciliation_completed" => Ok(PayoutReconciliationCompleted),
            "payout.updated" => Ok(PayoutUpdated),
            "person.created" => Ok(PersonCreated),
            "person.deleted" => Ok(PersonDeleted),
            "person.updated" => Ok(PersonUpdated),
            "plan.created" => Ok(PlanCreated),
            "plan.deleted" => Ok(PlanDeleted),
            "plan.updated" => Ok(PlanUpdated),
            "price.created" => Ok(PriceCreated),
            "price.deleted" => Ok(PriceDeleted),
            "price.updated" => Ok(PriceUpdated),
            "product.created" => Ok(ProductCreated),
            "product.deleted" => Ok(ProductDeleted),
            "product.updated" => Ok(ProductUpdated),
            "promotion_code.created" => Ok(PromotionCodeCreated),
            "promotion_code.updated" => Ok(PromotionCodeUpdated),
            "quote.accepted" => Ok(QuoteAccepted),
            "quote.canceled" => Ok(QuoteCanceled),
            "quote.created" => Ok(QuoteCreated),
            "quote.finalized" => Ok(QuoteFinalized),
            "radar.early_fraud_warning.created" => Ok(RadarEarlyFraudWarningCreated),
            "radar.early_fraud_warning.updated" => Ok(RadarEarlyFraudWarningUpdated),
            "refund.created" => Ok(RefundCreated),
            "refund.updated" => Ok(RefundUpdated),
            "reporting.report_run.failed" => Ok(ReportingReportRunFailed),
            "reporting.report_run.succeeded" => Ok(ReportingReportRunSucceeded),
            "reporting.report_type.updated" => Ok(ReportingReportTypeUpdated),
            "review.closed" => Ok(ReviewClosed),
            "review.opened" => Ok(ReviewOpened),
            "setup_intent.canceled" => Ok(SetupIntentCanceled),
            "setup_intent.created" => Ok(SetupIntentCreated),
            "setup_intent.requires_action" => Ok(SetupIntentRequiresAction),
            "setup_intent.setup_failed" => Ok(SetupIntentSetupFailed),
            "setup_intent.succeeded" => Ok(SetupIntentSucceeded),
            "sigma.scheduled_query_run.created" => Ok(SigmaScheduledQueryRunCreated),
            "source.canceled" => Ok(SourceCanceled),
            "source.chargeable" => Ok(SourceChargeable),
            "source.failed" => Ok(SourceFailed),
            "source.mandate_notification" => Ok(SourceMandateNotification),
            "source.refund_attributes_required" => Ok(SourceRefundAttributesRequired),
            "source.transaction.created" => Ok(SourceTransactionCreated),
            "source.transaction.updated" => Ok(SourceTransactionUpdated),
            "subscription_schedule.aborted" => Ok(SubscriptionScheduleAborted),
            "subscription_schedule.canceled" => Ok(SubscriptionScheduleCanceled),
            "subscription_schedule.completed" => Ok(SubscriptionScheduleCompleted),
            "subscription_schedule.created" => Ok(SubscriptionScheduleCreated),
            "subscription_schedule.expiring" => Ok(SubscriptionScheduleExpiring),
            "subscription_schedule.released" => Ok(SubscriptionScheduleReleased),
            "subscription_schedule.updated" => Ok(SubscriptionScheduleUpdated),
            "tax.settings.updated" => Ok(TaxSettingsUpdated),
            "tax_rate.created" => Ok(TaxRateCreated),
            "tax_rate.updated" => Ok(TaxRateUpdated),
            "terminal.reader.action_failed" => Ok(TerminalReaderActionFailed),
            "terminal.reader.action_succeeded" => Ok(TerminalReaderActionSucceeded),
            "test_helpers.test_clock.advancing" => Ok(TestHelpersTestClockAdvancing),
            "test_helpers.test_clock.created" => Ok(TestHelpersTestClockCreated),
            "test_helpers.test_clock.deleted" => Ok(TestHelpersTestClockDeleted),
            "test_helpers.test_clock.internal_failure" => Ok(TestHelpersTestClockInternalFailure),
            "test_helpers.test_clock.ready" => Ok(TestHelpersTestClockReady),
            "topup.canceled" => Ok(TopupCanceled),
            "topup.created" => Ok(TopupCreated),
            "topup.failed" => Ok(TopupFailed),
            "topup.reversed" => Ok(TopupReversed),
            "topup.succeeded" => Ok(TopupSucceeded),
            "transfer.created" => Ok(TransferCreated),
            "transfer.reversed" => Ok(TransferReversed),
            "transfer.updated" => Ok(TransferUpdated),
            "treasury.credit_reversal.created" => Ok(TreasuryCreditReversalCreated),
            "treasury.credit_reversal.posted" => Ok(TreasuryCreditReversalPosted),
            "treasury.debit_reversal.completed" => Ok(TreasuryDebitReversalCompleted),
            "treasury.debit_reversal.created" => Ok(TreasuryDebitReversalCreated),
            "treasury.debit_reversal.initial_credit_granted" => {
                Ok(TreasuryDebitReversalInitialCreditGranted)
            }
            "treasury.financial_account.closed" => Ok(TreasuryFinancialAccountClosed),
            "treasury.financial_account.created" => Ok(TreasuryFinancialAccountCreated),
            "treasury.financial_account.features_status_updated" => {
                Ok(TreasuryFinancialAccountFeaturesStatusUpdated)
            }
            "treasury.inbound_transfer.canceled" => Ok(TreasuryInboundTransferCanceled),
            "treasury.inbound_transfer.created" => Ok(TreasuryInboundTransferCreated),
            "treasury.inbound_transfer.failed" => Ok(TreasuryInboundTransferFailed),
            "treasury.inbound_transfer.succeeded" => Ok(TreasuryInboundTransferSucceeded),
            "treasury.outbound_payment.canceled" => Ok(TreasuryOutboundPaymentCanceled),
            "treasury.outbound_payment.created" => Ok(TreasuryOutboundPaymentCreated),
            "treasury.outbound_payment.expected_arrival_date_updated" => {
                Ok(TreasuryOutboundPaymentExpectedArrivalDateUpdated)
            }
            "treasury.outbound_payment.failed" => Ok(TreasuryOutboundPaymentFailed),
            "treasury.outbound_payment.posted" => Ok(TreasuryOutboundPaymentPosted),
            "treasury.outbound_payment.returned" => Ok(TreasuryOutboundPaymentReturned),
            "treasury.outbound_transfer.canceled" => Ok(TreasuryOutboundTransferCanceled),
            "treasury.outbound_transfer.created" => Ok(TreasuryOutboundTransferCreated),
            "treasury.outbound_transfer.expected_arrival_date_updated" => {
                Ok(TreasuryOutboundTransferExpectedArrivalDateUpdated)
            }
            "treasury.outbound_transfer.failed" => Ok(TreasuryOutboundTransferFailed),
            "treasury.outbound_transfer.posted" => Ok(TreasuryOutboundTransferPosted),
            "treasury.outbound_transfer.returned" => Ok(TreasuryOutboundTransferReturned),
            "treasury.received_credit.created" => Ok(TreasuryReceivedCreditCreated),
            "treasury.received_credit.failed" => Ok(TreasuryReceivedCreditFailed),
            "treasury.received_credit.succeeded" => Ok(TreasuryReceivedCreditSucceeded),
            "treasury.received_debit.created" => Ok(TreasuryReceivedDebitCreated),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for UpdateWebhookEndpointEnabledEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateWebhookEndpointEnabledEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateWebhookEndpointEnabledEvents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateWebhookEndpointEnabledEvents {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Updates the webhook endpoint.
/// You may edit the `url`, the list of `enabled_events`, and the status of your endpoint.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateWebhookEndpoint {
    inner: UpdateWebhookEndpointBuilder,
    webhook_endpoint: stripe_misc::WebhookEndpointId,
}
impl UpdateWebhookEndpoint {
    /// Construct a new `UpdateWebhookEndpoint`.
    pub fn new(webhook_endpoint: impl Into<stripe_misc::WebhookEndpointId>) -> Self {
        Self {
            webhook_endpoint: webhook_endpoint.into(),
            inner: UpdateWebhookEndpointBuilder::new(),
        }
    }
    /// An optional description of what the webhook is used for.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Disable the webhook endpoint if set to true.
    pub fn disabled(mut self, disabled: impl Into<bool>) -> Self {
        self.inner.disabled = Some(disabled.into());
        self
    }
    /// The list of events to enable for this endpoint.
    /// You may specify `['*']` to enable all events, except those that require explicit selection.
    pub fn enabled_events(
        mut self,
        enabled_events: impl Into<Vec<UpdateWebhookEndpointEnabledEvents>>,
    ) -> Self {
        self.inner.enabled_events = Some(enabled_events.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The URL of the webhook endpoint.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.inner.url = Some(url.into());
        self
    }
}
impl UpdateWebhookEndpoint {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateWebhookEndpoint {
    type Output = stripe_misc::WebhookEndpoint;

    fn build(&self) -> RequestBuilder {
        let webhook_endpoint = &self.webhook_endpoint;
        RequestBuilder::new(StripeMethod::Post, format!("/webhook_endpoints/{webhook_endpoint}"))
            .form(&self.inner)
    }
}
