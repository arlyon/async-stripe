#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum AccountExternalAccountCreated {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum AccountExternalAccountDeleted {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum AccountExternalAccountUpdated {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum CustomerSourceCreated {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum CustomerSourceDeleted {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum CustomerSourceExpiring {
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "object")]
pub enum CustomerSourceUpdated {
    #[serde(rename = "bank_account")]
    BankAccount(stripe_shared::BankAccount),
    #[serde(rename = "card")]
    Card(stripe_shared::Card),
    #[serde(rename = "source")]
    Source(stripe_shared::Source),
}
#[derive(Clone, Debug)]
#[non_exhaustive]
/// The event data for a webhook event.
pub enum EventObject {
    /// Occurs whenever a user authorizes an application. Sent to the related application only.
    AccountApplicationAuthorized(stripe_shared::Application),
    /// Occurs whenever a user deauthorizes an application. Sent to the related application only.
    AccountApplicationDeauthorized(stripe_shared::Application),
    /// Occurs whenever an external account is created.
    AccountExternalAccountCreated(AccountExternalAccountCreated),
    /// Occurs whenever an external account is deleted.
    AccountExternalAccountDeleted(AccountExternalAccountDeleted),
    /// Occurs whenever an external account is updated.
    AccountExternalAccountUpdated(AccountExternalAccountUpdated),
    /// Occurs whenever an account status or property has changed.
    AccountUpdated(stripe_shared::Account),
    /// Occurs whenever an application fee is created on a charge.
    ApplicationFeeCreated(stripe_shared::ApplicationFee),
    /// Occurs whenever an application fee refund is updated.
    ApplicationFeeRefundUpdated(stripe_shared::ApplicationFeeRefund),
    /// Occurs whenever an application fee is refunded, whether from refunding a charge or from [refunding the application fee directly](#fee_refunds).
    /// This includes partial refunds.
    ApplicationFeeRefunded(stripe_shared::ApplicationFee),
    /// Occurs whenever your Stripe balance has been updated (e.g., when a charge is available to be paid out).
    /// By default, Stripe automatically transfers funds in your balance to your bank account on a daily basis.
    /// This event is not fired for negative transactions.
    #[cfg(feature = "stripe_core")]
    BalanceAvailable(stripe_core::Balance),
    /// Occurs whenever a portal configuration is created.
    #[cfg(feature = "stripe_billing")]
    BillingPortalConfigurationCreated(stripe_billing::BillingPortalConfiguration),
    /// Occurs whenever a portal configuration is updated.
    #[cfg(feature = "stripe_billing")]
    BillingPortalConfigurationUpdated(stripe_billing::BillingPortalConfiguration),
    /// Occurs whenever a portal session is created.
    #[cfg(feature = "stripe_billing")]
    BillingPortalSessionCreated(stripe_billing::BillingPortalSession),
    /// Occurs whenever a capability has new requirements or a new status.
    CapabilityUpdated(stripe_shared::Capability),
    /// Occurs whenever there is a positive remaining cash balance after Stripe automatically reconciles new funds into the cash balance.
    /// If you enabled manual reconciliation, this webhook will fire whenever there are new funds into the cash balance.
    CashBalanceFundsAvailable(stripe_shared::CashBalance),
    /// Occurs whenever a previously uncaptured charge is captured.
    ChargeCaptured(stripe_shared::Charge),
    /// Occurs when a dispute is closed and the dispute status changes to `lost`, `warning_closed`, or `won`.
    ChargeDisputeClosed(stripe_shared::Dispute),
    /// Occurs whenever a customer disputes a charge with their bank.
    ChargeDisputeCreated(stripe_shared::Dispute),
    /// Occurs when funds are reinstated to your account after a dispute is closed.
    /// This includes [partially refunded payments](/docs/disputes#disputes-on-partially-refunded-payments).
    ChargeDisputeFundsReinstated(stripe_shared::Dispute),
    /// Occurs when funds are removed from your account due to a dispute.
    ChargeDisputeFundsWithdrawn(stripe_shared::Dispute),
    /// Occurs when the dispute is updated (usually with evidence).
    ChargeDisputeUpdated(stripe_shared::Dispute),
    /// Occurs whenever an uncaptured charge expires.
    ChargeExpired(stripe_shared::Charge),
    /// Occurs whenever a failed charge attempt occurs.
    ChargeFailed(stripe_shared::Charge),
    /// Occurs whenever a pending charge is created.
    ChargePending(stripe_shared::Charge),
    /// Occurs whenever a refund is updated, on selected payment methods.
    ChargeRefundUpdated(stripe_shared::Refund),
    /// Occurs whenever a charge is refunded, including partial refunds.
    ChargeRefunded(stripe_shared::Charge),
    /// Occurs whenever a charge is successful.
    ChargeSucceeded(stripe_shared::Charge),
    /// Occurs whenever a charge description or metadata is updated, or upon an asynchronous capture.
    ChargeUpdated(stripe_shared::Charge),
    /// Occurs when a payment intent using a delayed payment method fails.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionAsyncPaymentFailed(stripe_checkout::CheckoutSession),
    /// Occurs when a payment intent using a delayed payment method finally succeeds.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionAsyncPaymentSucceeded(stripe_checkout::CheckoutSession),
    /// Occurs when a Checkout Session has been successfully completed.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionCompleted(stripe_checkout::CheckoutSession),
    /// Occurs when a Checkout Session is expired.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionExpired(stripe_checkout::CheckoutSession),
    /// Occurs when a Climate order is canceled.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderCanceled(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is created.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderCreated(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is delayed.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderDelayed(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is delivered.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderDelivered(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order's product is substituted for another.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderProductSubstituted(stripe_misc::ClimateOrder),
    /// Occurs when a Climate product is created.
    #[cfg(feature = "stripe_misc")]
    ClimateProductCreated(stripe_misc::ClimateProduct),
    /// Occurs when a Climate product is updated.
    #[cfg(feature = "stripe_misc")]
    ClimateProductPricingUpdated(stripe_misc::ClimateProduct),
    /// Occurs whenever a coupon is created.
    CouponCreated(stripe_shared::Coupon),
    /// Occurs whenever a coupon is deleted.
    CouponDeleted(stripe_shared::Coupon),
    /// Occurs whenever a coupon is updated.
    CouponUpdated(stripe_shared::Coupon),
    /// Occurs whenever a credit note is created.
    CreditNoteCreated(stripe_shared::CreditNote),
    /// Occurs whenever a credit note is updated.
    CreditNoteUpdated(stripe_shared::CreditNote),
    /// Occurs whenever a credit note is voided.
    CreditNoteVoided(stripe_shared::CreditNote),
    /// Occurs whenever a new customer is created.
    CustomerCreated(stripe_shared::Customer),
    /// Occurs whenever a customer is deleted.
    CustomerDeleted(stripe_shared::Customer),
    /// Occurs whenever a coupon is attached to a customer.
    CustomerDiscountCreated(stripe_shared::Discount),
    /// Occurs whenever a coupon is removed from a customer.
    CustomerDiscountDeleted(stripe_shared::Discount),
    /// Occurs whenever a customer is switched from one coupon to another.
    CustomerDiscountUpdated(stripe_shared::Discount),
    /// Occurs whenever a new source is created for a customer.
    CustomerSourceCreated(CustomerSourceCreated),
    /// Occurs whenever a source is removed from a customer.
    CustomerSourceDeleted(CustomerSourceDeleted),
    /// Occurs whenever a card or source will expire at the end of the month.
    CustomerSourceExpiring(CustomerSourceExpiring),
    /// Occurs whenever a source's details are changed.
    CustomerSourceUpdated(CustomerSourceUpdated),
    /// Occurs whenever a customer is signed up for a new plan.
    CustomerSubscriptionCreated(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription ends.
    CustomerSubscriptionDeleted(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription is paused.
    /// Only applies when subscriptions enter `status=paused`, not when [payment collection](/docs/billing/subscriptions/pause) is paused.
    CustomerSubscriptionPaused(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription's pending update is applied, and the subscription is updated.
    CustomerSubscriptionPendingUpdateApplied(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription's pending update expires before the related invoice is paid.
    CustomerSubscriptionPendingUpdateExpired(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription is no longer paused.
    /// Only applies when a `status=paused` subscription is [resumed](/docs/api/subscriptions/resume), not when [payment collection](/docs/billing/subscriptions/pause) is resumed.
    CustomerSubscriptionResumed(stripe_shared::Subscription),
    /// Occurs three days before a subscription's trial period is scheduled to end, or when a trial is ended immediately (using `trial_end=now`).
    CustomerSubscriptionTrialWillEnd(stripe_shared::Subscription),
    /// Occurs whenever a subscription changes (e.g., switching from one plan to another, or changing the status from trial to active).
    CustomerSubscriptionUpdated(stripe_shared::Subscription),
    /// Occurs whenever a tax ID is created for a customer.
    CustomerTaxIdCreated(stripe_shared::TaxId),
    /// Occurs whenever a tax ID is deleted from a customer.
    CustomerTaxIdDeleted(stripe_shared::TaxId),
    /// Occurs whenever a customer's tax ID is updated.
    CustomerTaxIdUpdated(stripe_shared::TaxId),
    /// Occurs whenever any property of a customer changes.
    CustomerUpdated(stripe_shared::Customer),
    /// Occurs whenever a new customer cash balance transactions is created.
    CustomerCashBalanceTransactionCreated(stripe_shared::CustomerCashBalanceTransaction),
    /// Occurs whenever a new Stripe-generated file is available for your account.
    FileCreated(stripe_shared::File),
    /// Occurs when a new Financial Connections account is created.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountCreated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account's status is updated from `active` to `inactive`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountDeactivated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account is disconnected.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountDisconnected(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account's status is updated from `inactive` to `active`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountReactivated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `balance_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountRefreshedBalance(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `transaction_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountRefreshedTransactions(stripe_misc::FinancialConnectionsAccount),
    /// Occurs whenever a VerificationSession is canceled
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionCanceled(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession is created
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionCreated(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to processing
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionProcessing(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession is redacted.
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionRedacted(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to require user input
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionRequiresInput(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to verified
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionVerified(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a new invoice is created.
    /// To learn how webhooks can be used with this event, and how they can affect it, see [Using Webhooks with Subscriptions](/docs/subscriptions/webhooks).
    InvoiceCreated(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice is deleted.
    InvoiceDeleted(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice cannot be finalized.
    /// See the invoice’s [last finalization error](/docs/api/invoices/object#invoice_object-last_finalization_error) for details.
    InvoiceFinalizationFailed(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice is finalized and updated to be an open invoice.
    InvoiceFinalized(stripe_shared::Invoice),
    /// Occurs whenever an invoice is marked uncollectible.
    InvoiceMarkedUncollectible(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt succeeds or an invoice is marked as paid out-of-band.
    InvoicePaid(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt requires further user action to complete.
    InvoicePaymentActionRequired(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt fails, due either to a declined payment or to the lack of a stored payment method.
    InvoicePaymentFailed(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt succeeds.
    InvoicePaymentSucceeded(stripe_shared::Invoice),
    /// Occurs whenever an invoice email is sent out.
    InvoiceSent(stripe_shared::Invoice),
    /// Occurs X number of days before a subscription is scheduled to create an invoice that is automatically charged&mdash;where X is determined by your [subscriptions settings](https://dashboard.stripe.com/account/billing/automatic).
    /// Note: The received `Invoice` object will not have an invoice ID.
    InvoiceUpcoming(stripe_shared::Invoice),
    /// Occurs whenever an invoice changes (e.g., the invoice amount).
    InvoiceUpdated(stripe_shared::Invoice),
    /// Occurs whenever an invoice is voided.
    InvoiceVoided(stripe_shared::Invoice),
    /// Occurs whenever an invoice item is created.
    InvoiceitemCreated(stripe_shared::InvoiceItem),
    /// Occurs whenever an invoice item is deleted.
    InvoiceitemDeleted(stripe_shared::InvoiceItem),
    /// Occurs whenever an authorization is created.
    IssuingAuthorizationCreated(stripe_shared::IssuingAuthorization),
    /// Represents a synchronous request for authorization, see [Using your integration to handle authorization requests](/docs/issuing/purchases/authorizations#authorization-handling).
    IssuingAuthorizationRequest(stripe_shared::IssuingAuthorization),
    /// Occurs whenever an authorization is updated.
    IssuingAuthorizationUpdated(stripe_shared::IssuingAuthorization),
    /// Occurs whenever a card is created.
    IssuingCardCreated(stripe_shared::IssuingCard),
    /// Occurs whenever a card is updated.
    IssuingCardUpdated(stripe_shared::IssuingCard),
    /// Occurs whenever a cardholder is created.
    IssuingCardholderCreated(stripe_shared::IssuingCardholder),
    /// Occurs whenever a cardholder is updated.
    IssuingCardholderUpdated(stripe_shared::IssuingCardholder),
    /// Occurs whenever a dispute is won, lost or expired.
    IssuingDisputeClosed(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is created.
    IssuingDisputeCreated(stripe_shared::IssuingDispute),
    /// Occurs whenever funds are reinstated to your account for an Issuing dispute.
    IssuingDisputeFundsReinstated(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is submitted.
    IssuingDisputeSubmitted(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is updated.
    IssuingDisputeUpdated(stripe_shared::IssuingDispute),
    /// Occurs whenever an issuing digital wallet token is created.
    IssuingTokenCreated(stripe_shared::IssuingToken),
    /// Occurs whenever an issuing digital wallet token is updated.
    IssuingTokenUpdated(stripe_shared::IssuingToken),
    /// Occurs whenever an issuing transaction is created.
    IssuingTransactionCreated(stripe_shared::IssuingTransaction),
    /// Occurs whenever an issuing transaction is updated.
    IssuingTransactionUpdated(stripe_shared::IssuingTransaction),
    /// Occurs whenever a Mandate is updated.
    MandateUpdated(stripe_shared::Mandate),
    /// Occurs when a PaymentIntent has funds to be captured.
    /// Check the `amount_capturable` property on the PaymentIntent to determine the amount that can be captured.
    /// You may capture the PaymentIntent with an `amount_to_capture` value up to the specified amount.
    /// [Learn more about capturing PaymentIntents.](https://stripe.com/docs/api/payment_intents/capture).
    PaymentIntentAmountCapturableUpdated(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent is canceled.
    PaymentIntentCanceled(stripe_shared::PaymentIntent),
    /// Occurs when a new PaymentIntent is created.
    PaymentIntentCreated(stripe_shared::PaymentIntent),
    /// Occurs when funds are applied to a customer_balance PaymentIntent and the 'amount_remaining' changes.
    PaymentIntentPartiallyFunded(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has failed the attempt to create a payment method or a payment.
    PaymentIntentPaymentFailed(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has started processing.
    PaymentIntentProcessing(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent transitions to requires_action state
    PaymentIntentRequiresAction(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has successfully completed payment.
    PaymentIntentSucceeded(stripe_shared::PaymentIntent),
    /// Occurs when a payment link is created.
    PaymentLinkCreated(stripe_shared::PaymentLink),
    /// Occurs when a payment link is updated.
    PaymentLinkUpdated(stripe_shared::PaymentLink),
    /// Occurs whenever a new payment method is attached to a customer.
    PaymentMethodAttached(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method's details are automatically updated by the network.
    PaymentMethodAutomaticallyUpdated(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method is detached from a customer.
    PaymentMethodDetached(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method is updated via the [PaymentMethod update API](https://stripe.com/docs/api/payment_methods/update).
    PaymentMethodUpdated(stripe_shared::PaymentMethod),
    /// Occurs whenever a payout is canceled.
    PayoutCanceled(stripe_shared::Payout),
    /// Occurs whenever a payout is created.
    PayoutCreated(stripe_shared::Payout),
    /// Occurs whenever a payout attempt fails.
    PayoutFailed(stripe_shared::Payout),
    /// Occurs whenever a payout is *expected* to be available in the destination account.
    /// If the payout fails, a `payout.failed` notification is also sent, at a later time.
    PayoutPaid(stripe_shared::Payout),
    /// Occurs whenever balance transactions paid out in an automatic payout can be queried.
    PayoutReconciliationCompleted(stripe_shared::Payout),
    /// Occurs whenever a payout is updated.
    PayoutUpdated(stripe_shared::Payout),
    /// Occurs whenever a person associated with an account is created.
    PersonCreated(stripe_shared::Person),
    /// Occurs whenever a person associated with an account is deleted.
    PersonDeleted(stripe_shared::Person),
    /// Occurs whenever a person associated with an account is updated.
    PersonUpdated(stripe_shared::Person),
    /// Occurs whenever a plan is created.
    PlanCreated(stripe_shared::Plan),
    /// Occurs whenever a plan is deleted.
    PlanDeleted(stripe_shared::Plan),
    /// Occurs whenever a plan is updated.
    PlanUpdated(stripe_shared::Plan),
    /// Occurs whenever a price is created.
    PriceCreated(stripe_shared::Price),
    /// Occurs whenever a price is deleted.
    PriceDeleted(stripe_shared::Price),
    /// Occurs whenever a price is updated.
    PriceUpdated(stripe_shared::Price),
    /// Occurs whenever a product is created.
    ProductCreated(stripe_shared::Product),
    /// Occurs whenever a product is deleted.
    ProductDeleted(stripe_shared::Product),
    /// Occurs whenever a product is updated.
    ProductUpdated(stripe_shared::Product),
    /// Occurs whenever a promotion code is created.
    PromotionCodeCreated(stripe_shared::PromotionCode),
    /// Occurs whenever a promotion code is updated.
    PromotionCodeUpdated(stripe_shared::PromotionCode),
    /// Occurs whenever a quote is accepted.
    QuoteAccepted(stripe_shared::Quote),
    /// Occurs whenever a quote is canceled.
    QuoteCanceled(stripe_shared::Quote),
    /// Occurs whenever a quote is created.
    QuoteCreated(stripe_shared::Quote),
    /// Occurs whenever a quote is finalized.
    QuoteFinalized(stripe_shared::Quote),
    /// Occurs whenever an early fraud warning is created.
    #[cfg(feature = "stripe_fraud")]
    RadarEarlyFraudWarningCreated(stripe_fraud::RadarEarlyFraudWarning),
    /// Occurs whenever an early fraud warning is updated.
    #[cfg(feature = "stripe_fraud")]
    RadarEarlyFraudWarningUpdated(stripe_fraud::RadarEarlyFraudWarning),
    /// Occurs whenever a refund from a customer's cash balance is created.
    RefundCreated(stripe_shared::Refund),
    /// Occurs whenever a refund from a customer's cash balance is updated.
    RefundUpdated(stripe_shared::Refund),
    /// Occurs whenever a requested `ReportRun` failed to complete.
    #[cfg(feature = "stripe_misc")]
    ReportingReportRunFailed(stripe_misc::ReportingReportRun),
    /// Occurs whenever a requested `ReportRun` completed successfully.
    #[cfg(feature = "stripe_misc")]
    ReportingReportRunSucceeded(stripe_misc::ReportingReportRun),
    /// Occurs whenever a `ReportType` is updated (typically to indicate that a new day's data has come available).
    #[cfg(feature = "stripe_misc")]
    ReportingReportTypeUpdated(stripe_misc::ReportingReportType),
    /// Occurs whenever a review is closed.
    /// The review's `reason` field indicates why: `approved`, `disputed`, `refunded`, or `refunded_as_fraud`.
    ReviewClosed(stripe_shared::Review),
    /// Occurs whenever a review is opened.
    ReviewOpened(stripe_shared::Review),
    /// Occurs when a SetupIntent is canceled.
    SetupIntentCanceled(stripe_shared::SetupIntent),
    /// Occurs when a new SetupIntent is created.
    SetupIntentCreated(stripe_shared::SetupIntent),
    /// Occurs when a SetupIntent is in requires_action state.
    SetupIntentRequiresAction(stripe_shared::SetupIntent),
    /// Occurs when a SetupIntent has failed the attempt to setup a payment method.
    SetupIntentSetupFailed(stripe_shared::SetupIntent),
    /// Occurs when an SetupIntent has successfully setup a payment method.
    SetupIntentSucceeded(stripe_shared::SetupIntent),
    /// Occurs whenever a Sigma scheduled query run finishes.
    #[cfg(feature = "stripe_misc")]
    SigmaScheduledQueryRunCreated(stripe_misc::ScheduledQueryRun),
    /// Occurs whenever a source is canceled.
    SourceCanceled(stripe_shared::Source),
    /// Occurs whenever a source transitions to chargeable.
    SourceChargeable(stripe_shared::Source),
    /// Occurs whenever a source fails.
    SourceFailed(stripe_shared::Source),
    /// Occurs whenever a source mandate notification method is set to manual.
    #[cfg(feature = "stripe_payment")]
    SourceMandateNotification(stripe_payment::SourceMandateNotification),
    /// Occurs whenever the refund attributes are required on a receiver source to process a refund or a mispayment.
    SourceRefundAttributesRequired(stripe_shared::Source),
    /// Occurs whenever a source transaction is created.
    SourceTransactionCreated(stripe_shared::SourceTransaction),
    /// Occurs whenever a source transaction is updated.
    SourceTransactionUpdated(stripe_shared::SourceTransaction),
    /// Occurs whenever a subscription schedule is canceled due to the underlying subscription being canceled because of delinquency.
    SubscriptionScheduleAborted(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a subscription schedule is canceled.
    SubscriptionScheduleCanceled(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is completed.
    SubscriptionScheduleCompleted(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is created.
    SubscriptionScheduleCreated(stripe_shared::SubscriptionSchedule),
    /// Occurs 7 days before a subscription schedule will expire.
    SubscriptionScheduleExpiring(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is released.
    SubscriptionScheduleReleased(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a subscription schedule is updated.
    SubscriptionScheduleUpdated(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever tax settings is updated.
    #[cfg(feature = "stripe_misc")]
    TaxSettingsUpdated(stripe_misc::TaxSettings),
    /// Occurs whenever a new tax rate is created.
    TaxRateCreated(stripe_shared::TaxRate),
    /// Occurs whenever a tax rate is updated.
    TaxRateUpdated(stripe_shared::TaxRate),
    /// Occurs whenever an action sent to a Terminal reader failed.
    #[cfg(feature = "stripe_terminal")]
    TerminalReaderActionFailed(stripe_terminal::TerminalReader),
    /// Occurs whenever an action sent to a Terminal reader was successful.
    #[cfg(feature = "stripe_terminal")]
    TerminalReaderActionSucceeded(stripe_terminal::TerminalReader),
    /// Occurs whenever a test clock starts advancing.
    TestHelpersTestClockAdvancing(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock is created.
    TestHelpersTestClockCreated(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock is deleted.
    TestHelpersTestClockDeleted(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock fails to advance its frozen time.
    TestHelpersTestClockInternalFailure(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock transitions to a ready status.
    TestHelpersTestClockReady(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a top-up is canceled.
    TopupCanceled(stripe_shared::Topup),
    /// Occurs whenever a top-up is created.
    TopupCreated(stripe_shared::Topup),
    /// Occurs whenever a top-up fails.
    TopupFailed(stripe_shared::Topup),
    /// Occurs whenever a top-up is reversed.
    TopupReversed(stripe_shared::Topup),
    /// Occurs whenever a top-up succeeds.
    TopupSucceeded(stripe_shared::Topup),
    /// Occurs whenever a transfer is created.
    TransferCreated(stripe_shared::Transfer),
    /// Occurs whenever a transfer is reversed, including partial reversals.
    TransferReversed(stripe_shared::Transfer),
    /// Occurs whenever a transfer's description or metadata is updated.
    TransferUpdated(stripe_shared::Transfer),
    /// Occurs whenever an CreditReversal is submitted and created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryCreditReversalCreated(stripe_treasury::TreasuryCreditReversal),
    /// Occurs whenever an CreditReversal post is posted.
    #[cfg(feature = "stripe_treasury")]
    TreasuryCreditReversalPosted(stripe_treasury::TreasuryCreditReversal),
    /// Occurs whenever a DebitReversal is completed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryDebitReversalCompleted(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever a DebitReversal is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryDebitReversalCreated(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever an initial credit is granted on a DebitReversal.
    #[cfg(feature = "stripe_treasury")]
    TreasuryDebitReversalInitialCreditGranted(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever the status of the FinancialAccount becomes closed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryFinancialAccountClosed(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever a new FinancialAccount is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryFinancialAccountCreated(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever the statuses of any features within an existing FinancialAccount are updated.
    #[cfg(feature = "stripe_treasury")]
    TreasuryFinancialAccountFeaturesStatusUpdated(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever an InboundTransfer is canceled.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferCanceled(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferCreated(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer has failed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferFailed(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer has succeeded.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferSucceeded(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an OutboundPayment is canceled.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentCanceled(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever a new OutboundPayment is successfully created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentCreated(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever the arrival date on an OutboundPayment updates.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentExpectedArrivalDateUpdated(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment fails.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentFailed(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment posts.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentPosted(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment was returned.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentReturned(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundTransfer is canceled.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferCanceled(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferCreated(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever the arrival date on an OutboundTransfer updates.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferExpectedArrivalDateUpdated(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer has failed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferFailed(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is posted.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferPosted(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is returned.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferReturned(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever a received_credit is created as a result of funds being pushed by another account.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedCreditCreated(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_credit transitions to failed state. Only applicable for check deposits.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedCreditFailed(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_credit transitions to succeeded state.
    /// Only applicable for check deposits.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedCreditSucceeded(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_debit is created as a result of funds being pulled by another account.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedDebitCreated(stripe_treasury::TreasuryReceivedDebit),
    Unknown(serde_json::Value),
}
impl EventObject {
    pub(crate) fn from_raw_data(typ: &str, data: serde_json::Value) -> serde_json::Result<Self> {
        Ok(match typ {
            "account.application.authorized" => {
                EventObject::AccountApplicationAuthorized(serde_json::from_value(data)?)
            }
            "account.application.deauthorized" => {
                EventObject::AccountApplicationDeauthorized(serde_json::from_value(data)?)
            }
            "account.external_account.created" => {
                EventObject::AccountExternalAccountCreated(serde_json::from_value(data)?)
            }
            "account.external_account.deleted" => {
                EventObject::AccountExternalAccountDeleted(serde_json::from_value(data)?)
            }
            "account.external_account.updated" => {
                EventObject::AccountExternalAccountUpdated(serde_json::from_value(data)?)
            }
            "account.updated" => EventObject::AccountUpdated(serde_json::from_value(data)?),
            "application_fee.created" => {
                EventObject::ApplicationFeeCreated(serde_json::from_value(data)?)
            }
            "application_fee.refund.updated" => {
                EventObject::ApplicationFeeRefundUpdated(serde_json::from_value(data)?)
            }
            "application_fee.refunded" => {
                EventObject::ApplicationFeeRefunded(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_core")]
            "balance.available" => EventObject::BalanceAvailable(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_billing")]
            "billing_portal.configuration.created" => {
                EventObject::BillingPortalConfigurationCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_billing")]
            "billing_portal.configuration.updated" => {
                EventObject::BillingPortalConfigurationUpdated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_billing")]
            "billing_portal.session.created" => {
                EventObject::BillingPortalSessionCreated(serde_json::from_value(data)?)
            }
            "capability.updated" => EventObject::CapabilityUpdated(serde_json::from_value(data)?),
            "cash_balance.funds_available" => {
                EventObject::CashBalanceFundsAvailable(serde_json::from_value(data)?)
            }
            "charge.captured" => EventObject::ChargeCaptured(serde_json::from_value(data)?),
            "charge.dispute.closed" => {
                EventObject::ChargeDisputeClosed(serde_json::from_value(data)?)
            }
            "charge.dispute.created" => {
                EventObject::ChargeDisputeCreated(serde_json::from_value(data)?)
            }
            "charge.dispute.funds_reinstated" => {
                EventObject::ChargeDisputeFundsReinstated(serde_json::from_value(data)?)
            }
            "charge.dispute.funds_withdrawn" => {
                EventObject::ChargeDisputeFundsWithdrawn(serde_json::from_value(data)?)
            }
            "charge.dispute.updated" => {
                EventObject::ChargeDisputeUpdated(serde_json::from_value(data)?)
            }
            "charge.expired" => EventObject::ChargeExpired(serde_json::from_value(data)?),
            "charge.failed" => EventObject::ChargeFailed(serde_json::from_value(data)?),
            "charge.pending" => EventObject::ChargePending(serde_json::from_value(data)?),
            "charge.refund.updated" => {
                EventObject::ChargeRefundUpdated(serde_json::from_value(data)?)
            }
            "charge.refunded" => EventObject::ChargeRefunded(serde_json::from_value(data)?),
            "charge.succeeded" => EventObject::ChargeSucceeded(serde_json::from_value(data)?),
            "charge.updated" => EventObject::ChargeUpdated(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.async_payment_failed" => {
                EventObject::CheckoutSessionAsyncPaymentFailed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.async_payment_succeeded" => {
                EventObject::CheckoutSessionAsyncPaymentSucceeded(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.completed" => {
                EventObject::CheckoutSessionCompleted(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.expired" => {
                EventObject::CheckoutSessionExpired(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.order.canceled" => {
                EventObject::ClimateOrderCanceled(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.order.created" => {
                EventObject::ClimateOrderCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.order.delayed" => {
                EventObject::ClimateOrderDelayed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.order.delivered" => {
                EventObject::ClimateOrderDelivered(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.order.product_substituted" => {
                EventObject::ClimateOrderProductSubstituted(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.product.created" => {
                EventObject::ClimateProductCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.product.pricing_updated" => {
                EventObject::ClimateProductPricingUpdated(serde_json::from_value(data)?)
            }
            "coupon.created" => EventObject::CouponCreated(serde_json::from_value(data)?),
            "coupon.deleted" => EventObject::CouponDeleted(serde_json::from_value(data)?),
            "coupon.updated" => EventObject::CouponUpdated(serde_json::from_value(data)?),
            "credit_note.created" => EventObject::CreditNoteCreated(serde_json::from_value(data)?),
            "credit_note.updated" => EventObject::CreditNoteUpdated(serde_json::from_value(data)?),
            "credit_note.voided" => EventObject::CreditNoteVoided(serde_json::from_value(data)?),
            "customer.created" => EventObject::CustomerCreated(serde_json::from_value(data)?),
            "customer.deleted" => EventObject::CustomerDeleted(serde_json::from_value(data)?),
            "customer.discount.created" => {
                EventObject::CustomerDiscountCreated(serde_json::from_value(data)?)
            }
            "customer.discount.deleted" => {
                EventObject::CustomerDiscountDeleted(serde_json::from_value(data)?)
            }
            "customer.discount.updated" => {
                EventObject::CustomerDiscountUpdated(serde_json::from_value(data)?)
            }
            "customer.source.created" => {
                EventObject::CustomerSourceCreated(serde_json::from_value(data)?)
            }
            "customer.source.deleted" => {
                EventObject::CustomerSourceDeleted(serde_json::from_value(data)?)
            }
            "customer.source.expiring" => {
                EventObject::CustomerSourceExpiring(serde_json::from_value(data)?)
            }
            "customer.source.updated" => {
                EventObject::CustomerSourceUpdated(serde_json::from_value(data)?)
            }
            "customer.subscription.created" => {
                EventObject::CustomerSubscriptionCreated(serde_json::from_value(data)?)
            }
            "customer.subscription.deleted" => {
                EventObject::CustomerSubscriptionDeleted(serde_json::from_value(data)?)
            }
            "customer.subscription.paused" => {
                EventObject::CustomerSubscriptionPaused(serde_json::from_value(data)?)
            }
            "customer.subscription.pending_update_applied" => {
                EventObject::CustomerSubscriptionPendingUpdateApplied(serde_json::from_value(data)?)
            }
            "customer.subscription.pending_update_expired" => {
                EventObject::CustomerSubscriptionPendingUpdateExpired(serde_json::from_value(data)?)
            }
            "customer.subscription.resumed" => {
                EventObject::CustomerSubscriptionResumed(serde_json::from_value(data)?)
            }
            "customer.subscription.trial_will_end" => {
                EventObject::CustomerSubscriptionTrialWillEnd(serde_json::from_value(data)?)
            }
            "customer.subscription.updated" => {
                EventObject::CustomerSubscriptionUpdated(serde_json::from_value(data)?)
            }
            "customer.tax_id.created" => {
                EventObject::CustomerTaxIdCreated(serde_json::from_value(data)?)
            }
            "customer.tax_id.deleted" => {
                EventObject::CustomerTaxIdDeleted(serde_json::from_value(data)?)
            }
            "customer.tax_id.updated" => {
                EventObject::CustomerTaxIdUpdated(serde_json::from_value(data)?)
            }
            "customer.updated" => EventObject::CustomerUpdated(serde_json::from_value(data)?),
            "customer_cash_balance_transaction.created" => {
                EventObject::CustomerCashBalanceTransactionCreated(serde_json::from_value(data)?)
            }
            "file.created" => EventObject::FileCreated(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.created" => {
                EventObject::FinancialConnectionsAccountCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.deactivated" => {
                EventObject::FinancialConnectionsAccountDeactivated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.disconnected" => {
                EventObject::FinancialConnectionsAccountDisconnected(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.reactivated" => {
                EventObject::FinancialConnectionsAccountReactivated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.refreshed_balance" => {
                EventObject::FinancialConnectionsAccountRefreshedBalance(serde_json::from_value(
                    data,
                )?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.refreshed_transactions" => {
                EventObject::FinancialConnectionsAccountRefreshedTransactions(
                    serde_json::from_value(data)?,
                )
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.canceled" => {
                EventObject::IdentityVerificationSessionCanceled(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.created" => {
                EventObject::IdentityVerificationSessionCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.processing" => {
                EventObject::IdentityVerificationSessionProcessing(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.redacted" => {
                EventObject::IdentityVerificationSessionRedacted(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.requires_input" => {
                EventObject::IdentityVerificationSessionRequiresInput(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.verified" => {
                EventObject::IdentityVerificationSessionVerified(serde_json::from_value(data)?)
            }
            "invoice.created" => EventObject::InvoiceCreated(serde_json::from_value(data)?),
            "invoice.deleted" => EventObject::InvoiceDeleted(serde_json::from_value(data)?),
            "invoice.finalization_failed" => {
                EventObject::InvoiceFinalizationFailed(serde_json::from_value(data)?)
            }
            "invoice.finalized" => EventObject::InvoiceFinalized(serde_json::from_value(data)?),
            "invoice.marked_uncollectible" => {
                EventObject::InvoiceMarkedUncollectible(serde_json::from_value(data)?)
            }
            "invoice.paid" => EventObject::InvoicePaid(serde_json::from_value(data)?),
            "invoice.payment_action_required" => {
                EventObject::InvoicePaymentActionRequired(serde_json::from_value(data)?)
            }
            "invoice.payment_failed" => {
                EventObject::InvoicePaymentFailed(serde_json::from_value(data)?)
            }
            "invoice.payment_succeeded" => {
                EventObject::InvoicePaymentSucceeded(serde_json::from_value(data)?)
            }
            "invoice.sent" => EventObject::InvoiceSent(serde_json::from_value(data)?),
            "invoice.upcoming" => EventObject::InvoiceUpcoming(serde_json::from_value(data)?),
            "invoice.updated" => EventObject::InvoiceUpdated(serde_json::from_value(data)?),
            "invoice.voided" => EventObject::InvoiceVoided(serde_json::from_value(data)?),
            "invoiceitem.created" => EventObject::InvoiceitemCreated(serde_json::from_value(data)?),
            "invoiceitem.deleted" => EventObject::InvoiceitemDeleted(serde_json::from_value(data)?),
            "issuing_authorization.created" => {
                EventObject::IssuingAuthorizationCreated(serde_json::from_value(data)?)
            }
            "issuing_authorization.request" => {
                EventObject::IssuingAuthorizationRequest(serde_json::from_value(data)?)
            }
            "issuing_authorization.updated" => {
                EventObject::IssuingAuthorizationUpdated(serde_json::from_value(data)?)
            }
            "issuing_card.created" => {
                EventObject::IssuingCardCreated(serde_json::from_value(data)?)
            }
            "issuing_card.updated" => {
                EventObject::IssuingCardUpdated(serde_json::from_value(data)?)
            }
            "issuing_cardholder.created" => {
                EventObject::IssuingCardholderCreated(serde_json::from_value(data)?)
            }
            "issuing_cardholder.updated" => {
                EventObject::IssuingCardholderUpdated(serde_json::from_value(data)?)
            }
            "issuing_dispute.closed" => {
                EventObject::IssuingDisputeClosed(serde_json::from_value(data)?)
            }
            "issuing_dispute.created" => {
                EventObject::IssuingDisputeCreated(serde_json::from_value(data)?)
            }
            "issuing_dispute.funds_reinstated" => {
                EventObject::IssuingDisputeFundsReinstated(serde_json::from_value(data)?)
            }
            "issuing_dispute.submitted" => {
                EventObject::IssuingDisputeSubmitted(serde_json::from_value(data)?)
            }
            "issuing_dispute.updated" => {
                EventObject::IssuingDisputeUpdated(serde_json::from_value(data)?)
            }
            "issuing_token.created" => {
                EventObject::IssuingTokenCreated(serde_json::from_value(data)?)
            }
            "issuing_token.updated" => {
                EventObject::IssuingTokenUpdated(serde_json::from_value(data)?)
            }
            "issuing_transaction.created" => {
                EventObject::IssuingTransactionCreated(serde_json::from_value(data)?)
            }
            "issuing_transaction.updated" => {
                EventObject::IssuingTransactionUpdated(serde_json::from_value(data)?)
            }
            "mandate.updated" => EventObject::MandateUpdated(serde_json::from_value(data)?),
            "payment_intent.amount_capturable_updated" => {
                EventObject::PaymentIntentAmountCapturableUpdated(serde_json::from_value(data)?)
            }
            "payment_intent.canceled" => {
                EventObject::PaymentIntentCanceled(serde_json::from_value(data)?)
            }
            "payment_intent.created" => {
                EventObject::PaymentIntentCreated(serde_json::from_value(data)?)
            }
            "payment_intent.partially_funded" => {
                EventObject::PaymentIntentPartiallyFunded(serde_json::from_value(data)?)
            }
            "payment_intent.payment_failed" => {
                EventObject::PaymentIntentPaymentFailed(serde_json::from_value(data)?)
            }
            "payment_intent.processing" => {
                EventObject::PaymentIntentProcessing(serde_json::from_value(data)?)
            }
            "payment_intent.requires_action" => {
                EventObject::PaymentIntentRequiresAction(serde_json::from_value(data)?)
            }
            "payment_intent.succeeded" => {
                EventObject::PaymentIntentSucceeded(serde_json::from_value(data)?)
            }
            "payment_link.created" => {
                EventObject::PaymentLinkCreated(serde_json::from_value(data)?)
            }
            "payment_link.updated" => {
                EventObject::PaymentLinkUpdated(serde_json::from_value(data)?)
            }
            "payment_method.attached" => {
                EventObject::PaymentMethodAttached(serde_json::from_value(data)?)
            }
            "payment_method.automatically_updated" => {
                EventObject::PaymentMethodAutomaticallyUpdated(serde_json::from_value(data)?)
            }
            "payment_method.detached" => {
                EventObject::PaymentMethodDetached(serde_json::from_value(data)?)
            }
            "payment_method.updated" => {
                EventObject::PaymentMethodUpdated(serde_json::from_value(data)?)
            }
            "payout.canceled" => EventObject::PayoutCanceled(serde_json::from_value(data)?),
            "payout.created" => EventObject::PayoutCreated(serde_json::from_value(data)?),
            "payout.failed" => EventObject::PayoutFailed(serde_json::from_value(data)?),
            "payout.paid" => EventObject::PayoutPaid(serde_json::from_value(data)?),
            "payout.reconciliation_completed" => {
                EventObject::PayoutReconciliationCompleted(serde_json::from_value(data)?)
            }
            "payout.updated" => EventObject::PayoutUpdated(serde_json::from_value(data)?),
            "person.created" => EventObject::PersonCreated(serde_json::from_value(data)?),
            "person.deleted" => EventObject::PersonDeleted(serde_json::from_value(data)?),
            "person.updated" => EventObject::PersonUpdated(serde_json::from_value(data)?),
            "plan.created" => EventObject::PlanCreated(serde_json::from_value(data)?),
            "plan.deleted" => EventObject::PlanDeleted(serde_json::from_value(data)?),
            "plan.updated" => EventObject::PlanUpdated(serde_json::from_value(data)?),
            "price.created" => EventObject::PriceCreated(serde_json::from_value(data)?),
            "price.deleted" => EventObject::PriceDeleted(serde_json::from_value(data)?),
            "price.updated" => EventObject::PriceUpdated(serde_json::from_value(data)?),
            "product.created" => EventObject::ProductCreated(serde_json::from_value(data)?),
            "product.deleted" => EventObject::ProductDeleted(serde_json::from_value(data)?),
            "product.updated" => EventObject::ProductUpdated(serde_json::from_value(data)?),
            "promotion_code.created" => {
                EventObject::PromotionCodeCreated(serde_json::from_value(data)?)
            }
            "promotion_code.updated" => {
                EventObject::PromotionCodeUpdated(serde_json::from_value(data)?)
            }
            "quote.accepted" => EventObject::QuoteAccepted(serde_json::from_value(data)?),
            "quote.canceled" => EventObject::QuoteCanceled(serde_json::from_value(data)?),
            "quote.created" => EventObject::QuoteCreated(serde_json::from_value(data)?),
            "quote.finalized" => EventObject::QuoteFinalized(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_fraud")]
            "radar.early_fraud_warning.created" => {
                EventObject::RadarEarlyFraudWarningCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_fraud")]
            "radar.early_fraud_warning.updated" => {
                EventObject::RadarEarlyFraudWarningUpdated(serde_json::from_value(data)?)
            }
            "refund.created" => EventObject::RefundCreated(serde_json::from_value(data)?),
            "refund.updated" => EventObject::RefundUpdated(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "reporting.report_run.failed" => {
                EventObject::ReportingReportRunFailed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "reporting.report_run.succeeded" => {
                EventObject::ReportingReportRunSucceeded(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "reporting.report_type.updated" => {
                EventObject::ReportingReportTypeUpdated(serde_json::from_value(data)?)
            }
            "review.closed" => EventObject::ReviewClosed(serde_json::from_value(data)?),
            "review.opened" => EventObject::ReviewOpened(serde_json::from_value(data)?),
            "setup_intent.canceled" => {
                EventObject::SetupIntentCanceled(serde_json::from_value(data)?)
            }
            "setup_intent.created" => {
                EventObject::SetupIntentCreated(serde_json::from_value(data)?)
            }
            "setup_intent.requires_action" => {
                EventObject::SetupIntentRequiresAction(serde_json::from_value(data)?)
            }
            "setup_intent.setup_failed" => {
                EventObject::SetupIntentSetupFailed(serde_json::from_value(data)?)
            }
            "setup_intent.succeeded" => {
                EventObject::SetupIntentSucceeded(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "sigma.scheduled_query_run.created" => {
                EventObject::SigmaScheduledQueryRunCreated(serde_json::from_value(data)?)
            }
            "source.canceled" => EventObject::SourceCanceled(serde_json::from_value(data)?),
            "source.chargeable" => EventObject::SourceChargeable(serde_json::from_value(data)?),
            "source.failed" => EventObject::SourceFailed(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_payment")]
            "source.mandate_notification" => {
                EventObject::SourceMandateNotification(serde_json::from_value(data)?)
            }
            "source.refund_attributes_required" => {
                EventObject::SourceRefundAttributesRequired(serde_json::from_value(data)?)
            }
            "source.transaction.created" => {
                EventObject::SourceTransactionCreated(serde_json::from_value(data)?)
            }
            "source.transaction.updated" => {
                EventObject::SourceTransactionUpdated(serde_json::from_value(data)?)
            }
            "subscription_schedule.aborted" => {
                EventObject::SubscriptionScheduleAborted(serde_json::from_value(data)?)
            }
            "subscription_schedule.canceled" => {
                EventObject::SubscriptionScheduleCanceled(serde_json::from_value(data)?)
            }
            "subscription_schedule.completed" => {
                EventObject::SubscriptionScheduleCompleted(serde_json::from_value(data)?)
            }
            "subscription_schedule.created" => {
                EventObject::SubscriptionScheduleCreated(serde_json::from_value(data)?)
            }
            "subscription_schedule.expiring" => {
                EventObject::SubscriptionScheduleExpiring(serde_json::from_value(data)?)
            }
            "subscription_schedule.released" => {
                EventObject::SubscriptionScheduleReleased(serde_json::from_value(data)?)
            }
            "subscription_schedule.updated" => {
                EventObject::SubscriptionScheduleUpdated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "tax.settings.updated" => {
                EventObject::TaxSettingsUpdated(serde_json::from_value(data)?)
            }
            "tax_rate.created" => EventObject::TaxRateCreated(serde_json::from_value(data)?),
            "tax_rate.updated" => EventObject::TaxRateUpdated(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_terminal")]
            "terminal.reader.action_failed" => {
                EventObject::TerminalReaderActionFailed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_terminal")]
            "terminal.reader.action_succeeded" => {
                EventObject::TerminalReaderActionSucceeded(serde_json::from_value(data)?)
            }
            "test_helpers.test_clock.advancing" => {
                EventObject::TestHelpersTestClockAdvancing(serde_json::from_value(data)?)
            }
            "test_helpers.test_clock.created" => {
                EventObject::TestHelpersTestClockCreated(serde_json::from_value(data)?)
            }
            "test_helpers.test_clock.deleted" => {
                EventObject::TestHelpersTestClockDeleted(serde_json::from_value(data)?)
            }
            "test_helpers.test_clock.internal_failure" => {
                EventObject::TestHelpersTestClockInternalFailure(serde_json::from_value(data)?)
            }
            "test_helpers.test_clock.ready" => {
                EventObject::TestHelpersTestClockReady(serde_json::from_value(data)?)
            }
            "topup.canceled" => EventObject::TopupCanceled(serde_json::from_value(data)?),
            "topup.created" => EventObject::TopupCreated(serde_json::from_value(data)?),
            "topup.failed" => EventObject::TopupFailed(serde_json::from_value(data)?),
            "topup.reversed" => EventObject::TopupReversed(serde_json::from_value(data)?),
            "topup.succeeded" => EventObject::TopupSucceeded(serde_json::from_value(data)?),
            "transfer.created" => EventObject::TransferCreated(serde_json::from_value(data)?),
            "transfer.reversed" => EventObject::TransferReversed(serde_json::from_value(data)?),
            "transfer.updated" => EventObject::TransferUpdated(serde_json::from_value(data)?),
            #[cfg(feature = "stripe_treasury")]
            "treasury.credit_reversal.created" => {
                EventObject::TreasuryCreditReversalCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.credit_reversal.posted" => {
                EventObject::TreasuryCreditReversalPosted(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.debit_reversal.completed" => {
                EventObject::TreasuryDebitReversalCompleted(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.debit_reversal.created" => {
                EventObject::TreasuryDebitReversalCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.debit_reversal.initial_credit_granted" => {
                EventObject::TreasuryDebitReversalInitialCreditGranted(serde_json::from_value(
                    data,
                )?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.financial_account.closed" => {
                EventObject::TreasuryFinancialAccountClosed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.financial_account.created" => {
                EventObject::TreasuryFinancialAccountCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.financial_account.features_status_updated" => {
                EventObject::TreasuryFinancialAccountFeaturesStatusUpdated(serde_json::from_value(
                    data,
                )?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.canceled" => {
                EventObject::TreasuryInboundTransferCanceled(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.created" => {
                EventObject::TreasuryInboundTransferCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.failed" => {
                EventObject::TreasuryInboundTransferFailed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.succeeded" => {
                EventObject::TreasuryInboundTransferSucceeded(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.canceled" => {
                EventObject::TreasuryOutboundPaymentCanceled(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.created" => {
                EventObject::TreasuryOutboundPaymentCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.expected_arrival_date_updated" => {
                EventObject::TreasuryOutboundPaymentExpectedArrivalDateUpdated(
                    serde_json::from_value(data)?,
                )
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.failed" => {
                EventObject::TreasuryOutboundPaymentFailed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.posted" => {
                EventObject::TreasuryOutboundPaymentPosted(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.returned" => {
                EventObject::TreasuryOutboundPaymentReturned(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.canceled" => {
                EventObject::TreasuryOutboundTransferCanceled(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.created" => {
                EventObject::TreasuryOutboundTransferCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.expected_arrival_date_updated" => {
                EventObject::TreasuryOutboundTransferExpectedArrivalDateUpdated(
                    serde_json::from_value(data)?,
                )
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.failed" => {
                EventObject::TreasuryOutboundTransferFailed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.posted" => {
                EventObject::TreasuryOutboundTransferPosted(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.returned" => {
                EventObject::TreasuryOutboundTransferReturned(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_credit.created" => {
                EventObject::TreasuryReceivedCreditCreated(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_credit.failed" => {
                EventObject::TreasuryReceivedCreditFailed(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_credit.succeeded" => {
                EventObject::TreasuryReceivedCreditSucceeded(serde_json::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_debit.created" => {
                EventObject::TreasuryReceivedDebitCreated(serde_json::from_value(data)?)
            }

            _ => EventObject::Unknown(data),
        })
    }
}
