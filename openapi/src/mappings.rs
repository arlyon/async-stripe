use std::collections::BTreeMap;

pub fn id_renames() -> BTreeMap<&'static str, &'static str> {
    [
        ("fee_refund", "application_fee_refund"),
        ("invoiceitem", "invoice_item"),
        ("line_item", "invoice_line_item"),
        ("source_transaction", "charge"),
        ("item", "checkout_session_item"),
    ]
    .iter()
    .copied()
    .collect()
}

pub type ObjectMap = BTreeMap<&'static str, &'static str>;

#[rustfmt::skip]
pub fn object_mappings() -> ObjectMap {
    [
        // Config for object types
        ("account_business_profile", "business_profile"),
        ("account_capabilities_card_issuing", "capability_status"),
        ("account_capabilities_card_payments", "capability_status"),
        ("account_capabilities_legacy_payments", "capability_status"),
        ("account_capabilities_platform_payments", "capability_status"),
        ("account_capabilities_jcb_payments", "capability_status"),
        ("account_capabilities_au_becs_debit_payments", "capability_status"),
        ("account_capabilities_tax_reporting_us_1099_k", "capability_status"),
        ("account_capabilities_tax_reporting_us_1099_misc", "capability_status"),
        ("account_capabilities_transfers", "capability_status"),
        ("account_branding_settings", "branding_settings"),
        ("account_card_payments_settings", "card_payments_settings"),
        ("account_dashboard_settings", "dashboard_settings"),
        ("account_decline_charge_on", "decline_charge_on"),
        ("account_external_account", "external_account"),
        ("account_payments_settings", "payments_settings"),
        ("account_payout_settings", "payout_settings"),
        ("account_tos_acceptance", "tos_acceptance"),
        ("charge_fraud_details", "fraud_details"),
        ("charge_transfer_data", "transfer_data"),
        ("fee_refund", "application_fee_refund"),
        ("issuing_authorization_merchant_data", "merchant_data"),
        ("issuing_authorization_wallet_provider", "wallet_provider"),
        ("item", "checkout_session_item"),
        ("invoice_collection_method", "collection_method"),
        ("invoices_resource_invoice_tax_id_type", "tax_id_type"),
        ("invoice_tax_amount", "tax_amount"),
        ("invoiceitem", "invoice_item"),
        ("legal_entity_company", "company"),
        ("legal_entity_japan_address", "address"),
        ("legal_entity_company_verification", "company_verification"),
        ("legal_entity_company_verification_document", "company_verification_document"),
        ("legal_entity_person_verification", "person_verification"),
        ("legal_entity_person_verification_document", "person_verification_document"),
        ("line_item", "invoice_line_item"),
        ("payment_method_card", "card_details"),
        ("payment_method_card_present", "card_present"),
        ("payment_method_card_wallet", "wallet_details"),
        ("payment_method_card_wallet_amex_express_checkout", "wallet_amex_express_checkout"),
        ("payment_method_card_wallet_apple_pay", "wallet_apple_pay"),
        ("payment_method_card_wallet_google_pay", "wallet_google_pay"),
        ("payment_method_card_wallet_masterpass", "wallet_masterpass"),
        ("payment_method_card_wallet_samsung_pay", "wallet_samsung_pay"),
        ("payment_method_card_wallet_visa_checkout", "wallet_visa_checkout"),
        ("payment_method_card_wallet_type", "wallet_type"),
        ("payment_pages_payment_page_resources_shipping_address_collection", "shipping_address_collection"),
        ("tax_id_data_type", "tax_id_type"),

        // Config for `account` params
        ("create_account_company", "company_params"),
        ("update_account_company", "company_params"),
        ("company_params_verification", "company_verification_params"),
        ("person_verification_params_document", "verification_document_params"),
        ("person_verification_params_additional_document", "verification_document_params"),
        ("create_account_individual", "person_params"),
        ("update_account_individual", "person_params"),
        ("create_account_requested_capabilities", "requested_capability"),
        ("update_account_requested_capabilities", "requested_capability"),
        ("create_account_settings", "account_settings_params"),
        ("update_account_settings", "account_settings_params"),
        ("account_settings_params_branding", "branding_settings_params"),
        ("account_settings_params_card_payments", "card_payments_settings_params"),
        ("account_settings_params_payments", "payments_settings_params"),
        ("account_settings_params_dashboard_settings", "dashboard_settings_params"),
        ("account_settings_params_payouts", "payout_settings_params"),
        ("create_account_tos_acceptance", "accept_tos"),
        ("update_account_tos_acceptance", "accept_tos"),
        ("card_payments_settings_params_decline_on", "decline_charge_on_params"),
        ("payout_settings_params_schedule", "transfer_schedule_params"),
        ("transfer_schedule_params_interval", "transfer_schedule_interval"),
        ("invoice_setting_subscription_schedule_setting", "subscription_schedule_invoice_settings"),
        ("create_token_person_verification", "person_verification_params"),

        // Config for `charge` params
        ("create_charge_transfer_data", "transfer_data_params"),
        ("update_charge_fraud_details", "fraud_details_params"),

        // Config for `customer` params
        ("create_customer_invoice_settings", "customer_invoice_settings"),
        ("update_customer_invoice_settings", "customer_invoice_settings"),
        ("create_customer_tax_id_data", "tax_id_data"),
        ("create_customer_tax_info", "tax_info_params"),
        ("update_customer_tax_info", "tax_info_params"),
        ("tax_info_params_type", "tax_info_type"),

        // Config for `invoiceitem` params
        ("create_invoiceitem", "create_invoice_item"),
        ("update_invoiceitem", "update_invoice_item"),
        ("create_invoice_item_price_data", "invoice_item_price_data"),
        ("update_invoice_item_price_data", "invoice_item_price_data"),
        ("add_invoice_items_price_data", "invoice_item_price_data"),

        // Config for `payment_intent` params
        ("create_order_items", "order_item_params"),
        ("order_items_params_type", "order_item_type"),

        // Config for `source` params
        ("create_source_mandate", "source_mandate_params"),
        ("update_source_mandate", "source_mandate_params"),
        // TODO: Maybe allow "union" if structs have optional params
        /*
        ("create_source_source_order", "source_order_params"),
        ("update_source_source_order", "source_order_params"),
        */
        ("source_mandate_params_acceptance", "source_acceptance_params"),
        ("source_mandate_params_interval", "source_mandate_interval"),
        ("source_mandate_params_notification_method", "source_mandate_notification_method"),
        ("source_acceptance_params_offline", "source_acceptance_offline_params"),
        ("source_acceptance_params_online", "source_acceptance_online_params"),
        ("create_source_receiver_refund_attributes_method", "source_refund_notification_method"),

        // Config for `subscription_schedule` params
        // TODO: Maybe allow "union" if structs have optional params
        /*
        ("create_subscription_schedule_phases", "subscription_schedule_phases_params"),
        ("update_subscription_schedule_phases", "subscription_schedule_phases_params"),
        */
        ("subscription_pending_invoice_item_interval_interval", "plan_interval"),
        ("create_subscription_pending_invoice_item_interval_interval", "plan_interval"),
        ("update_subscription_pending_invoice_item_interval_interval", "plan_interval"),
        ("subscription_item_price_data_recurring_interval", "plan_interval"),
        ("create_subscription_item_price_data", "subscription_item_price_data"),
        ("update_subscription_item_price_data", "subscription_item_price_data"),
        ("create_subscription_items_price_data", "subscription_item_price_data"),
        ("update_subscription_items_price_data", "subscription_item_price_data"),
        ("subscription_schedule_phases_plans_params_price_data", "subscription_item_price_data"),
        ("subscription_item_payment_behavior", "subscription_payment_behavior"),
        ("subscription_proration_behavior", "subscription_proration_behavior"),
        ("subscription_item_proration_behavior", "subscription_proration_behavior"),
        ("subscription_schedule_proration_behavior", "subscription_proration_behavior"),
        ("subscription_schedule_phase_configuration_proration_behavior", "subscription_proration_behavior"),
        ("create_subscription_schedule_phases_proration_behavior", "subscription_proration_behavior"),
        ("update_subscription_schedule_phases_proration_behavior", "subscription_proration_behavior"),
        ("create_subscription_schedule_phases_plans", "subscription_schedule_phases_plans_params"),
        ("update_subscription_schedule_phases_plans", "subscription_schedule_phases_plans_params"),
        ("create_subscription_schedule_phases_plans", "subscription_schedule_phases_plans_params"),
        ("update_subscription_schedule_phases_plans", "subscription_schedule_phases_plans_params"),
        ("create_subscription_schedule_invoice_settings", "subscription_schedule_invoice_settings"),
        ("update_subscription_schedule_invoice_settings", "subscription_schedule_invoice_settings"),
        ("create_subscription_schedule_phases_invoice_settings", "subscription_schedule_invoice_settings"),
        ("update_subscription_schedule_phases_invoice_settings", "subscription_schedule_invoice_settings"),
        ("create_subscription_schedule_renewal_interval", "subscription_schedule_renewal_interval_params"),
        ("update_subscription_schedule_renewal_interval", "subscription_schedule_renewal_interval_params"),
        ("create_subscription_schedule_renewal_interval", "subscription_schedule_renewal_interval_params"),
        ("update_subscription_schedule_renewal_interval", "subscription_schedule_renewal_interval_params"),
        ("create_subscription_schedule_default_settings", "subscription_schedule_default_settings_params"),
        ("update_subscription_schedule_default_settings", "subscription_schedule_default_settings_params"),
        ("create_subscription_add_invoice_items", "add_invoice_items"),
        ("update_subscription_add_invoice_items", "add_invoice_items"),
        ("create_subscription_schedule_phases_add_invoice_items", "add_invoice_items"),
        ("update_subscription_schedule_phases_add_invoice_items", "add_invoice_items"),
        ("subscription_schedule_end_behavior_filter", "subscription_schedule_renewal_behavior"),
        ("subscription_schedules_resource_default_settings", "subscription_schedule_default_settings"),
        ("subscription_schedule_default_settings_params_billing_thresholds", "subscription_schedule_billing_thresholds"),

        // Config for `webhook` params
        ("webhook_endpoint_enabled_events", "event_filter"),
        ("webhook_endpoint_api_version", "api_version"),
        ("create_webhook_endpoint_enabled_events", "event_filter"),
        ("update_webhook_endpoint_enabled_events", "event_filter"),
    ]
    .iter()
    .copied()
    .collect()
}

pub type FieldMap = BTreeMap<FieldSpec, ImportSpec>;
pub type FieldSpec = (
    &'static str, // schema name
    &'static str, // field name
);
pub type ImportSpec = (
    &'static str, // "use" name
    &'static str, // field type
);

#[rustfmt::skip]
pub fn field_mappings() -> FieldMap {
    [
        // Config for object types
        (("account", "type"), ("AccountType", "Option<AccountType>")),
        (("account", "business_type"), ("BusinessType", "Option<BusinessType>")),
        (("balance_transaction", "status"), ("BalanceTransactionStatus", "BalanceTransactionStatus")),
        (
            ("bank_account", "account_holder_type"),
            ("AccountHolderType", "Option<AccountHolderType>"),
        ),
        (("bank_account", "status"), ("BankAccountStatus", "Option<BankAccountStatus>")),
        (("fee", "type"), ("FeeType", "FeeType")),
        (("charge", "source"), ("PaymentSource", "Option<PaymentSource>")),
        (
            ("charge_fraud_details", "stripe_report"),
            ("FraudDetailsReport", "Option<FraudDetailsReport>"),
        ),
        (("customer", "default_source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("customer", "sources"), ("PaymentSource", "List<PaymentSource>")),
        (("invoice", "billing"), ("", "Option<CollectionMethod>")),
        (("invoice", "default_source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("invoiceitem", "period"), ("Period", "Option<Period>")),
        (("line_item", "period"), ("Period", "Option<Period>")),
        (
            ("issuing_authorization", "authorization_method"),
            ("IssuingAuthorizationMethod", "IssuingAuthorizationMethod"),
        ),
        (
            ("issuing_authorization", "wallet_provider"),
            ("IssuingAuthorizationWalletProvider", "Option<IssuingAuthorizationWalletProvider>"),
        ),
        (
            ("issuing_authorization_request", "reason"),
            ("IssuingAuthorizationReason", "IssuingAuthorizationReason"),
        ),
        (
            ("issuing_authorization_verification_data", "address_line1_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (
            ("issuing_authorization_verification_data", "address_zip_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (
            ("issuing_authorization_verification_data", "address_postal_code_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (
            ("issuing_authorization_verification_data", "cvc_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (
            ("issuing_authorization_verification_data", "expiry_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (("issuing_card", "brand"), ("CardBrand", "CardBrand")),
        (("issuing_card", "type"), ("IssuingCardType", "IssuingCardType")),
        (
            ("issuing_card_authorization_controls", "allowed_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_card_authorization_controls", "blocked_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_card_authorization_controls", "spending_limits"),
            ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
        ),
        (
            ("issuing_cardholder_authorization_controls", "allowed_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_cardholder_authorization_controls", "blocked_categories"),
            ("MerchantCategory", "Option<Vec<MerchantCategory>>"),
        ),
        (
            ("issuing_cardholder_authorization_controls", "spending_limits"),
            ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
        ),
        (
            ("issuing_card_pin", "status"),
            ("IssuingCardPinStatus", "IssuingCardPinStatus"),
        ),
        (
            ("issuing_card_shipping", "type"),
            ("IssuingCardShippingType", "IssuingCardShippingType"),
        ),
        (
            ("issuing_card_shipping", "status"),
            ("IssuingCardShippingStatus", "Option<IssuingCardShippingStatus>"),
        ),
        (
            ("issuing_dispute", "reason"),
            ("IssuingDisputeReason", "IssuingDisputeReason"),
        ),
        (
            ("issuing_dispute", "status"),
            ("IssuingDisputeStatus", "IssuingDisputeStatus"),
        ),
        (
            ("issuing_transaction", "type"),
            ("IssuingTransactionType", "IssuingTransactionType"),
        ),
        (("file", "purpose"), ("", "FilePurpose")),
        (("order", "status"), ("", "OrderStatus")),
        (("person", "dob"), ("Dob", "Option<Dob>")),
        (("recipient", "type"), ("", "Option<RecipientType>")),
        (("review", "reason"), ("ReviewReason", "ReviewReason")),
        (("sku", "attributes"), ("Metadata", "Option<Metadata>")),
        (
            ("subscription", "default_source"),
            ("PaymentSource", "Option<Expandable<PaymentSource>>"),
        ),
        (("source", "flow"), ("", "SourceFlow")),
        (("source", "status"), ("SourceStatus", "SourceStatus")),
        (("source", "usage"), ("SourceUsage", "Option<SourceUsage>")),
        (
            ("source_redirect_flow", "failure_reason"),
            ("SourceRedirectFlowFailureReason", "Option<SourceRedirectFlowFailureReason>"),
        ),
        (
            ("source_redirect_flow", "status"),
            ("SourceRedirectFlowStatus", "SourceRedirectFlowStatus"),
        ),
        (
            ("subscription", "billing"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("subscription", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("subscription_schedule", "billing"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("subscription_schedule", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("subscription_schedule_phase_configuration", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("subscription_schedule_renewal_interval", "interval"),
            ("PlanInterval", "PlanInterval"),
        ),
        (
            ("subscription_schedule_default_settings", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("subscription_schedule_default_settings_params", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (("token", "type"), ("TokenType", "TokenType")),
        (("transfer", "source_type"), ("", "Option<TransferSourceType>")),
        (("transfer_schedule", "weekly_anchor"), ("Weekday", "Option<Weekday>")),
        (("webhook_endpoint", "api_version"), ("ApiVersion", "Option<ApiVersion>")),
        (("webhook_endpoint", "enabled_events"), ("", "Option<Vec<EventFilter>>")),
        (("webhook_endpoint", "status"), ("WebhookEndpointStatus", "Option<WebhookEndpointStatus>")),

        // Config for `account` params
        (("create_account", "business_profile"), ("BusinessProfile", "Option<BusinessProfile>")),
        (("update_account", "business_profile"), ("BusinessProfile", "Option<BusinessProfile>")),
        (("create_account", "business_type"), ("BusinessType", "Option<BusinessType>")),
        (("update_account", "business_type"), ("BusinessType", "Option<BusinessType>")),
        (("company_params", "address"), ("Address", "Option<Address>")),
        (("company_params", "address_kana"), ("Address", "Option<Address>")),
        (("company_params", "address_kanji"), ("Address", "Option<Address>")),
        // (("company_params", "verification"), ("CompanyVerificationParams", "Option<CompanyVerificationParams>")),
        (("person_params", "address"), ("Address", "Option<Address>")),
        (("person_params", "address_kana"), ("Address", "Option<Address>")),
        (("person_params", "address_kanji"), ("Address", "Option<Address>")),
        (("person_params", "dob"), ("Dob", "Option<Dob>")),
        (("person_params", "verification"), ("PersonVerificationParams", "Option<PersonVerificationParams>")),
        (("company_verification_params", "document"), ("VerificationDocumentParams", "Option<VerificationDocumentParams>")),
        // (("person_verification_params", "document"), ("VerificationDocumentParams", "Option<VerificationDocumentParams>")),
        // (("person_verification_params", "additional_document"), ("VerificationDocumentParams", "Option<VerificationDocumentParams>")),

        // Config for `charge` params
        (("create_charge", "shipping"), ("Shipping", "Option<Shipping>")),
        (("create_charge", "source"), ("ChargeSourceParams", "Option<ChargeSourceParams>")),
        (("update_charge", "shipping"), ("Shipping", "Option<Shipping>")),
        (("fraud_details_params", "user_report"), ("FraudDetailsReport", "FraudDetailsReport")),

        // Config for `customer` params
        (("create_customer", "address"), ("Address", "Option<Address>")),
        (("update_customer", "address"), ("Address", "Option<Address>")),
        (
            ("update_customer", "default_alipay_account"),
            ("AlipayAccountId", "Option<AlipayAccountId>"),
        ),
        (("update_customer", "default_bank_account"), ("BankAccountId", "Option<BankAccountId>")),
        (("update_customer", "default_card"), ("CardId", "Option<CardId>")),
        (("create_customer", "default_source"), ("PaymentSourceId", "Option<PaymentSourceId>")),
        (("update_customer", "default_source"), ("PaymentSourceId", "Option<PaymentSourceId>")),
        (("create_customer", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("update_customer", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("create_customer", "source"), ("PaymentSourceParams", "Option<PaymentSourceParams>")),
        (("update_customer", "source"), ("PaymentSourceParams", "Option<PaymentSourceParams>")),
        (("update_customer", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (
            ("customer_invoice_settings", "custom_fields"),
            ("CustomField", "Option<Vec<CustomField>>"),
        ),

        // Config for `invoice` params
        (("list_invoices", "billing"), ("", "Option<CollectionMethod>")),
        (("create_invoice", "billing"), ("", "Option<CollectionMethod>")),
        (("create_invoice", "custom_fields"), ("CustomField", "Option<Vec<CustomField>>")),

        // Config for `invoiceitem` params
        (("create_invoice_item", "period"), ("Period", "Option<Period>")),
        (("update_invoice_item", "period"), ("Period", "Option<Period>")),

        // Config for `order` params
        (("list_orders", "status"), ("OrderStatusFilter", "Option<OrderStatusFilter>")),
        (("create_order", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("update_order", "shipping"), ("ShippingParams", "Option<ShippingParams>")),

        // Config for `payment_intent` params
        (("payment_intent", "source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("payment_intent_next_action", "use_stripe_sdk"), ("", "Option<serde_json::Value>")),
        (("create_payment_intent", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (
            ("create_payment_intent", "off_session"),
            ("Option<PaymentIntentOffSession>", "Option<PaymentIntentOffSession>"),
        ),
        (("update_payment_intent", "shipping"), ("ShippingParams", "Option<ShippingParams>")),
        (("create_setup_intent", "usage"), ("", "Option<SetupIntentUsage>")),
        (("setup_intent_next_action", "use_stripe_sdk"), ("", "Option<serde_json::Value>")),

        // Config for `sku` params
        (("list_skus", "attributes"), ("Metadata", "Option<Metadata>")),
        (("create_sku", "attributes"), ("Metadata", "Option<Metadata>")),
        (("update_sku", "attributes"), ("Metadata", "Option<Metadata>")),
        (("create_sku", "inventory"), ("Inventory", "Option<Inventory>")),
        (("update_sku", "inventory"), ("Inventory", "Option<Inventory>")),
        (("create_sku", "package_dimensions"), ("PackageDimensions", "Option<PackageDimensions>")),
        (("update_sku", "package_dimensions"), ("PackageDimensions", "Option<PackageDimensions>")),

        // Config for `source` params
        (("create_source", "owner"), ("BillingDetails", "Option<BillingDetails>")),
        (("update_source", "owner"), ("BillingDetails", "Option<BillingDetails>")),

        // Config for `subscription` params
        (
            ("create_subscription", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("update_subscription", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("create_subscription_item", "billing_thresholds"),
            ("SubscriptionItemBillingThresholds", "Option<SubscriptionItemBillingThresholds>"),
        ),
        (
            ("update_subscription_item", "billing_thresholds"),
            ("SubscriptionItemBillingThresholds", "Option<SubscriptionItemBillingThresholds>"),
        ),
        (
            ("update_subscription_items", "billing_thresholds"),
            ("SubscriptionItemBillingThresholds", "Option<SubscriptionItemBillingThresholds>"),
        ),
        (
            ("subscription_schedule_phases_plans_params", "billing_thresholds"),
            ("SubscriptionItemBillingThresholds", "Option<SubscriptionItemBillingThresholds>"),
        ),
        (
            ("list_subscriptions", "billing"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("create_subscription", "billing"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("update_subscription", "billing"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("list_subscriptions", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("create_subscription", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("update_subscription", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (("create_subscription", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription", "trial_end"), ("Scheduled", "Option<Scheduled>")),

        // Config for `subscription_schedule` params
        (("create_subscription_schedule", "collection_method"), ("CollectionMethod", "Option<CollectionMethod>")),
        (("update_subscription_schedule", "collection_method"), ("CollectionMethod", "Option<CollectionMethod>")),
        (("create_subscription_schedule", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (
            ("create_subscription_schedule", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("update_subscription_schedule", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("create_subscription_schedule", "billing"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("update_subscription_schedule", "billing"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("create_subscription_schedule_phases", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("update_subscription_schedule_phases", "billing_thresholds"),
            ("SubscriptionBillingThresholds", "Option<SubscriptionBillingThresholds>"),
        ),
        (
            ("create_subscription_schedule_phases", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (
            ("update_subscription_schedule_phases", "collection_method"),
            ("CollectionMethod", "Option<CollectionMethod>"),
        ),
        (("create_subscription_schedule_phases", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "end_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "end_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (
            ("subscription_schedule_renewal_interval_params", "interval"),
            ("PlanInterval", "PlanInterval"),
        ),
        (
            ("subscription_schedule_default_settings_params", "invoice_settings"),
            ("SubscriptionScheduleInvoiceSettings", "Option<SubscriptionScheduleInvoiceSettings>"),
        ),

        // Miscellaneous params
        (("create_recipient", "type"), ("", "RecipientType")),
        (
            ("update_payment_method", "billing_details"),
            ("BillingDetails", "Option<BillingDetails>"),
        ),
        (
            ("create_product", "package_dimensions"),
            ("PackageDimensions", "Option<PackageDimensions>"),
        ),
        (
            ("update_product", "package_dimensions"),
            ("PackageDimensions", "Option<PackageDimensions>"),
        ),
        (("create_plan_tiers", "up_to"), ("UpTo", "Option<UpTo>")),
        (("create_price_tiers", "up_to"), ("UpTo", "Option<UpTo>")),
        (("update_file_link", "expires_at"), ("Scheduled", "Option<Scheduled>")),
        (("create_token_account", "business_type"), ("BusinessType", "Option<BusinessType>")),
        (("create_token_account", "company"), ("CompanyParams", "Option<CompanyParams>")),
        (("create_token_account", "individual"), ("PersonParams", "Option<PersonParams>")),

        (("create_token_person", "address"), ("Address", "Option<Address>")),
        (("create_token_person", "address_kana"), ("Address", "Option<Address>")),
        (("create_token_person", "address_kanji"), ("Address", "Option<Address>")),
        (("create_token_person", "dob"), ("Dob", "Option<Dob>")),
        (
            ("create_payment_method", "billing_details"),
            ("BillingDetails", "Option<BillingDetails>"),
        ),
        (("transfer_schedule_params", "delay_days"), ("DelayDays", "Option<DelayDays>")),
        (("transfer_schedule_params", "weekly_anchor"), ("Weekday", "Option<Weekday>")),
        (("create_webhook_endpoint", "api_version"), ("ApiVersion", "Option<ApiVersion>")),
    ]
    .iter()
    .copied()
    .collect()
}
