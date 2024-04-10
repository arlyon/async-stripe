use crate::deserialization_fixture::check_object;

pub fn check_fixtures(resources: &serde_json::Value) {
    check_object::<stripe_connect::Account>(resources, "account");
    check_object::<stripe_connect::AccountLink>(resources, "account_link");
    check_object::<stripe_connect::AccountSession>(resources, "account_session");
    check_object::<stripe_misc::ApplePayDomain>(resources, "apple_pay_domain");
    check_object::<stripe_connect::Application>(resources, "application");
    check_object::<stripe_connect::ApplicationFee>(resources, "application_fee");
    check_object::<stripe_connect::AppsSecret>(resources, "apps.secret");
    check_object::<stripe_core::Balance>(resources, "balance");
    check_object::<stripe_core::BalanceTransaction>(resources, "balance_transaction");
    check_object::<stripe_payment::BankAccount>(resources, "bank_account");
    check_object::<stripe_billing::BillingPortalConfiguration>(
        resources,
        "billing_portal.configuration",
    );
    check_object::<stripe_billing::BillingPortalSession>(resources, "billing_portal.session");
    check_object::<stripe_connect::Capability>(resources, "capability");
    check_object::<stripe_payment::Card>(resources, "card");
    check_object::<stripe_core::CashBalance>(resources, "cash_balance");
    check_object::<stripe_core::Charge>(resources, "charge");
    check_object::<stripe_checkout::CheckoutSession>(resources, "checkout.session");
    check_object::<stripe_misc::ClimateOrder>(resources, "climate.order");
    check_object::<stripe_misc::ClimateProduct>(resources, "climate.product");
    check_object::<stripe_misc::ClimateSupplier>(resources, "climate.supplier");
    check_object::<stripe_core::ConnectCollectionTransfer>(
        resources,
        "connect_collection_transfer",
    );
    check_object::<stripe_connect::CountrySpec>(resources, "country_spec");
    check_object::<stripe_product::Coupon>(resources, "coupon");
    check_object::<stripe_billing::CreditNote>(resources, "credit_note");
    check_object::<stripe_billing::CreditNoteLineItem>(resources, "credit_note_line_item");
    check_object::<stripe_core::Customer>(resources, "customer");
    check_object::<stripe_core::CustomerBalanceTransaction>(
        resources,
        "customer_balance_transaction",
    );
    check_object::<stripe_core::CustomerCashBalanceTransaction>(
        resources,
        "customer_cash_balance_transaction",
    );
    check_object::<stripe_core::CustomerSession>(resources, "customer_session");
    check_object::<stripe_connect::DeletedAccount>(resources, "deleted_account");
    check_object::<stripe_misc::DeletedApplePayDomain>(resources, "deleted_apple_pay_domain");
    check_object::<stripe_payment::DeletedBankAccount>(resources, "deleted_bank_account");
    check_object::<stripe_payment::DeletedCard>(resources, "deleted_card");
    check_object::<stripe_product::DeletedCoupon>(resources, "deleted_coupon");
    check_object::<stripe_core::DeletedCustomer>(resources, "deleted_customer");
    check_object::<stripe_product::DeletedDiscount>(resources, "deleted_discount");
    check_object::<stripe_billing::DeletedInvoice>(resources, "deleted_invoice");
    check_object::<stripe_billing::DeletedInvoiceitem>(resources, "deleted_invoiceitem");
    check_object::<stripe_connect::DeletedPerson>(resources, "deleted_person");
    check_object::<stripe_billing::DeletedPlan>(resources, "deleted_plan");
    check_object::<stripe_product::DeletedProduct>(resources, "deleted_product");
    check_object::<stripe_fraud::DeletedRadarValueList>(resources, "deleted_radar.value_list");
    check_object::<stripe_fraud::DeletedRadarValueListItem>(
        resources,
        "deleted_radar.value_list_item",
    );
    check_object::<stripe_billing::DeletedSubscriptionItem>(resources, "deleted_subscription_item");
    check_object::<stripe_billing::DeletedTaxId>(resources, "deleted_tax_id");
    check_object::<stripe_terminal::DeletedTerminalConfiguration>(
        resources,
        "deleted_terminal.configuration",
    );
    check_object::<stripe_terminal::DeletedTerminalLocation>(
        resources,
        "deleted_terminal.location",
    );
    check_object::<stripe_terminal::DeletedTerminalReader>(resources, "deleted_terminal.reader");
    check_object::<stripe_billing::DeletedTestHelpersTestClock>(
        resources,
        "deleted_test_helpers.test_clock",
    );
    check_object::<stripe_misc::DeletedWebhookEndpoint>(resources, "deleted_webhook_endpoint");
    check_object::<stripe_product::Discount>(resources, "discount");
    check_object::<stripe_core::Dispute>(resources, "dispute");
    check_object::<stripe_misc::EphemeralKey>(resources, "ephemeral_key");
    check_object::<stripe_core::Event>(resources, "event");
    check_object::<stripe_misc::ExchangeRate>(resources, "exchange_rate");
    check_object::<stripe_connect::ApplicationFeeRefund>(resources, "fee_refund");
    check_object::<stripe_core::File>(resources, "file");
    check_object::<stripe_core::FileLink>(resources, "file_link");
    check_object::<stripe_misc::FinancialConnectionsAccount>(
        resources,
        "financial_connections.account",
    );
    check_object::<stripe_misc::FinancialConnectionsAccountOwner>(
        resources,
        "financial_connections.account_owner",
    );
    check_object::<stripe_misc::FinancialConnectionsAccountOwnership>(
        resources,
        "financial_connections.account_ownership",
    );
    check_object::<stripe_misc::FinancialConnectionsSession>(
        resources,
        "financial_connections.session",
    );
    check_object::<stripe_misc::FinancialConnectionsTransaction>(
        resources,
        "financial_connections.transaction",
    );
    check_object::<stripe_issuing::FundingInstructions>(resources, "funding_instructions");
    check_object::<stripe_misc::IdentityVerificationReport>(
        resources,
        "identity.verification_report",
    );
    check_object::<stripe_misc::IdentityVerificationSession>(
        resources,
        "identity.verification_session",
    );
    check_object::<stripe_billing::Invoice>(resources, "invoice");
    check_object::<stripe_billing::InvoiceItem>(resources, "invoiceitem");
    check_object::<stripe_issuing::IssuingAuthorization>(resources, "issuing.authorization");
    check_object::<stripe_issuing::IssuingCard>(resources, "issuing.card");
    check_object::<stripe_issuing::IssuingCardholder>(resources, "issuing.cardholder");
    check_object::<stripe_issuing::IssuingDispute>(resources, "issuing.dispute");
    check_object::<stripe_issuing::IssuingToken>(resources, "issuing.token");
    check_object::<stripe_issuing::IssuingTransaction>(resources, "issuing.transaction");
    check_object::<stripe_billing::CheckoutSessionItem>(resources, "item");
    check_object::<stripe_billing::InvoiceLineItem>(resources, "line_item");
    check_object::<stripe_connect::LoginLink>(resources, "login_link");
    check_object::<stripe_core::Mandate>(resources, "mandate");
    check_object::<stripe_core::PaymentIntent>(resources, "payment_intent");
    check_object::<stripe_payment::PaymentLink>(resources, "payment_link");
    check_object::<stripe_payment::PaymentMethod>(resources, "payment_method");
    check_object::<stripe_payment::PaymentMethodConfiguration>(
        resources,
        "payment_method_configuration",
    );
    check_object::<stripe_payment::PaymentMethodDomain>(resources, "payment_method_domain");
    check_object::<stripe_core::Payout>(resources, "payout");
    check_object::<stripe_connect::Person>(resources, "person");
    check_object::<stripe_billing::Plan>(resources, "plan");
    check_object::<stripe_core::PlatformTaxFee>(resources, "platform_tax_fee");
    check_object::<stripe_product::Price>(resources, "price");
    check_object::<stripe_product::Product>(resources, "product");
    check_object::<stripe_product::PromotionCode>(resources, "promotion_code");
    check_object::<stripe_billing::Quote>(resources, "quote");
    check_object::<stripe_fraud::RadarEarlyFraudWarning>(resources, "radar.early_fraud_warning");
    check_object::<stripe_fraud::RadarValueList>(resources, "radar.value_list");
    check_object::<stripe_fraud::RadarValueListItem>(resources, "radar.value_list_item");
    check_object::<stripe_core::Refund>(resources, "refund");
    check_object::<stripe_misc::ReportingReportRun>(resources, "reporting.report_run");
    check_object::<stripe_misc::ReportingReportType>(resources, "reporting.report_type");
    check_object::<stripe_core::ReserveTransaction>(resources, "reserve_transaction");
    check_object::<stripe_fraud::Review>(resources, "review");
    check_object::<stripe_misc::ScheduledQueryRun>(resources, "scheduled_query_run");
    check_object::<stripe_core::SetupAttempt>(resources, "setup_attempt");
    check_object::<stripe_core::SetupIntent>(resources, "setup_intent");
    check_object::<stripe_product::ShippingRate>(resources, "shipping_rate");
    check_object::<stripe_payment::Source>(resources, "source");
    check_object::<stripe_payment::SourceMandateNotification>(
        resources,
        "source_mandate_notification",
    );
    check_object::<stripe_payment::SourceTransaction>(resources, "source_transaction");
    check_object::<stripe_billing::Subscription>(resources, "subscription");
    check_object::<stripe_billing::SubscriptionItem>(resources, "subscription_item");
    check_object::<stripe_billing::SubscriptionSchedule>(resources, "subscription_schedule");
    check_object::<stripe_misc::TaxCalculation>(resources, "tax.calculation");
    check_object::<stripe_misc::TaxCalculationLineItem>(resources, "tax.calculation_line_item");
    check_object::<stripe_misc::TaxRegistration>(resources, "tax.registration");
    check_object::<stripe_misc::TaxSettings>(resources, "tax.settings");
    check_object::<stripe_misc::TaxTransaction>(resources, "tax.transaction");
    check_object::<stripe_misc::TaxTransactionLineItem>(resources, "tax.transaction_line_item");
    check_object::<stripe_product::TaxCode>(resources, "tax_code");
    check_object::<stripe_core::TaxDeductedAtSource>(resources, "tax_deducted_at_source");
    check_object::<stripe_billing::TaxId>(resources, "tax_id");
    check_object::<stripe_product::TaxRate>(resources, "tax_rate");
    check_object::<stripe_terminal::TerminalConfiguration>(resources, "terminal.configuration");
    check_object::<stripe_terminal::TerminalConnectionToken>(
        resources,
        "terminal.connection_token",
    );
    check_object::<stripe_terminal::TerminalLocation>(resources, "terminal.location");
    check_object::<stripe_terminal::TerminalReader>(resources, "terminal.reader");
    check_object::<stripe_billing::TestHelpersTestClock>(resources, "test_helpers.test_clock");
    check_object::<stripe_core::Token>(resources, "token");
    check_object::<stripe_connect::Topup>(resources, "topup");
    check_object::<stripe_connect::Transfer>(resources, "transfer");
    check_object::<stripe_connect::TransferReversal>(resources, "transfer_reversal");
    check_object::<stripe_treasury::TreasuryCreditReversal>(resources, "treasury.credit_reversal");
    check_object::<stripe_treasury::TreasuryDebitReversal>(resources, "treasury.debit_reversal");
    check_object::<stripe_treasury::TreasuryFinancialAccount>(
        resources,
        "treasury.financial_account",
    );
    check_object::<stripe_treasury::TreasuryFinancialAccountFeatures>(
        resources,
        "treasury.financial_account_features",
    );
    check_object::<stripe_treasury::TreasuryInboundTransfer>(
        resources,
        "treasury.inbound_transfer",
    );
    check_object::<stripe_treasury::TreasuryOutboundPayment>(
        resources,
        "treasury.outbound_payment",
    );
    check_object::<stripe_treasury::TreasuryOutboundTransfer>(
        resources,
        "treasury.outbound_transfer",
    );
    check_object::<stripe_treasury::TreasuryReceivedCredit>(resources, "treasury.received_credit");
    check_object::<stripe_treasury::TreasuryReceivedDebit>(resources, "treasury.received_debit");
    check_object::<stripe_treasury::TreasuryTransaction>(resources, "treasury.transaction");
    check_object::<stripe_treasury::TreasuryTransactionEntry>(
        resources,
        "treasury.transaction_entry",
    );
    check_object::<stripe_billing::UsageRecord>(resources, "usage_record");
    check_object::<stripe_billing::UsageRecordSummary>(resources, "usage_record_summary");
    check_object::<stripe_misc::WebhookEndpoint>(resources, "webhook_endpoint");
}
