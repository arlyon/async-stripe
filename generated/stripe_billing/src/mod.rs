#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Billing` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_billing;

miniserde::make_place!(Place);
pub use billing_meter::types::*;
pub use stripe_shared::automatic_tax::*;
pub mod billing_meter;
pub use billing_meter_event::types::*;
pub mod billing_meter_event;
pub use billing_meter_event_adjustment::types::*;
pub mod billing_meter_event_adjustment;
pub use billing_meter_event_summary::types::*;
pub mod billing_meter_event_summary;
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
pub mod credit_note;
pub use stripe_shared::credit_note::*;
pub mod credit_note_line_item;
pub use stripe_shared::credit_note_line_item::*;
pub use stripe_shared::credit_note_tax_amount::*;
pub use stripe_shared::deleted_invoice::*;
pub use stripe_shared::deleted_invoiceitem::*;
pub use stripe_shared::deleted_plan::*;
pub use stripe_shared::deleted_subscription_item::*;
pub use stripe_shared::deleted_tax_id::*;
pub use stripe_shared::deleted_test_helpers_test_clock::*;
pub use stripe_shared::discounts_resource_discount_amount::*;
pub use stripe_shared::discounts_resource_stackable_discount::*;
pub mod invoice;
pub use stripe_shared::invoice::*;pub use stripe_shared::invoice_installments_card::*;pub use stripe_shared::invoice_item_threshold_reason::*;pub use stripe_shared::invoice_line_item_period::*;pub use stripe_shared::invoice_mandate_options_card::*;pub use stripe_shared::invoice_payment_method_options_acss_debit::*;pub use stripe_shared::invoice_payment_method_options_acss_debit_mandate_options::*;pub use stripe_shared::invoice_payment_method_options_bancontact::*;pub use stripe_shared::invoice_payment_method_options_card::*;pub use stripe_shared::invoice_payment_method_options_customer_balance::*;pub use stripe_shared::invoice_payment_method_options_customer_balance_bank_transfer::*;pub use stripe_shared::invoice_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer::*;pub use stripe_shared::invoice_payment_method_options_konbini::*;pub use stripe_shared::invoice_payment_method_options_sepa_debit::*;pub use stripe_shared::invoice_payment_method_options_us_bank_account::*;pub use stripe_shared::invoice_payment_method_options_us_bank_account_linked_account_options::*;pub use stripe_shared::invoice_rendering_pdf::*;pub use stripe_shared::invoice_setting_custom_field::*;pub use stripe_shared::invoice_setting_customer_rendering_options::*;pub use stripe_shared::invoice_setting_customer_setting::*;pub use stripe_shared::invoice_setting_quote_setting::*;pub use stripe_shared::invoice_setting_rendering_options::*;pub use stripe_shared::invoice_setting_subscription_schedule_phase_setting::*;pub use stripe_shared::invoice_setting_subscription_schedule_setting::*;pub use stripe_shared::invoice_tax_amount::*;pub use stripe_shared::invoice_threshold_reason::*;pub use stripe_shared::invoice_transfer_data::*;
pub mod invoice_item;
pub use stripe_shared::checkout_session_item::*;
pub use stripe_shared::invoice_item::*;
pub use stripe_shared::invoices_payment_method_options::*;
pub use stripe_shared::invoices_payment_settings::*;
pub use stripe_shared::invoices_resource_from_invoice::*;
pub use stripe_shared::invoices_resource_invoice_rendering::*;
pub use stripe_shared::invoices_resource_invoice_tax_id::*;
pub use stripe_shared::invoices_resource_line_items_credited_items::*;
pub use stripe_shared::invoices_resource_line_items_proration_details::*;
pub use stripe_shared::invoices_resource_shipping_cost::*;
pub use stripe_shared::invoices_resource_status_transitions::*;
pub mod invoice_line_item;
pub use stripe_shared::invoice_line_item::*;
pub use stripe_shared::period::*;
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
pub mod quote;
pub use stripe_shared::quote::*;
pub use stripe_shared::quotes_resource_automatic_tax::*;
pub use stripe_shared::quotes_resource_computed::*;
pub use stripe_shared::quotes_resource_from_quote::*;
pub use stripe_shared::quotes_resource_recurring::*;
pub use stripe_shared::quotes_resource_status_transitions::*;
pub use stripe_shared::quotes_resource_subscription_data_subscription_data::*;
pub use stripe_shared::quotes_resource_total_details::*;
pub use stripe_shared::quotes_resource_total_details_resource_breakdown::*;
pub use stripe_shared::quotes_resource_transfer_data::*;
pub use stripe_shared::quotes_resource_upfront::*;
pub use stripe_shared::schedules_phase_automatic_tax::*;
pub mod subscription;
pub use stripe_shared::subscription::*;
pub use stripe_shared::subscription_automatic_tax::*;
pub use stripe_shared::subscription_billing_thresholds::*;
pub use stripe_shared::subscription_details_data::*;
pub mod subscription_item;
pub use stripe_shared::subscription_item::*;
pub use stripe_shared::subscription_item_billing_thresholds::*;
pub use stripe_shared::subscription_payment_method_options_card::*;
pub use stripe_shared::subscription_pending_invoice_item_interval::*;
pub mod subscription_schedule;
pub use stripe_shared::subscription_schedule::*;
pub use stripe_shared::subscription_schedule_add_invoice_item::*;
pub use stripe_shared::subscription_schedule_configuration_item::*;
pub use stripe_shared::subscription_schedule_current_phase::*;
pub use stripe_shared::subscription_schedule_phase_configuration::*;
pub use stripe_shared::subscription_schedules_resource_default_settings::*;
pub use stripe_shared::subscription_schedules_resource_default_settings_automatic_tax::*;
pub use stripe_shared::subscription_transfer_data::*;
pub use stripe_shared::subscriptions_resource_billing_cycle_anchor_config::*;
pub use stripe_shared::subscriptions_resource_pause_collection::*;
pub use stripe_shared::subscriptions_resource_payment_method_options::*;
pub use stripe_shared::subscriptions_resource_payment_settings::*;
pub use stripe_shared::subscriptions_resource_pending_update::*;
pub use stripe_shared::tax_i_ds_owner::*;
pub mod tax_id;
pub use stripe_shared::tax_id::*;
pub use stripe_shared::tax_id_verification::*;
pub mod test_helpers_test_clock;
pub use stripe_shared::test_helpers_test_clock::*;
pub use stripe_shared::transform_usage::*;
pub use usage_record::types::*;
pub mod usage_record;
pub mod usage_record_summary;
pub use stripe_shared::usage_record_summary::*;
