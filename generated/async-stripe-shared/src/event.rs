/// Events are our way of letting you know when something interesting happens in
/// your account. When an interesting event occurs, we create a new `Event`
/// object. For example, when a charge succeeds, we create a `charge.succeeded`
/// event, and when an invoice payment attempt fails, we create an
/// `invoice.payment_failed` event. Certain API requests might create multiple
/// events. For example, if you create a new subscription for a
/// customer, you receive both a `customer.subscription.created` event and a
/// `charge.succeeded` event.
///
/// Events occur when the state of another API resource changes. The event's data
/// field embeds the resource's state at the time of the change. For
/// example, a `charge.succeeded` event contains a charge, and an
/// `invoice.payment_failed` event contains an invoice.
///
/// As with other API resources, you can use endpoints to retrieve an
/// [individual event](https://stripe.com/docs/api#retrieve_event) or a [list of events](https://stripe.com/docs/api#list_events).
/// from the API. We also have a separate
/// [webhooks](http://en.wikipedia.org/wiki/Webhook) system for sending the
/// `Event` objects directly to an endpoint on your server. You can manage
/// webhooks in your
/// [account settings](https://dashboard.stripe.com/account/webhooks). Learn how
/// to [listen for events](https://docs.stripe.com/webhooks)
/// so that your integration can automatically trigger reactions.
///
/// When using [Connect](https://docs.stripe.com/connect), you can also receive event notifications
/// that occur in connected accounts. For these events, there's an
/// additional `account` attribute in the received `Event` object.
///
/// We only guarantee access to events through the [Retrieve Event API](https://stripe.com/docs/api#retrieve_event).
/// for 30 days.
///
/// For more details see <<https://stripe.com/docs/api/events/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Event {
    /// The connected account that originates the event.
    pub account: Option<String>,
    /// The Stripe API version used to render `data`.
    /// This property is populated only for events on or after October 31, 2014.
    pub api_version: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub data: stripe_shared::NotificationEventData,
    /// Unique identifier for the object.
    pub id: stripe_shared::EventId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Number of webhooks that haven't been successfully delivered (for example, to return a 20x response) to the URLs you specify.
    pub pending_webhooks: i64,
    /// Information on the API request that triggers the event.
    pub request: Option<stripe_shared::NotificationEventRequest>,
    /// Description of the event (for example, `invoice.created` or `charge.refunded`).
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: EventType,
}
#[doc(hidden)]
pub struct EventBuilder {
    account: Option<Option<String>>,
    api_version: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    data: Option<stripe_shared::NotificationEventData>,
    id: Option<stripe_shared::EventId>,
    livemode: Option<bool>,
    pending_webhooks: Option<i64>,
    request: Option<Option<stripe_shared::NotificationEventRequest>>,
    type_: Option<EventType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Event {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Event>,
        builder: EventBuilder,
    }

    impl Visitor for Place<Event> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: EventBuilder::deser_default() }))
        }
    }

    impl MapBuilder for EventBuilder {
        type Out = Event;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "api_version" => Deserialize::begin(&mut self.api_version),
                "created" => Deserialize::begin(&mut self.created),
                "data" => Deserialize::begin(&mut self.data),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "pending_webhooks" => Deserialize::begin(&mut self.pending_webhooks),
                "request" => Deserialize::begin(&mut self.request),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                api_version: Deserialize::default(),
                created: Deserialize::default(),
                data: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                pending_webhooks: Deserialize::default(),
                request: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account: self.account.take()?,
                api_version: self.api_version.take()?,
                created: self.created?,
                data: self.data.take()?,
                id: self.id.take()?,
                livemode: self.livemode?,
                pending_webhooks: self.pending_webhooks?,
                request: self.request.take()?,
                type_: self.type_?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Event {
        type Builder = EventBuilder;
    }

    impl FromValueOpt for Event {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = EventBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = Some(FromValueOpt::from_value(v)?),
                    "api_version" => b.api_version = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "data" => b.data = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "pending_webhooks" => b.pending_webhooks = Some(FromValueOpt::from_value(v)?),
                    "request" => b.request = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Event {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Event", 10)?;
        s.serialize_field("account", &self.account)?;
        s.serialize_field("api_version", &self.api_version)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("pending_webhooks", &self.pending_webhooks)?;
        s.serialize_field("request", &self.request)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "event")?;
        s.end()
    }
}
/// Description of the event (for example, `invoice.created` or `charge.refunded`).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum EventType {
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
impl EventType {
    pub fn as_str(self) -> &'static str {
        use EventType::*;
        match self {
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

impl std::str::FromStr for EventType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EventType::*;
        match s {
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
impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for EventType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<EventType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(EventType::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(EventType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for EventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
impl stripe_types::Object for Event {
    type Id = stripe_shared::EventId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(EventId);
