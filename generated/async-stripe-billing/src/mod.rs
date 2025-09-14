#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(non_camel_case_types)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Billing` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_billing;

miniserde::make_place!(Place);
pub use billing_alert::types::*;
pub use stripe_shared::automatic_tax::*;
pub mod billing_alert;
#[doc(hidden)]
pub mod billing_alert_triggered;
#[doc(inline)]
pub use billing_alert_triggered::*;
pub use billing_credit_balance_summary::types::*;
pub mod billing_credit_balance_summary;
pub mod billing_credit_balance_transaction;
pub use stripe_shared::billing_credit_balance_transaction::*;
pub mod billing_credit_grant;
pub use billing_meter::types::*;
pub use stripe_shared::billing_credit_grant::*;
pub mod billing_meter;
pub use billing_meter_event::types::*;
pub mod billing_meter_event;
pub use billing_meter_event_adjustment::types::*;
pub mod billing_meter_event_adjustment;
pub use billing_meter_event_summary::types::*;
#[doc(hidden)]
pub mod billing_bill_resource_invoice_item_parents_invoice_item_parent;
pub mod billing_meter_event_summary;
#[doc(inline)]
pub use billing_bill_resource_invoice_item_parents_invoice_item_parent::*;
#[doc(hidden)]
pub mod billing_bill_resource_invoice_item_parents_invoice_item_subscription_parent;
#[doc(inline)]
pub use billing_bill_resource_invoice_item_parents_invoice_item_subscription_parent::*;pub use stripe_shared::billing_bill_resource_invoicing_lines_common_credited_items::*;pub use stripe_shared::billing_bill_resource_invoicing_lines_common_proration_details::*;pub use stripe_shared::billing_bill_resource_invoicing_lines_parents_invoice_line_item_invoice_item_parent::*;pub use stripe_shared::billing_bill_resource_invoicing_lines_parents_invoice_line_item_parent::*;pub use stripe_shared::billing_bill_resource_invoicing_lines_parents_invoice_line_item_subscription_item_parent::*;pub use stripe_shared::billing_bill_resource_invoicing_parents_invoice_parent::*;pub use stripe_shared::billing_bill_resource_invoicing_parents_invoice_quote_parent::*;pub use stripe_shared::billing_bill_resource_invoicing_parents_invoice_subscription_parent::*;pub use stripe_shared::billing_bill_resource_invoicing_pricing_pricing::*;pub use stripe_shared::billing_bill_resource_invoicing_pricing_pricing_price_details::*;pub use stripe_shared::billing_bill_resource_invoicing_taxes_tax::*;pub use stripe_shared::billing_bill_resource_invoicing_taxes_tax_rate_details::*;pub use stripe_shared::billing_clocks_resource_status_details_advancing_status_details::*;pub use stripe_shared::billing_clocks_resource_status_details_status_details::*;pub use stripe_shared::billing_credit_grants_resource_amount::*;pub use stripe_shared::billing_credit_grants_resource_applicability_config::*;pub use stripe_shared::billing_credit_grants_resource_applicable_price::*;pub use stripe_shared::billing_credit_grants_resource_balance_credit::*;pub use stripe_shared::billing_credit_grants_resource_balance_credits_application_invoice_voided::*;pub use stripe_shared::billing_credit_grants_resource_balance_credits_applied::*;pub use stripe_shared::billing_credit_grants_resource_balance_debit::*;pub use stripe_shared::billing_credit_grants_resource_monetary_amount::*;pub use stripe_shared::billing_credit_grants_resource_scope::*;
#[doc(hidden)]
pub mod billing_meter_resource_aggregation_settings;
#[doc(inline)]
pub use billing_meter_resource_aggregation_settings::*;
#[doc(hidden)]
pub mod billing_meter_resource_billing_meter_event_adjustment_cancel;
#[doc(inline)]
pub use billing_meter_resource_billing_meter_event_adjustment_cancel::*;
#[doc(hidden)]
pub mod billing_meter_resource_billing_meter_status_transitions;
#[doc(inline)]
pub use billing_meter_resource_billing_meter_status_transitions::*;
#[doc(hidden)]
pub mod billing_meter_resource_billing_meter_value;
#[doc(inline)]
pub use billing_meter_resource_billing_meter_value::*;
#[doc(hidden)]
pub mod billing_meter_resource_customer_mapping_settings;
#[doc(inline)]
pub use billing_meter_resource_customer_mapping_settings::*;
pub use billing_portal_configuration::types::*;
pub mod billing_portal_configuration;
pub use billing_portal_session::types::*;
pub mod billing_portal_session;
pub use stripe_shared::cancellation_details::*;
#[doc(hidden)]
pub mod credit_balance;
#[doc(inline)]
pub use credit_balance::*;
pub mod credit_note;
pub use stripe_shared::credit_note::*;
pub mod credit_note_line_item;
pub use stripe_shared::credit_note_line_item::*;
pub use stripe_shared::credit_note_refund::*;
pub use stripe_shared::credit_notes_pretax_credit_amount::*;
pub use stripe_shared::deleted_invoice::*;
#[doc(hidden)]
pub mod deleted_invoiceitem;
#[doc(inline)]
pub use deleted_invoiceitem::*;
pub use stripe_shared::deleted_plan::*;
pub use stripe_shared::deleted_subscription_item::*;
pub use stripe_shared::deleted_tax_id::*;
pub use stripe_shared::deleted_test_helpers_test_clock::*;
pub use stripe_shared::discounts_resource_discount_amount::*;
pub use stripe_shared::discounts_resource_stackable_discount::*;
pub mod invoice;
pub use stripe_shared::invoice::*;
pub use stripe_shared::invoice_installments_card::*;
pub use stripe_shared::invoice_item_threshold_reason::*;
pub use stripe_shared::invoice_line_item_period::*;
pub use stripe_shared::invoice_mandate_options_card::*;
pub mod invoice_payment;
pub use stripe_shared::invoice_payment::*;pub use stripe_shared::invoice_payment_method_options_acss_debit::*;pub use stripe_shared::invoice_payment_method_options_acss_debit_mandate_options::*;pub use stripe_shared::invoice_payment_method_options_bancontact::*;pub use stripe_shared::invoice_payment_method_options_card::*;pub use stripe_shared::invoice_payment_method_options_customer_balance::*;pub use stripe_shared::invoice_payment_method_options_customer_balance_bank_transfer::*;pub use stripe_shared::invoice_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer::*;pub use stripe_shared::invoice_payment_method_options_konbini::*;pub use stripe_shared::invoice_payment_method_options_sepa_debit::*;pub use stripe_shared::invoice_payment_method_options_us_bank_account::*;pub use stripe_shared::invoice_payment_method_options_us_bank_account_linked_account_options::*;pub use stripe_shared::invoice_payment_method_options_us_bank_account_linked_account_options_filters::*;pub use stripe_shared::invoice_rendering_pdf::*;pub use invoice_rendering_template::types::*;
pub mod invoice_rendering_template;
pub use stripe_shared::invoice_setting_checkout_rendering_options::*;
pub use stripe_shared::invoice_setting_custom_field::*;
pub use stripe_shared::invoice_setting_customer_rendering_options::*;
pub use stripe_shared::invoice_setting_customer_setting::*;
#[doc(hidden)]
pub mod invoice_setting_quote_setting;
pub use invoice_item::types::*;
#[doc(inline)]
pub use invoice_setting_quote_setting::*;
pub use stripe_shared::invoice_setting_subscription_schedule_phase_setting::*;
pub use stripe_shared::invoice_setting_subscription_schedule_setting::*;
pub use stripe_shared::invoice_threshold_reason::*;
pub mod invoice_item;
pub use stripe_shared::checkout_session_item::*;
pub use stripe_shared::invoices_payment_method_options::*;
pub use stripe_shared::invoices_payment_settings::*;
pub use stripe_shared::invoices_payments_invoice_payment_associated_payment::*;
pub use stripe_shared::invoices_payments_invoice_payment_status_transitions::*;
pub use stripe_shared::invoices_resource_confirmation_secret::*;
pub use stripe_shared::invoices_resource_from_invoice::*;
pub use stripe_shared::invoices_resource_invoice_rendering::*;
pub use stripe_shared::invoices_resource_invoice_tax_id::*;
pub use stripe_shared::invoices_resource_pretax_credit_amount::*;
pub use stripe_shared::invoices_resource_shipping_cost::*;
pub use stripe_shared::invoices_resource_status_transitions::*;
pub mod invoice_line_item;
pub use stripe_shared::invoice_line_item::*;
pub mod plan;
pub use stripe_shared::plan::*;
pub use stripe_shared::plan_tier::*;
#[doc(hidden)]
pub mod portal_business_profile;
#[doc(inline)]
pub use portal_business_profile::*;
#[doc(hidden)]
pub mod portal_customer_update;
#[doc(inline)]
pub use portal_customer_update::*;
#[doc(hidden)]
pub mod portal_features;
#[doc(inline)]
pub use portal_features::*;
#[doc(hidden)]
pub mod portal_flows_after_completion_hosted_confirmation;
#[doc(inline)]
pub use portal_flows_after_completion_hosted_confirmation::*;
#[doc(hidden)]
pub mod portal_flows_after_completion_redirect;
#[doc(inline)]
pub use portal_flows_after_completion_redirect::*;
#[doc(hidden)]
pub mod portal_flows_coupon_offer;
#[doc(inline)]
pub use portal_flows_coupon_offer::*;
#[doc(hidden)]
pub mod portal_flows_flow;
#[doc(inline)]
pub use portal_flows_flow::*;
#[doc(hidden)]
pub mod portal_flows_flow_after_completion;
#[doc(inline)]
pub use portal_flows_flow_after_completion::*;
#[doc(hidden)]
pub mod portal_flows_flow_subscription_cancel;
#[doc(inline)]
pub use portal_flows_flow_subscription_cancel::*;
#[doc(hidden)]
pub mod portal_flows_flow_subscription_update;
#[doc(inline)]
pub use portal_flows_flow_subscription_update::*;
#[doc(hidden)]
pub mod portal_flows_flow_subscription_update_confirm;
#[doc(inline)]
pub use portal_flows_flow_subscription_update_confirm::*;
#[doc(hidden)]
pub mod portal_flows_retention;
#[doc(inline)]
pub use portal_flows_retention::*;
#[doc(hidden)]
pub mod portal_flows_subscription_update_confirm_discount;
#[doc(inline)]
pub use portal_flows_subscription_update_confirm_discount::*;
#[doc(hidden)]
pub mod portal_flows_subscription_update_confirm_item;
#[doc(inline)]
pub use portal_flows_subscription_update_confirm_item::*;
#[doc(hidden)]
pub mod portal_invoice_list;
#[doc(inline)]
pub use portal_invoice_list::*;
#[doc(hidden)]
pub mod portal_login_page;
#[doc(inline)]
pub use portal_login_page::*;
#[doc(hidden)]
pub mod portal_payment_method_update;
#[doc(inline)]
pub use portal_payment_method_update::*;
#[doc(hidden)]
pub mod portal_resource_schedule_update_at_period_end;
#[doc(inline)]
pub use portal_resource_schedule_update_at_period_end::*;
#[doc(hidden)]
pub mod portal_resource_schedule_update_at_period_end_condition;
#[doc(inline)]
pub use portal_resource_schedule_update_at_period_end_condition::*;
#[doc(hidden)]
pub mod portal_subscription_cancel;
#[doc(inline)]
pub use portal_subscription_cancel::*;
#[doc(hidden)]
pub mod portal_subscription_cancellation_reason;
#[doc(inline)]
pub use portal_subscription_cancellation_reason::*;
#[doc(hidden)]
pub mod portal_subscription_update;
#[doc(inline)]
pub use portal_subscription_update::*;
#[doc(hidden)]
pub mod portal_subscription_update_product;
#[doc(inline)]
pub use portal_subscription_update_product::*;
#[doc(hidden)]
pub mod portal_subscription_update_product_adjustable_quantity;
#[doc(inline)]
pub use portal_subscription_update_product_adjustable_quantity::*;
pub use quote::types::*;
pub mod quote;
#[doc(hidden)]
pub mod quotes_resource_automatic_tax;
#[doc(inline)]
pub use quotes_resource_automatic_tax::*;
#[doc(hidden)]
pub mod quotes_resource_computed;
#[doc(inline)]
pub use quotes_resource_computed::*;
#[doc(hidden)]
pub mod quotes_resource_from_quote;
#[doc(inline)]
pub use quotes_resource_from_quote::*;
#[doc(hidden)]
pub mod quotes_resource_recurring;
#[doc(inline)]
pub use quotes_resource_recurring::*;
#[doc(hidden)]
pub mod quotes_resource_status_transitions;
#[doc(inline)]
pub use quotes_resource_status_transitions::*;
#[doc(hidden)]
pub mod quotes_resource_subscription_data_billing_mode;
#[doc(inline)]
pub use quotes_resource_subscription_data_billing_mode::*;
#[doc(hidden)]
pub mod quotes_resource_subscription_data_subscription_data;
#[doc(inline)]
pub use quotes_resource_subscription_data_subscription_data::*;
#[doc(hidden)]
pub mod quotes_resource_total_details;
#[doc(inline)]
pub use quotes_resource_total_details::*;
#[doc(hidden)]
pub mod quotes_resource_total_details_resource_breakdown;
#[doc(inline)]
pub use quotes_resource_total_details_resource_breakdown::*;
#[doc(hidden)]
pub mod quotes_resource_transfer_data;
#[doc(inline)]
pub use quotes_resource_transfer_data::*;
#[doc(hidden)]
pub mod quotes_resource_upfront;
#[doc(inline)]
pub use quotes_resource_upfront::*;
pub use stripe_shared::schedules_phase_automatic_tax::*;
pub mod subscription;
pub use stripe_shared::subscription::*;
pub use stripe_shared::subscription_automatic_tax::*;
pub use stripe_shared::subscription_billing_thresholds::*;
pub mod subscription_item;
pub use stripe_shared::subscription_item::*;
pub use stripe_shared::subscription_item_billing_thresholds::*;
pub use stripe_shared::subscription_payment_method_options_card::*;
pub use stripe_shared::subscription_pending_invoice_item_interval::*;
pub mod subscription_schedule;
pub use stripe_shared::subscription_schedule::*;
pub use stripe_shared::subscription_schedule_add_invoice_item::*;
pub use stripe_shared::subscription_schedule_add_invoice_item_period::*;
pub use stripe_shared::subscription_schedule_configuration_item::*;
pub use stripe_shared::subscription_schedule_current_phase::*;
pub use stripe_shared::subscription_schedule_phase_configuration::*;
pub use stripe_shared::subscription_schedules_resource_default_settings::*;
pub use stripe_shared::subscription_schedules_resource_default_settings_automatic_tax::*;
pub use stripe_shared::subscription_schedules_resource_invoice_item_period_resource_period_end::*;
pub use stripe_shared::subscription_schedules_resource_invoice_item_period_resource_period_start::*;
pub use stripe_shared::subscription_transfer_data::*;
pub use stripe_shared::subscriptions_resource_billing_cycle_anchor_config::*;
pub use stripe_shared::subscriptions_resource_billing_mode::*;
pub use stripe_shared::subscriptions_resource_pause_collection::*;
pub use stripe_shared::subscriptions_resource_payment_method_options::*;
pub use stripe_shared::subscriptions_resource_payment_settings::*;
pub use stripe_shared::subscriptions_resource_pending_update::*;
pub use stripe_shared::subscriptions_resource_subscription_invoice_settings::*;
pub use stripe_shared::tax_i_ds_owner::*;
pub mod tax_id;
pub use stripe_shared::tax_id::*;
pub use stripe_shared::tax_id_verification::*;
pub mod test_helpers_test_clock;
pub use stripe_shared::test_helpers_test_clock::*;
#[doc(hidden)]
pub mod thresholds_resource_usage_alert_filter;
#[doc(inline)]
pub use thresholds_resource_usage_alert_filter::*;
#[doc(hidden)]
pub mod thresholds_resource_usage_threshold_config;
pub use stripe_shared::transform_usage::*;
#[doc(inline)]
pub use thresholds_resource_usage_threshold_config::*;
