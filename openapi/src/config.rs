use std::collections::BTreeMap;

pub fn id_renames() -> BTreeMap<&'static str, &'static str> {
    [
        ("fee_refund", "application_fee_refund"),
        ("invoiceitem", "invoice_item"),
        ("line_item", "invoice_line_item"),
        ("source_transaction", "charge"),
    ]
    .into_iter()
    .copied()
    .collect()
}

#[rustfmt::skip]
pub fn schema_renames() -> BTreeMap<&'static str, &'static str> {
    [
        ("account_business_profile", "business_profile"),
        ("account_business_type", "business_type"),
        ("account_capabilities_card_payments", "capability_status"),
        ("account_capabilities_legacy_payments", "capability_status"),
        ("account_capabilities_platform_payments", "capability_status"),
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
        ("issuing.authorization_wallet_provider", "wallet_provider"),
        ("invoiceitem", "invoice_item"),
        ("legal_entity_company", "company"),
        ("legal_entity_japan_address", "address"),
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

        // Config for `account` params
        ("create_account_company", "company_params"),
        ("update_account_company", "company_params"),
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
        ("person_params_verification", "person_verification_params"),
        ("person_verification_params_document", "person_verification_document_params"),
        ("transfer_schedule_params_interval", "transfer_schedule_interval"),

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

        // Config for `payment_intent` params
        ("create_order_items", "order_item_params"),
        ("order_items_params_type", "order_item_type"),

        // Config for `source` params
        ("create_source_mandate", "source_mandate_params"),
        ("update_source_mandate", "source_mandate_params"),
        ("source_mandate_params_acceptance", "source_acceptance_params"),
        ("source_mandate_params_interval", "source_mandate_interval"),
        ("source_mandate_params_notification_method", "source_mandate_notification_method"),
        ("source_acceptance_params_offline", "source_acceptance_offline_params"),
        ("source_acceptance_params_online", "source_acceptance_online_params"),
        ("create_source_receiver_refund_attributes_method", "source_refund_notification_method"),

        // Config for `webhook`
        ("webhook_endpoint_enabled_events", "event_filter"),
        ("webhook_endpoint_api_version", "api_version"),
        ("create_webhook_endpoint_enabled_events", "event_filter"),
        ("update_webhook_endpoint_enabled_events", "event_filter"),
    ]
    .into_iter()
    .copied()
    .collect()
}

pub type FieldSpec = (&'static str, &'static str);

#[rustfmt::skip]
pub fn field_overrides() -> BTreeMap<FieldSpec, FieldSpec> {
    [
        (
            ("bank_account", "account_holder_type"),
            ("AccountHolderType", "Option<AccountHolderType>"),
        ),
        (("charge", "source"), ("PaymentSource", "Option<PaymentSource>")),
        (
            ("charge_fraud_details", "stripe_report"),
            ("FraudDetailsReport", "Option<FraudDetailsReport>"),
        ),
        (("customer", "default_source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("customer", "sources"), ("PaymentSource", "List<PaymentSource>")),
        (("invoice", "default_source"), ("PaymentSource", "Option<Expandable<PaymentSource>>")),
        (("invoiceitem", "period"), ("Period", "Option<Period>")),
        (("line_item", "period"), ("Period", "Option<Period>")),
        (
            ("issuing.authorization", "authorization_method"),
            ("IssuingAuthorizationMethod", "IssuingAuthorizationMethod"),
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
            ("issuing_authorization_verification_data", "cvc_check"),
            ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
        ),
        (("issuing.card", "brand"), ("CardBrand", "CardBrand")),
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
        (("create_setup_intent", "usage"), ("", "Option<SetupIntentUsage>")),
        (("setup_intent_next_action", "use_stripe_sdk"), ("", "Option<serde_json::Value>")),
        (("person", "dob"), ("Dob", "Option<Dob>")),
        (("sku", "attributes"), ("Metadata", "Option<Metadata>")),
        (
            ("subscription", "default_source"),
            ("PaymentSource", "Option<Expandable<PaymentSource>>"),
        ),
        (("transfer_schedule", "weekly_anchor"), ("Weekday", "Option<Weekday>")),

        // Config for `account` params
        (("create_account", "business_profile"), ("BusinessProfile", "Option<BusinessProfile>")),
        (("update_account", "business_profile"), ("BusinessProfile", "Option<BusinessProfile>")),
        (("company_params", "address"), ("Address", "Option<Address>")),
        (("company_params", "address_kana"), ("Address", "Option<Address>")),
        (("company_params", "address_kanji"), ("Address", "Option<Address>")),
        (("person_params", "address"), ("Address", "Option<Address>")),
        (("person_params", "address_kana"), ("Address", "Option<Address>")),
        (("person_params", "address_kanji"), ("Address", "Option<Address>")),
        (("person_params", "dob"), ("Dob", "Option<Dob>")),
        (
            ("create_payment_method", "billing_details"),
            ("BillingDetails", "Option<BillingDetails>"),
        ),
        (("transfer_schedule_params", "delay_days"), ("DelayDays", "Option<DelayDays>")),
        (("transfer_schedule_params", "weekly_anchor"), ("Weekday", "Option<Weekday>")),

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
        (("create_invoice", "custom_fields"), ("CustomField", "Option<Vec<CustomField>>")),

        // Config for `invoiceitem` params
        (("create_invoice_item", "period"), ("Period", "Option<Period>")),
        (("update_invoice_item", "period"), ("Period", "Option<Period>")),

        // Config for `order` params
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
            ("create_subscription_items", "billing_thresholds"),
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
        (("create_subscription", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription", "trial_end"), ("Scheduled", "Option<Scheduled>")),

        // Config for `subscription_schedule` params
        (("create_subscription_schedule", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "start_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "end_date"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "end_date"), ("Scheduled", "Option<Scheduled>")),
        (("create_subscription_schedule_phases", "trial_end"), ("Scheduled", "Option<Scheduled>")),
        (("update_subscription_schedule_phases", "trial_end"), ("Scheduled", "Option<Scheduled>")),

        // Miscellaneous
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
        (("update_file_link", "expires_at"), ("Scheduled", "Option<Scheduled>")),
        (("create_token_account", "business_type"), ("BusinessType", "Option<BusinessType>")),
        (("create_token_account", "company"), ("CompanyParams", "Option<CompanyParams>")),
        (("create_token_account", "individual"), ("PersonParams", "Option<PersonParams>")),
    ]
    .into_iter()
    .copied()
    .collect()
}
