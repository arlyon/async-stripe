use stripe::{Client, Response};

impl stripe_misc::webhook_endpoint::WebhookEndpoint {
    /// Returns a list of your webhook endpoints.
    pub fn list(
        client: &Client,
        params: ListWebhookEndpoint,
    ) -> Response<stripe_types::List<stripe_misc::webhook_endpoint::WebhookEndpoint>> {
        client.get_query("/webhook_endpoints", params)
    }
    /// Retrieves the webhook endpoint with the given ID.
    pub fn retrieve(
        client: &Client,
        webhook_endpoint: &stripe_misc::webhook_endpoint::WebhookEndpointId,
        params: RetrieveWebhookEndpoint,
    ) -> Response<stripe_misc::webhook_endpoint::WebhookEndpoint> {
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
    ) -> Response<stripe_misc::webhook_endpoint::WebhookEndpoint> {
        client.send_form("/webhook_endpoints", params, http_types::Method::Post)
    }
    /// Updates the webhook endpoint.
    ///
    /// You may edit the `url`, the list of `enabled_events`, and the status of your endpoint.
    pub fn update(
        client: &Client,
        webhook_endpoint: &stripe_misc::webhook_endpoint::WebhookEndpointId,
        params: UpdateWebhookEndpoint,
    ) -> Response<stripe_misc::webhook_endpoint::WebhookEndpoint> {
        client.send_form(
            &format!("/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = webhook_endpoint),
            params,
            http_types::Method::Post,
        )
    }
    /// You can also delete webhook endpoints via the [webhook endpoint management](https://dashboard.stripe.com/account/webhooks) page of the Stripe dashboard.
    pub fn delete(
        client: &Client,
        webhook_endpoint: &stripe_misc::webhook_endpoint::WebhookEndpointId,
    ) -> Response<stripe_misc::webhook_endpoint::DeletedWebhookEndpoint> {
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
    pub api_version: Option<stripe_types::ApiVersion>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
/// The list of events to enable for this endpoint.
///
/// You may specify `['*']` to enable all events, except those that require explicit selection.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    CustomerSubscriptionPendingUpdateApplied,
    CustomerSubscriptionPendingUpdateExpired,
    CustomerSubscriptionTrialWillEnd,
    CustomerSubscriptionUpdated,
    CustomerTaxIdCreated,
    CustomerTaxIdDeleted,
    CustomerTaxIdUpdated,
    CustomerUpdated,
    CustomerCashBalanceTransactionCreated,
    FileCreated,
    FinancialConnectionsAccountCreated,
    FinancialConnectionsAccountDeactivated,
    FinancialConnectionsAccountDisconnected,
    FinancialConnectionsAccountReactivated,
    FinancialConnectionsAccountRefreshedBalance,
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
    InvoiceitemUpdated,
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
    IssuingTransactionCreated,
    IssuingTransactionUpdated,
    MandateUpdated,
    OrderCreated,
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
    RecipientCreated,
    RecipientDeleted,
    RecipientUpdated,
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
    SkuCreated,
    SkuDeleted,
    SkuUpdated,
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

impl std::str::FromStr for CreateWebhookEndpointEnabledEvents {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::All),
            "account.application.authorized" => Ok(Self::AccountApplicationAuthorized),
            "account.application.deauthorized" => Ok(Self::AccountApplicationDeauthorized),
            "account.external_account.created" => Ok(Self::AccountExternalAccountCreated),
            "account.external_account.deleted" => Ok(Self::AccountExternalAccountDeleted),
            "account.external_account.updated" => Ok(Self::AccountExternalAccountUpdated),
            "account.updated" => Ok(Self::AccountUpdated),
            "application_fee.created" => Ok(Self::ApplicationFeeCreated),
            "application_fee.refund.updated" => Ok(Self::ApplicationFeeRefundUpdated),
            "application_fee.refunded" => Ok(Self::ApplicationFeeRefunded),
            "balance.available" => Ok(Self::BalanceAvailable),
            "billing_portal.configuration.created" => Ok(Self::BillingPortalConfigurationCreated),
            "billing_portal.configuration.updated" => Ok(Self::BillingPortalConfigurationUpdated),
            "billing_portal.session.created" => Ok(Self::BillingPortalSessionCreated),
            "capability.updated" => Ok(Self::CapabilityUpdated),
            "cash_balance.funds_available" => Ok(Self::CashBalanceFundsAvailable),
            "charge.captured" => Ok(Self::ChargeCaptured),
            "charge.dispute.closed" => Ok(Self::ChargeDisputeClosed),
            "charge.dispute.created" => Ok(Self::ChargeDisputeCreated),
            "charge.dispute.funds_reinstated" => Ok(Self::ChargeDisputeFundsReinstated),
            "charge.dispute.funds_withdrawn" => Ok(Self::ChargeDisputeFundsWithdrawn),
            "charge.dispute.updated" => Ok(Self::ChargeDisputeUpdated),
            "charge.expired" => Ok(Self::ChargeExpired),
            "charge.failed" => Ok(Self::ChargeFailed),
            "charge.pending" => Ok(Self::ChargePending),
            "charge.refund.updated" => Ok(Self::ChargeRefundUpdated),
            "charge.refunded" => Ok(Self::ChargeRefunded),
            "charge.succeeded" => Ok(Self::ChargeSucceeded),
            "charge.updated" => Ok(Self::ChargeUpdated),
            "checkout.session.async_payment_failed" => Ok(Self::CheckoutSessionAsyncPaymentFailed),
            "checkout.session.async_payment_succeeded" => {
                Ok(Self::CheckoutSessionAsyncPaymentSucceeded)
            }
            "checkout.session.completed" => Ok(Self::CheckoutSessionCompleted),
            "checkout.session.expired" => Ok(Self::CheckoutSessionExpired),
            "coupon.created" => Ok(Self::CouponCreated),
            "coupon.deleted" => Ok(Self::CouponDeleted),
            "coupon.updated" => Ok(Self::CouponUpdated),
            "credit_note.created" => Ok(Self::CreditNoteCreated),
            "credit_note.updated" => Ok(Self::CreditNoteUpdated),
            "credit_note.voided" => Ok(Self::CreditNoteVoided),
            "customer.created" => Ok(Self::CustomerCreated),
            "customer.deleted" => Ok(Self::CustomerDeleted),
            "customer.discount.created" => Ok(Self::CustomerDiscountCreated),
            "customer.discount.deleted" => Ok(Self::CustomerDiscountDeleted),
            "customer.discount.updated" => Ok(Self::CustomerDiscountUpdated),
            "customer.source.created" => Ok(Self::CustomerSourceCreated),
            "customer.source.deleted" => Ok(Self::CustomerSourceDeleted),
            "customer.source.expiring" => Ok(Self::CustomerSourceExpiring),
            "customer.source.updated" => Ok(Self::CustomerSourceUpdated),
            "customer.subscription.created" => Ok(Self::CustomerSubscriptionCreated),
            "customer.subscription.deleted" => Ok(Self::CustomerSubscriptionDeleted),
            "customer.subscription.pending_update_applied" => {
                Ok(Self::CustomerSubscriptionPendingUpdateApplied)
            }
            "customer.subscription.pending_update_expired" => {
                Ok(Self::CustomerSubscriptionPendingUpdateExpired)
            }
            "customer.subscription.trial_will_end" => Ok(Self::CustomerSubscriptionTrialWillEnd),
            "customer.subscription.updated" => Ok(Self::CustomerSubscriptionUpdated),
            "customer.tax_id.created" => Ok(Self::CustomerTaxIdCreated),
            "customer.tax_id.deleted" => Ok(Self::CustomerTaxIdDeleted),
            "customer.tax_id.updated" => Ok(Self::CustomerTaxIdUpdated),
            "customer.updated" => Ok(Self::CustomerUpdated),
            "customer_cash_balance_transaction.created" => {
                Ok(Self::CustomerCashBalanceTransactionCreated)
            }
            "file.created" => Ok(Self::FileCreated),
            "financial_connections.account.created" => Ok(Self::FinancialConnectionsAccountCreated),
            "financial_connections.account.deactivated" => {
                Ok(Self::FinancialConnectionsAccountDeactivated)
            }
            "financial_connections.account.disconnected" => {
                Ok(Self::FinancialConnectionsAccountDisconnected)
            }
            "financial_connections.account.reactivated" => {
                Ok(Self::FinancialConnectionsAccountReactivated)
            }
            "financial_connections.account.refreshed_balance" => {
                Ok(Self::FinancialConnectionsAccountRefreshedBalance)
            }
            "identity.verification_session.canceled" => {
                Ok(Self::IdentityVerificationSessionCanceled)
            }
            "identity.verification_session.created" => Ok(Self::IdentityVerificationSessionCreated),
            "identity.verification_session.processing" => {
                Ok(Self::IdentityVerificationSessionProcessing)
            }
            "identity.verification_session.redacted" => {
                Ok(Self::IdentityVerificationSessionRedacted)
            }
            "identity.verification_session.requires_input" => {
                Ok(Self::IdentityVerificationSessionRequiresInput)
            }
            "identity.verification_session.verified" => {
                Ok(Self::IdentityVerificationSessionVerified)
            }
            "invoice.created" => Ok(Self::InvoiceCreated),
            "invoice.deleted" => Ok(Self::InvoiceDeleted),
            "invoice.finalization_failed" => Ok(Self::InvoiceFinalizationFailed),
            "invoice.finalized" => Ok(Self::InvoiceFinalized),
            "invoice.marked_uncollectible" => Ok(Self::InvoiceMarkedUncollectible),
            "invoice.paid" => Ok(Self::InvoicePaid),
            "invoice.payment_action_required" => Ok(Self::InvoicePaymentActionRequired),
            "invoice.payment_failed" => Ok(Self::InvoicePaymentFailed),
            "invoice.payment_succeeded" => Ok(Self::InvoicePaymentSucceeded),
            "invoice.sent" => Ok(Self::InvoiceSent),
            "invoice.upcoming" => Ok(Self::InvoiceUpcoming),
            "invoice.updated" => Ok(Self::InvoiceUpdated),
            "invoice.voided" => Ok(Self::InvoiceVoided),
            "invoiceitem.created" => Ok(Self::InvoiceitemCreated),
            "invoiceitem.deleted" => Ok(Self::InvoiceitemDeleted),
            "invoiceitem.updated" => Ok(Self::InvoiceitemUpdated),
            "issuing_authorization.created" => Ok(Self::IssuingAuthorizationCreated),
            "issuing_authorization.request" => Ok(Self::IssuingAuthorizationRequest),
            "issuing_authorization.updated" => Ok(Self::IssuingAuthorizationUpdated),
            "issuing_card.created" => Ok(Self::IssuingCardCreated),
            "issuing_card.updated" => Ok(Self::IssuingCardUpdated),
            "issuing_cardholder.created" => Ok(Self::IssuingCardholderCreated),
            "issuing_cardholder.updated" => Ok(Self::IssuingCardholderUpdated),
            "issuing_dispute.closed" => Ok(Self::IssuingDisputeClosed),
            "issuing_dispute.created" => Ok(Self::IssuingDisputeCreated),
            "issuing_dispute.funds_reinstated" => Ok(Self::IssuingDisputeFundsReinstated),
            "issuing_dispute.submitted" => Ok(Self::IssuingDisputeSubmitted),
            "issuing_dispute.updated" => Ok(Self::IssuingDisputeUpdated),
            "issuing_transaction.created" => Ok(Self::IssuingTransactionCreated),
            "issuing_transaction.updated" => Ok(Self::IssuingTransactionUpdated),
            "mandate.updated" => Ok(Self::MandateUpdated),
            "order.created" => Ok(Self::OrderCreated),
            "payment_intent.amount_capturable_updated" => {
                Ok(Self::PaymentIntentAmountCapturableUpdated)
            }
            "payment_intent.canceled" => Ok(Self::PaymentIntentCanceled),
            "payment_intent.created" => Ok(Self::PaymentIntentCreated),
            "payment_intent.partially_funded" => Ok(Self::PaymentIntentPartiallyFunded),
            "payment_intent.payment_failed" => Ok(Self::PaymentIntentPaymentFailed),
            "payment_intent.processing" => Ok(Self::PaymentIntentProcessing),
            "payment_intent.requires_action" => Ok(Self::PaymentIntentRequiresAction),
            "payment_intent.succeeded" => Ok(Self::PaymentIntentSucceeded),
            "payment_link.created" => Ok(Self::PaymentLinkCreated),
            "payment_link.updated" => Ok(Self::PaymentLinkUpdated),
            "payment_method.attached" => Ok(Self::PaymentMethodAttached),
            "payment_method.automatically_updated" => Ok(Self::PaymentMethodAutomaticallyUpdated),
            "payment_method.detached" => Ok(Self::PaymentMethodDetached),
            "payment_method.updated" => Ok(Self::PaymentMethodUpdated),
            "payout.canceled" => Ok(Self::PayoutCanceled),
            "payout.created" => Ok(Self::PayoutCreated),
            "payout.failed" => Ok(Self::PayoutFailed),
            "payout.paid" => Ok(Self::PayoutPaid),
            "payout.updated" => Ok(Self::PayoutUpdated),
            "person.created" => Ok(Self::PersonCreated),
            "person.deleted" => Ok(Self::PersonDeleted),
            "person.updated" => Ok(Self::PersonUpdated),
            "plan.created" => Ok(Self::PlanCreated),
            "plan.deleted" => Ok(Self::PlanDeleted),
            "plan.updated" => Ok(Self::PlanUpdated),
            "price.created" => Ok(Self::PriceCreated),
            "price.deleted" => Ok(Self::PriceDeleted),
            "price.updated" => Ok(Self::PriceUpdated),
            "product.created" => Ok(Self::ProductCreated),
            "product.deleted" => Ok(Self::ProductDeleted),
            "product.updated" => Ok(Self::ProductUpdated),
            "promotion_code.created" => Ok(Self::PromotionCodeCreated),
            "promotion_code.updated" => Ok(Self::PromotionCodeUpdated),
            "quote.accepted" => Ok(Self::QuoteAccepted),
            "quote.canceled" => Ok(Self::QuoteCanceled),
            "quote.created" => Ok(Self::QuoteCreated),
            "quote.finalized" => Ok(Self::QuoteFinalized),
            "radar.early_fraud_warning.created" => Ok(Self::RadarEarlyFraudWarningCreated),
            "radar.early_fraud_warning.updated" => Ok(Self::RadarEarlyFraudWarningUpdated),
            "recipient.created" => Ok(Self::RecipientCreated),
            "recipient.deleted" => Ok(Self::RecipientDeleted),
            "recipient.updated" => Ok(Self::RecipientUpdated),
            "reporting.report_run.failed" => Ok(Self::ReportingReportRunFailed),
            "reporting.report_run.succeeded" => Ok(Self::ReportingReportRunSucceeded),
            "reporting.report_type.updated" => Ok(Self::ReportingReportTypeUpdated),
            "review.closed" => Ok(Self::ReviewClosed),
            "review.opened" => Ok(Self::ReviewOpened),
            "setup_intent.canceled" => Ok(Self::SetupIntentCanceled),
            "setup_intent.created" => Ok(Self::SetupIntentCreated),
            "setup_intent.requires_action" => Ok(Self::SetupIntentRequiresAction),
            "setup_intent.setup_failed" => Ok(Self::SetupIntentSetupFailed),
            "setup_intent.succeeded" => Ok(Self::SetupIntentSucceeded),
            "sigma.scheduled_query_run.created" => Ok(Self::SigmaScheduledQueryRunCreated),
            "sku.created" => Ok(Self::SkuCreated),
            "sku.deleted" => Ok(Self::SkuDeleted),
            "sku.updated" => Ok(Self::SkuUpdated),
            "source.canceled" => Ok(Self::SourceCanceled),
            "source.chargeable" => Ok(Self::SourceChargeable),
            "source.failed" => Ok(Self::SourceFailed),
            "source.mandate_notification" => Ok(Self::SourceMandateNotification),
            "source.refund_attributes_required" => Ok(Self::SourceRefundAttributesRequired),
            "source.transaction.created" => Ok(Self::SourceTransactionCreated),
            "source.transaction.updated" => Ok(Self::SourceTransactionUpdated),
            "subscription_schedule.aborted" => Ok(Self::SubscriptionScheduleAborted),
            "subscription_schedule.canceled" => Ok(Self::SubscriptionScheduleCanceled),
            "subscription_schedule.completed" => Ok(Self::SubscriptionScheduleCompleted),
            "subscription_schedule.created" => Ok(Self::SubscriptionScheduleCreated),
            "subscription_schedule.expiring" => Ok(Self::SubscriptionScheduleExpiring),
            "subscription_schedule.released" => Ok(Self::SubscriptionScheduleReleased),
            "subscription_schedule.updated" => Ok(Self::SubscriptionScheduleUpdated),
            "tax_rate.created" => Ok(Self::TaxRateCreated),
            "tax_rate.updated" => Ok(Self::TaxRateUpdated),
            "terminal.reader.action_failed" => Ok(Self::TerminalReaderActionFailed),
            "terminal.reader.action_succeeded" => Ok(Self::TerminalReaderActionSucceeded),
            "test_helpers.test_clock.advancing" => Ok(Self::TestHelpersTestClockAdvancing),
            "test_helpers.test_clock.created" => Ok(Self::TestHelpersTestClockCreated),
            "test_helpers.test_clock.deleted" => Ok(Self::TestHelpersTestClockDeleted),
            "test_helpers.test_clock.internal_failure" => {
                Ok(Self::TestHelpersTestClockInternalFailure)
            }
            "test_helpers.test_clock.ready" => Ok(Self::TestHelpersTestClockReady),
            "topup.canceled" => Ok(Self::TopupCanceled),
            "topup.created" => Ok(Self::TopupCreated),
            "topup.failed" => Ok(Self::TopupFailed),
            "topup.reversed" => Ok(Self::TopupReversed),
            "topup.succeeded" => Ok(Self::TopupSucceeded),
            "transfer.created" => Ok(Self::TransferCreated),
            "transfer.reversed" => Ok(Self::TransferReversed),
            "transfer.updated" => Ok(Self::TransferUpdated),
            "treasury.credit_reversal.created" => Ok(Self::TreasuryCreditReversalCreated),
            "treasury.credit_reversal.posted" => Ok(Self::TreasuryCreditReversalPosted),
            "treasury.debit_reversal.completed" => Ok(Self::TreasuryDebitReversalCompleted),
            "treasury.debit_reversal.created" => Ok(Self::TreasuryDebitReversalCreated),
            "treasury.debit_reversal.initial_credit_granted" => {
                Ok(Self::TreasuryDebitReversalInitialCreditGranted)
            }
            "treasury.financial_account.closed" => Ok(Self::TreasuryFinancialAccountClosed),
            "treasury.financial_account.created" => Ok(Self::TreasuryFinancialAccountCreated),
            "treasury.financial_account.features_status_updated" => {
                Ok(Self::TreasuryFinancialAccountFeaturesStatusUpdated)
            }
            "treasury.inbound_transfer.canceled" => Ok(Self::TreasuryInboundTransferCanceled),
            "treasury.inbound_transfer.created" => Ok(Self::TreasuryInboundTransferCreated),
            "treasury.inbound_transfer.failed" => Ok(Self::TreasuryInboundTransferFailed),
            "treasury.inbound_transfer.succeeded" => Ok(Self::TreasuryInboundTransferSucceeded),
            "treasury.outbound_payment.canceled" => Ok(Self::TreasuryOutboundPaymentCanceled),
            "treasury.outbound_payment.created" => Ok(Self::TreasuryOutboundPaymentCreated),
            "treasury.outbound_payment.expected_arrival_date_updated" => {
                Ok(Self::TreasuryOutboundPaymentExpectedArrivalDateUpdated)
            }
            "treasury.outbound_payment.failed" => Ok(Self::TreasuryOutboundPaymentFailed),
            "treasury.outbound_payment.posted" => Ok(Self::TreasuryOutboundPaymentPosted),
            "treasury.outbound_payment.returned" => Ok(Self::TreasuryOutboundPaymentReturned),
            "treasury.outbound_transfer.canceled" => Ok(Self::TreasuryOutboundTransferCanceled),
            "treasury.outbound_transfer.created" => Ok(Self::TreasuryOutboundTransferCreated),
            "treasury.outbound_transfer.expected_arrival_date_updated" => {
                Ok(Self::TreasuryOutboundTransferExpectedArrivalDateUpdated)
            }
            "treasury.outbound_transfer.failed" => Ok(Self::TreasuryOutboundTransferFailed),
            "treasury.outbound_transfer.posted" => Ok(Self::TreasuryOutboundTransferPosted),
            "treasury.outbound_transfer.returned" => Ok(Self::TreasuryOutboundTransferReturned),
            "treasury.received_credit.created" => Ok(Self::TreasuryReceivedCreditCreated),
            "treasury.received_credit.failed" => Ok(Self::TreasuryReceivedCreditFailed),
            "treasury.received_credit.succeeded" => Ok(Self::TreasuryReceivedCreditSucceeded),
            "treasury.received_debit.created" => Ok(Self::TreasuryReceivedDebitCreated),

            _ => Err(()),
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
impl serde::Serialize for CreateWebhookEndpointEnabledEvents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    CustomerSubscriptionPendingUpdateApplied,
    CustomerSubscriptionPendingUpdateExpired,
    CustomerSubscriptionTrialWillEnd,
    CustomerSubscriptionUpdated,
    CustomerTaxIdCreated,
    CustomerTaxIdDeleted,
    CustomerTaxIdUpdated,
    CustomerUpdated,
    CustomerCashBalanceTransactionCreated,
    FileCreated,
    FinancialConnectionsAccountCreated,
    FinancialConnectionsAccountDeactivated,
    FinancialConnectionsAccountDisconnected,
    FinancialConnectionsAccountReactivated,
    FinancialConnectionsAccountRefreshedBalance,
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
    InvoiceitemUpdated,
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
    IssuingTransactionCreated,
    IssuingTransactionUpdated,
    MandateUpdated,
    OrderCreated,
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
    RecipientCreated,
    RecipientDeleted,
    RecipientUpdated,
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
    SkuCreated,
    SkuDeleted,
    SkuUpdated,
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

impl std::str::FromStr for UpdateWebhookEndpointEnabledEvents {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::All),
            "account.application.authorized" => Ok(Self::AccountApplicationAuthorized),
            "account.application.deauthorized" => Ok(Self::AccountApplicationDeauthorized),
            "account.external_account.created" => Ok(Self::AccountExternalAccountCreated),
            "account.external_account.deleted" => Ok(Self::AccountExternalAccountDeleted),
            "account.external_account.updated" => Ok(Self::AccountExternalAccountUpdated),
            "account.updated" => Ok(Self::AccountUpdated),
            "application_fee.created" => Ok(Self::ApplicationFeeCreated),
            "application_fee.refund.updated" => Ok(Self::ApplicationFeeRefundUpdated),
            "application_fee.refunded" => Ok(Self::ApplicationFeeRefunded),
            "balance.available" => Ok(Self::BalanceAvailable),
            "billing_portal.configuration.created" => Ok(Self::BillingPortalConfigurationCreated),
            "billing_portal.configuration.updated" => Ok(Self::BillingPortalConfigurationUpdated),
            "billing_portal.session.created" => Ok(Self::BillingPortalSessionCreated),
            "capability.updated" => Ok(Self::CapabilityUpdated),
            "cash_balance.funds_available" => Ok(Self::CashBalanceFundsAvailable),
            "charge.captured" => Ok(Self::ChargeCaptured),
            "charge.dispute.closed" => Ok(Self::ChargeDisputeClosed),
            "charge.dispute.created" => Ok(Self::ChargeDisputeCreated),
            "charge.dispute.funds_reinstated" => Ok(Self::ChargeDisputeFundsReinstated),
            "charge.dispute.funds_withdrawn" => Ok(Self::ChargeDisputeFundsWithdrawn),
            "charge.dispute.updated" => Ok(Self::ChargeDisputeUpdated),
            "charge.expired" => Ok(Self::ChargeExpired),
            "charge.failed" => Ok(Self::ChargeFailed),
            "charge.pending" => Ok(Self::ChargePending),
            "charge.refund.updated" => Ok(Self::ChargeRefundUpdated),
            "charge.refunded" => Ok(Self::ChargeRefunded),
            "charge.succeeded" => Ok(Self::ChargeSucceeded),
            "charge.updated" => Ok(Self::ChargeUpdated),
            "checkout.session.async_payment_failed" => Ok(Self::CheckoutSessionAsyncPaymentFailed),
            "checkout.session.async_payment_succeeded" => {
                Ok(Self::CheckoutSessionAsyncPaymentSucceeded)
            }
            "checkout.session.completed" => Ok(Self::CheckoutSessionCompleted),
            "checkout.session.expired" => Ok(Self::CheckoutSessionExpired),
            "coupon.created" => Ok(Self::CouponCreated),
            "coupon.deleted" => Ok(Self::CouponDeleted),
            "coupon.updated" => Ok(Self::CouponUpdated),
            "credit_note.created" => Ok(Self::CreditNoteCreated),
            "credit_note.updated" => Ok(Self::CreditNoteUpdated),
            "credit_note.voided" => Ok(Self::CreditNoteVoided),
            "customer.created" => Ok(Self::CustomerCreated),
            "customer.deleted" => Ok(Self::CustomerDeleted),
            "customer.discount.created" => Ok(Self::CustomerDiscountCreated),
            "customer.discount.deleted" => Ok(Self::CustomerDiscountDeleted),
            "customer.discount.updated" => Ok(Self::CustomerDiscountUpdated),
            "customer.source.created" => Ok(Self::CustomerSourceCreated),
            "customer.source.deleted" => Ok(Self::CustomerSourceDeleted),
            "customer.source.expiring" => Ok(Self::CustomerSourceExpiring),
            "customer.source.updated" => Ok(Self::CustomerSourceUpdated),
            "customer.subscription.created" => Ok(Self::CustomerSubscriptionCreated),
            "customer.subscription.deleted" => Ok(Self::CustomerSubscriptionDeleted),
            "customer.subscription.pending_update_applied" => {
                Ok(Self::CustomerSubscriptionPendingUpdateApplied)
            }
            "customer.subscription.pending_update_expired" => {
                Ok(Self::CustomerSubscriptionPendingUpdateExpired)
            }
            "customer.subscription.trial_will_end" => Ok(Self::CustomerSubscriptionTrialWillEnd),
            "customer.subscription.updated" => Ok(Self::CustomerSubscriptionUpdated),
            "customer.tax_id.created" => Ok(Self::CustomerTaxIdCreated),
            "customer.tax_id.deleted" => Ok(Self::CustomerTaxIdDeleted),
            "customer.tax_id.updated" => Ok(Self::CustomerTaxIdUpdated),
            "customer.updated" => Ok(Self::CustomerUpdated),
            "customer_cash_balance_transaction.created" => {
                Ok(Self::CustomerCashBalanceTransactionCreated)
            }
            "file.created" => Ok(Self::FileCreated),
            "financial_connections.account.created" => Ok(Self::FinancialConnectionsAccountCreated),
            "financial_connections.account.deactivated" => {
                Ok(Self::FinancialConnectionsAccountDeactivated)
            }
            "financial_connections.account.disconnected" => {
                Ok(Self::FinancialConnectionsAccountDisconnected)
            }
            "financial_connections.account.reactivated" => {
                Ok(Self::FinancialConnectionsAccountReactivated)
            }
            "financial_connections.account.refreshed_balance" => {
                Ok(Self::FinancialConnectionsAccountRefreshedBalance)
            }
            "identity.verification_session.canceled" => {
                Ok(Self::IdentityVerificationSessionCanceled)
            }
            "identity.verification_session.created" => Ok(Self::IdentityVerificationSessionCreated),
            "identity.verification_session.processing" => {
                Ok(Self::IdentityVerificationSessionProcessing)
            }
            "identity.verification_session.redacted" => {
                Ok(Self::IdentityVerificationSessionRedacted)
            }
            "identity.verification_session.requires_input" => {
                Ok(Self::IdentityVerificationSessionRequiresInput)
            }
            "identity.verification_session.verified" => {
                Ok(Self::IdentityVerificationSessionVerified)
            }
            "invoice.created" => Ok(Self::InvoiceCreated),
            "invoice.deleted" => Ok(Self::InvoiceDeleted),
            "invoice.finalization_failed" => Ok(Self::InvoiceFinalizationFailed),
            "invoice.finalized" => Ok(Self::InvoiceFinalized),
            "invoice.marked_uncollectible" => Ok(Self::InvoiceMarkedUncollectible),
            "invoice.paid" => Ok(Self::InvoicePaid),
            "invoice.payment_action_required" => Ok(Self::InvoicePaymentActionRequired),
            "invoice.payment_failed" => Ok(Self::InvoicePaymentFailed),
            "invoice.payment_succeeded" => Ok(Self::InvoicePaymentSucceeded),
            "invoice.sent" => Ok(Self::InvoiceSent),
            "invoice.upcoming" => Ok(Self::InvoiceUpcoming),
            "invoice.updated" => Ok(Self::InvoiceUpdated),
            "invoice.voided" => Ok(Self::InvoiceVoided),
            "invoiceitem.created" => Ok(Self::InvoiceitemCreated),
            "invoiceitem.deleted" => Ok(Self::InvoiceitemDeleted),
            "invoiceitem.updated" => Ok(Self::InvoiceitemUpdated),
            "issuing_authorization.created" => Ok(Self::IssuingAuthorizationCreated),
            "issuing_authorization.request" => Ok(Self::IssuingAuthorizationRequest),
            "issuing_authorization.updated" => Ok(Self::IssuingAuthorizationUpdated),
            "issuing_card.created" => Ok(Self::IssuingCardCreated),
            "issuing_card.updated" => Ok(Self::IssuingCardUpdated),
            "issuing_cardholder.created" => Ok(Self::IssuingCardholderCreated),
            "issuing_cardholder.updated" => Ok(Self::IssuingCardholderUpdated),
            "issuing_dispute.closed" => Ok(Self::IssuingDisputeClosed),
            "issuing_dispute.created" => Ok(Self::IssuingDisputeCreated),
            "issuing_dispute.funds_reinstated" => Ok(Self::IssuingDisputeFundsReinstated),
            "issuing_dispute.submitted" => Ok(Self::IssuingDisputeSubmitted),
            "issuing_dispute.updated" => Ok(Self::IssuingDisputeUpdated),
            "issuing_transaction.created" => Ok(Self::IssuingTransactionCreated),
            "issuing_transaction.updated" => Ok(Self::IssuingTransactionUpdated),
            "mandate.updated" => Ok(Self::MandateUpdated),
            "order.created" => Ok(Self::OrderCreated),
            "payment_intent.amount_capturable_updated" => {
                Ok(Self::PaymentIntentAmountCapturableUpdated)
            }
            "payment_intent.canceled" => Ok(Self::PaymentIntentCanceled),
            "payment_intent.created" => Ok(Self::PaymentIntentCreated),
            "payment_intent.partially_funded" => Ok(Self::PaymentIntentPartiallyFunded),
            "payment_intent.payment_failed" => Ok(Self::PaymentIntentPaymentFailed),
            "payment_intent.processing" => Ok(Self::PaymentIntentProcessing),
            "payment_intent.requires_action" => Ok(Self::PaymentIntentRequiresAction),
            "payment_intent.succeeded" => Ok(Self::PaymentIntentSucceeded),
            "payment_link.created" => Ok(Self::PaymentLinkCreated),
            "payment_link.updated" => Ok(Self::PaymentLinkUpdated),
            "payment_method.attached" => Ok(Self::PaymentMethodAttached),
            "payment_method.automatically_updated" => Ok(Self::PaymentMethodAutomaticallyUpdated),
            "payment_method.detached" => Ok(Self::PaymentMethodDetached),
            "payment_method.updated" => Ok(Self::PaymentMethodUpdated),
            "payout.canceled" => Ok(Self::PayoutCanceled),
            "payout.created" => Ok(Self::PayoutCreated),
            "payout.failed" => Ok(Self::PayoutFailed),
            "payout.paid" => Ok(Self::PayoutPaid),
            "payout.updated" => Ok(Self::PayoutUpdated),
            "person.created" => Ok(Self::PersonCreated),
            "person.deleted" => Ok(Self::PersonDeleted),
            "person.updated" => Ok(Self::PersonUpdated),
            "plan.created" => Ok(Self::PlanCreated),
            "plan.deleted" => Ok(Self::PlanDeleted),
            "plan.updated" => Ok(Self::PlanUpdated),
            "price.created" => Ok(Self::PriceCreated),
            "price.deleted" => Ok(Self::PriceDeleted),
            "price.updated" => Ok(Self::PriceUpdated),
            "product.created" => Ok(Self::ProductCreated),
            "product.deleted" => Ok(Self::ProductDeleted),
            "product.updated" => Ok(Self::ProductUpdated),
            "promotion_code.created" => Ok(Self::PromotionCodeCreated),
            "promotion_code.updated" => Ok(Self::PromotionCodeUpdated),
            "quote.accepted" => Ok(Self::QuoteAccepted),
            "quote.canceled" => Ok(Self::QuoteCanceled),
            "quote.created" => Ok(Self::QuoteCreated),
            "quote.finalized" => Ok(Self::QuoteFinalized),
            "radar.early_fraud_warning.created" => Ok(Self::RadarEarlyFraudWarningCreated),
            "radar.early_fraud_warning.updated" => Ok(Self::RadarEarlyFraudWarningUpdated),
            "recipient.created" => Ok(Self::RecipientCreated),
            "recipient.deleted" => Ok(Self::RecipientDeleted),
            "recipient.updated" => Ok(Self::RecipientUpdated),
            "reporting.report_run.failed" => Ok(Self::ReportingReportRunFailed),
            "reporting.report_run.succeeded" => Ok(Self::ReportingReportRunSucceeded),
            "reporting.report_type.updated" => Ok(Self::ReportingReportTypeUpdated),
            "review.closed" => Ok(Self::ReviewClosed),
            "review.opened" => Ok(Self::ReviewOpened),
            "setup_intent.canceled" => Ok(Self::SetupIntentCanceled),
            "setup_intent.created" => Ok(Self::SetupIntentCreated),
            "setup_intent.requires_action" => Ok(Self::SetupIntentRequiresAction),
            "setup_intent.setup_failed" => Ok(Self::SetupIntentSetupFailed),
            "setup_intent.succeeded" => Ok(Self::SetupIntentSucceeded),
            "sigma.scheduled_query_run.created" => Ok(Self::SigmaScheduledQueryRunCreated),
            "sku.created" => Ok(Self::SkuCreated),
            "sku.deleted" => Ok(Self::SkuDeleted),
            "sku.updated" => Ok(Self::SkuUpdated),
            "source.canceled" => Ok(Self::SourceCanceled),
            "source.chargeable" => Ok(Self::SourceChargeable),
            "source.failed" => Ok(Self::SourceFailed),
            "source.mandate_notification" => Ok(Self::SourceMandateNotification),
            "source.refund_attributes_required" => Ok(Self::SourceRefundAttributesRequired),
            "source.transaction.created" => Ok(Self::SourceTransactionCreated),
            "source.transaction.updated" => Ok(Self::SourceTransactionUpdated),
            "subscription_schedule.aborted" => Ok(Self::SubscriptionScheduleAborted),
            "subscription_schedule.canceled" => Ok(Self::SubscriptionScheduleCanceled),
            "subscription_schedule.completed" => Ok(Self::SubscriptionScheduleCompleted),
            "subscription_schedule.created" => Ok(Self::SubscriptionScheduleCreated),
            "subscription_schedule.expiring" => Ok(Self::SubscriptionScheduleExpiring),
            "subscription_schedule.released" => Ok(Self::SubscriptionScheduleReleased),
            "subscription_schedule.updated" => Ok(Self::SubscriptionScheduleUpdated),
            "tax_rate.created" => Ok(Self::TaxRateCreated),
            "tax_rate.updated" => Ok(Self::TaxRateUpdated),
            "terminal.reader.action_failed" => Ok(Self::TerminalReaderActionFailed),
            "terminal.reader.action_succeeded" => Ok(Self::TerminalReaderActionSucceeded),
            "test_helpers.test_clock.advancing" => Ok(Self::TestHelpersTestClockAdvancing),
            "test_helpers.test_clock.created" => Ok(Self::TestHelpersTestClockCreated),
            "test_helpers.test_clock.deleted" => Ok(Self::TestHelpersTestClockDeleted),
            "test_helpers.test_clock.internal_failure" => {
                Ok(Self::TestHelpersTestClockInternalFailure)
            }
            "test_helpers.test_clock.ready" => Ok(Self::TestHelpersTestClockReady),
            "topup.canceled" => Ok(Self::TopupCanceled),
            "topup.created" => Ok(Self::TopupCreated),
            "topup.failed" => Ok(Self::TopupFailed),
            "topup.reversed" => Ok(Self::TopupReversed),
            "topup.succeeded" => Ok(Self::TopupSucceeded),
            "transfer.created" => Ok(Self::TransferCreated),
            "transfer.reversed" => Ok(Self::TransferReversed),
            "transfer.updated" => Ok(Self::TransferUpdated),
            "treasury.credit_reversal.created" => Ok(Self::TreasuryCreditReversalCreated),
            "treasury.credit_reversal.posted" => Ok(Self::TreasuryCreditReversalPosted),
            "treasury.debit_reversal.completed" => Ok(Self::TreasuryDebitReversalCompleted),
            "treasury.debit_reversal.created" => Ok(Self::TreasuryDebitReversalCreated),
            "treasury.debit_reversal.initial_credit_granted" => {
                Ok(Self::TreasuryDebitReversalInitialCreditGranted)
            }
            "treasury.financial_account.closed" => Ok(Self::TreasuryFinancialAccountClosed),
            "treasury.financial_account.created" => Ok(Self::TreasuryFinancialAccountCreated),
            "treasury.financial_account.features_status_updated" => {
                Ok(Self::TreasuryFinancialAccountFeaturesStatusUpdated)
            }
            "treasury.inbound_transfer.canceled" => Ok(Self::TreasuryInboundTransferCanceled),
            "treasury.inbound_transfer.created" => Ok(Self::TreasuryInboundTransferCreated),
            "treasury.inbound_transfer.failed" => Ok(Self::TreasuryInboundTransferFailed),
            "treasury.inbound_transfer.succeeded" => Ok(Self::TreasuryInboundTransferSucceeded),
            "treasury.outbound_payment.canceled" => Ok(Self::TreasuryOutboundPaymentCanceled),
            "treasury.outbound_payment.created" => Ok(Self::TreasuryOutboundPaymentCreated),
            "treasury.outbound_payment.expected_arrival_date_updated" => {
                Ok(Self::TreasuryOutboundPaymentExpectedArrivalDateUpdated)
            }
            "treasury.outbound_payment.failed" => Ok(Self::TreasuryOutboundPaymentFailed),
            "treasury.outbound_payment.posted" => Ok(Self::TreasuryOutboundPaymentPosted),
            "treasury.outbound_payment.returned" => Ok(Self::TreasuryOutboundPaymentReturned),
            "treasury.outbound_transfer.canceled" => Ok(Self::TreasuryOutboundTransferCanceled),
            "treasury.outbound_transfer.created" => Ok(Self::TreasuryOutboundTransferCreated),
            "treasury.outbound_transfer.expected_arrival_date_updated" => {
                Ok(Self::TreasuryOutboundTransferExpectedArrivalDateUpdated)
            }
            "treasury.outbound_transfer.failed" => Ok(Self::TreasuryOutboundTransferFailed),
            "treasury.outbound_transfer.posted" => Ok(Self::TreasuryOutboundTransferPosted),
            "treasury.outbound_transfer.returned" => Ok(Self::TreasuryOutboundTransferReturned),
            "treasury.received_credit.created" => Ok(Self::TreasuryReceivedCreditCreated),
            "treasury.received_credit.failed" => Ok(Self::TreasuryReceivedCreditFailed),
            "treasury.received_credit.succeeded" => Ok(Self::TreasuryReceivedCreditSucceeded),
            "treasury.received_debit.created" => Ok(Self::TreasuryReceivedDebitCreated),

            _ => Err(()),
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
impl serde::Serialize for UpdateWebhookEndpointEnabledEvents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
