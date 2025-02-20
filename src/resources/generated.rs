//! generated module
//!
//! Contains the generated impls we use. This code
//! is automatically generated from the openapi spec
//! and should not be changed manually. To update the
//! spec, use cargo make.
//!
//! It is possible more files are generated than are
//! listed as modules here. These are modules that
//! have not yet been exposed by the client.

#[path = "generated"]
pub mod core {
    pub mod address;
    pub mod api_errors;
    pub mod balance;
    pub mod balance_amount_by_source_type;
    pub mod balance_transaction;
    pub mod billing_details;
    pub mod cash_balance;
    pub mod charge;
    pub mod connect_account_reference;
    pub mod custom_unit_amount;
    pub mod customer;
    pub mod customer_session;
    pub mod dispute;
    pub mod ephemeral_key;
    pub mod file;
    pub mod file_link;
    pub mod invoice_setting_rendering_options;
    pub mod linked_account_options_us_bank_account;
    pub mod mandate;
    pub mod payment_intent;
    pub mod payment_intent_next_action_cashapp_handle_redirect_or_display_qr_code;
    pub mod payment_method_config_biz_payment_method_configuration_details;
    pub mod payment_method_details_card_wallet_apple_pay;
    pub mod payment_method_details_card_wallet_google_pay;
    pub mod payment_method_options_customer_balance_eu_bank_account;
    pub mod payment_method_options_us_bank_account_mandate_options;
    pub mod payout;
    pub mod platform_tax_fee;
    pub mod price;
    pub mod product;
    pub mod radar_radar_options;
    pub mod refund;
    pub mod reserve_transaction;
    pub mod setup_attempt;
    pub mod setup_intent;
    pub mod shipping;
    pub mod shipping_rate;
    pub mod tax_code;
    pub mod tax_deducted_at_source;
    pub mod test_helpers_test_clock;
    pub mod token;
    pub mod version;
}

#[path = "generated"]
pub mod payment {
    pub mod bank_account;
    pub mod card;
    pub mod payment_method;
    pub mod payment_method_card_present_networks;
    pub mod source;
}

#[path = "generated"]
#[cfg(feature = "checkout")]
pub mod checkout {
    pub mod checkout_session;
    pub mod item;
    pub mod payment_link;
}

#[path = "generated"]
#[cfg(feature = "tax-calculation")]
pub mod tax_calculation {
    pub mod tax_calculation;
    pub mod tax_calculation_line_item;
    pub mod tax_product_resource_customer_details;
}

#[path = "generated"]
#[cfg(feature = "billing")]
pub mod billing {
    pub mod billing_portal_configuration;
    pub mod billing_portal_session;
    pub mod coupon;
    pub mod credit_note;
    pub mod credit_note_line_item;
    pub mod customer_balance_transaction;
    pub mod discount;
    pub mod invoice;
    pub mod invoice_payment_method_options_acss_debit;
    pub mod invoice_payment_method_options_bancontact;
    pub mod invoice_payment_method_options_customer_balance;
    pub mod invoice_payment_method_options_konbini;
    pub mod invoice_payment_method_options_us_bank_account;
    pub mod invoiceitem;
    pub mod invoices_shipping_cost;
    pub mod line_item;
    pub mod plan;
    pub mod promotion_code;
    pub mod quote;
    pub mod quotes_resource_total_details;
    pub mod subscription;
    pub mod subscription_billing_thresholds;
    pub mod subscription_item;
    pub mod subscription_item_billing_thresholds;
    pub mod subscription_schedule;
    pub mod subscriptions_trials_resource_trial_settings;
    pub mod tax_id;
    pub mod tax_rate;
    pub mod usage_record;
    pub mod usage_record_summary;
}

#[path = "generated"]
#[cfg(feature = "connect")]
pub mod connect {
    pub mod account;
    pub mod account_link;
    pub mod application;
    pub mod application_fee;
    pub mod connect_collection_transfer;
    pub mod fee_refund;
    pub mod login_link;
    pub mod person;
    pub mod topup;
    pub mod transfer;
    pub mod transfer_reversal;
}

#[path = "generated"]
#[cfg(feature = "fraud")]
pub mod fraud {
    pub mod review;
}

#[path = "generated"]
#[cfg(feature = "issuing")]
pub mod issuing {
    pub mod issuing_authorization;
    pub mod issuing_card;
    pub mod issuing_cardholder;
    pub mod issuing_dispute;
    pub mod issuing_token;
    pub mod issuing_transaction;
}

#[cfg(feature = "sigma")]
pub mod scheduled_query_run;

#[path = "generated"]
#[cfg(feature = "terminal")]
pub mod terminal {
    pub mod terminal_configuration;
    pub mod terminal_connection_token;
    pub mod terminal_location;
    pub mod terminal_reader;
}

#[cfg(feature = "events")]
pub mod event;

#[path = "generated"]
#[cfg(feature = "webhook-endpoints")]
pub mod webhook_endpoints {
    pub mod webhook_endpoint;
}

#[cfg(not(feature = "full"))]
pub mod placeholders;
