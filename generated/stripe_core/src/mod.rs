#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Core Resources` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_core;

miniserde::make_place!(Place);
pub use balance::types::*;
pub mod balance;
#[doc(hidden)]
pub mod balance_amount;
#[doc(inline)]
pub use balance_amount::*;
#[doc(hidden)]
pub mod balance_amount_by_source_type;
#[doc(inline)]
pub use balance_amount_by_source_type::*;
#[doc(hidden)]
pub mod balance_amount_net;
#[doc(inline)]
pub use balance_amount_net::*;
#[doc(hidden)]
pub mod balance_detail;
#[doc(inline)]
pub use balance_detail::*;
pub mod balance_transaction;
pub use stripe_shared::balance_transaction::*;
pub use stripe_shared::balance_transaction_source::*;
pub mod cash_balance;
pub use stripe_shared::cash_balance::*;
pub mod charge;
pub use stripe_shared::charge::*;
pub use stripe_shared::charge_fraud_details::*;
pub use stripe_shared::charge_outcome::*;
pub use stripe_shared::charge_transfer_data::*;
pub use stripe_shared::connect_collection_transfer::*;
pub mod customer;
pub use stripe_shared::customer::*;pub use stripe_shared::customer_acceptance::*;pub use stripe_shared::customer_balance_customer_balance_settings::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_adjusted_for_overdraft::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_applied_to_payment_transaction::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_funded_transaction::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_funded_transaction_resource_bank_transfer::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_funded_transaction_resource_bank_transfer_resource_eu_bank_transfer::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_funded_transaction_resource_bank_transfer_resource_gb_bank_transfer::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_funded_transaction_resource_bank_transfer_resource_jp_bank_transfer::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_funded_transaction_resource_bank_transfer_resource_us_bank_transfer::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_refunded_from_payment_transaction::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_transferred_to_balance::*;pub use stripe_shared::customer_balance_resource_cash_balance_transaction_resource_unapplied_from_payment_transaction::*;
pub mod customer_balance_transaction;
pub use stripe_shared::customer_balance_transaction::*;
pub mod customer_cash_balance_transaction;
pub use customer_session::types::*;
pub use stripe_shared::customer_cash_balance_transaction::*;
pub mod customer_session;
#[doc(hidden)]
pub mod customer_session_resource_components;
#[doc(inline)]
pub use customer_session_resource_components::*;
#[doc(hidden)]
pub mod customer_session_resource_components_resource_buy_button;
#[doc(inline)]
pub use customer_session_resource_components_resource_buy_button::*;
#[doc(hidden)]
pub mod customer_session_resource_components_resource_pricing_table;
#[doc(inline)]
pub use customer_session_resource_components_resource_pricing_table::*;
pub use stripe_shared::customer_tax::*;
pub use stripe_shared::customer_tax_location::*;
pub use stripe_shared::deleted_customer::*;
pub use stripe_shared::destination_details_unimplemented::*;
pub mod dispute;
pub use stripe_shared::dispute::*;
pub use stripe_shared::dispute_evidence::*;
pub use stripe_shared::dispute_evidence_details::*;
pub use stripe_shared::dispute_payment_method_details::*;
pub use stripe_shared::dispute_payment_method_details_card::*;
pub use stripe_shared::email_sent::*;
pub mod event;
pub use stripe_shared::event::*;
pub use stripe_shared::fee::*;
pub mod file;
pub use stripe_shared::file::*;
pub mod file_link;
pub use stripe_shared::file_link::*;
pub use stripe_shared::level3::*;
pub use stripe_shared::level3_line_items::*;
pub mod mandate;
pub use stripe_shared::mandate::*;
pub use stripe_shared::mandate_acss_debit::*;
pub use stripe_shared::mandate_au_becs_debit::*;
pub use stripe_shared::mandate_bacs_debit::*;
pub use stripe_shared::mandate_cashapp::*;
pub use stripe_shared::mandate_link::*;
pub use stripe_shared::mandate_multi_use::*;
pub use stripe_shared::mandate_payment_method_details::*;
pub use stripe_shared::mandate_paypal::*;
pub use stripe_shared::mandate_sepa_debit::*;
pub use stripe_shared::mandate_single_use::*;
pub use stripe_shared::mandate_us_bank_account::*;
pub use stripe_shared::notification_event_data::*;
pub use stripe_shared::notification_event_request::*;
pub use stripe_shared::offline_acceptance::*;
pub use stripe_shared::online_acceptance::*;
pub use stripe_shared::payment_flows_amount_details::*;
pub use stripe_shared::payment_flows_amount_details_resource_tip::*;
pub use stripe_shared::payment_flows_automatic_payment_methods_payment_intent::*;
pub use stripe_shared::payment_flows_automatic_payment_methods_setup_intent::*;
pub mod payment_intent;
pub use stripe_shared::payment_intent::*;
pub use stripe_shared::payment_intent_card_processing::*;
pub use stripe_shared::payment_intent_next_action::*;
pub use stripe_shared::payment_intent_next_action_alipay_handle_redirect::*;
pub use stripe_shared::payment_intent_next_action_boleto::*;
pub use stripe_shared::payment_intent_next_action_card_await_notification::*;
pub use stripe_shared::payment_intent_next_action_cashapp_handle_redirect_or_display_qr_code::*;
pub use stripe_shared::payment_intent_next_action_cashapp_qr_code::*;
pub use stripe_shared::payment_intent_next_action_display_bank_transfer_instructions::*;
pub use stripe_shared::payment_intent_next_action_display_oxxo_details::*;
pub use stripe_shared::payment_intent_next_action_konbini::*;
pub use stripe_shared::payment_intent_next_action_konbini_familymart::*;
pub use stripe_shared::payment_intent_next_action_konbini_lawson::*;
pub use stripe_shared::payment_intent_next_action_konbini_ministop::*;
pub use stripe_shared::payment_intent_next_action_konbini_seicomart::*;
pub use stripe_shared::payment_intent_next_action_konbini_stores::*;
pub use stripe_shared::payment_intent_next_action_paynow_display_qr_code::*;
pub use stripe_shared::payment_intent_next_action_pix_display_qr_code::*;
pub use stripe_shared::payment_intent_next_action_promptpay_display_qr_code::*;
pub use stripe_shared::payment_intent_next_action_redirect_to_url::*;
pub use stripe_shared::payment_intent_next_action_swish_handle_redirect_or_display_qr_code::*;
pub use stripe_shared::payment_intent_next_action_swish_qr_code::*;
pub use stripe_shared::payment_intent_next_action_verify_with_microdeposits::*;
pub use stripe_shared::payment_intent_next_action_wechat_pay_display_qr_code::*;
pub use stripe_shared::payment_intent_next_action_wechat_pay_redirect_to_android_app::*;
pub use stripe_shared::payment_intent_next_action_wechat_pay_redirect_to_ios_app::*;
pub use stripe_shared::payment_intent_payment_method_options::*;
pub use stripe_shared::payment_intent_payment_method_options_acss_debit::*;
pub use stripe_shared::payment_intent_payment_method_options_au_becs_debit::*;
pub use stripe_shared::payment_intent_payment_method_options_blik::*;
pub use stripe_shared::payment_intent_payment_method_options_card::*;
pub use stripe_shared::payment_intent_payment_method_options_eps::*;
pub use stripe_shared::payment_intent_payment_method_options_link::*;
pub use stripe_shared::payment_intent_payment_method_options_mandate_options_acss_debit::*;
pub use stripe_shared::payment_intent_payment_method_options_mandate_options_sepa_debit::*;
pub use stripe_shared::payment_intent_payment_method_options_sepa_debit::*;
pub use stripe_shared::payment_intent_payment_method_options_swish::*;
pub use stripe_shared::payment_intent_payment_method_options_us_bank_account::*;
pub use stripe_shared::payment_intent_processing::*;
pub use stripe_shared::payment_intent_processing_customer_notification::*;
pub mod payment_source;
pub use stripe_shared::payment_source::*;
pub mod payout;
pub use stripe_shared::payout::*;
pub use stripe_shared::platform_tax_fee::*;
pub mod refund;
pub use stripe_shared::refund::*;
pub use stripe_shared::refund_destination_details::*;
pub use stripe_shared::refund_destination_details_card::*;
pub use stripe_shared::refund_destination_details_generic::*;
pub use stripe_shared::refund_next_action::*;
pub use stripe_shared::refund_next_action_display_details::*;
pub use stripe_shared::reserve_transaction::*;
pub mod setup_attempt;
pub use stripe_shared::setup_attempt::*;
pub use stripe_shared::setup_attempt_payment_method_details::*;
pub use stripe_shared::setup_attempt_payment_method_details_acss_debit::*;
pub use stripe_shared::setup_attempt_payment_method_details_au_becs_debit::*;
pub use stripe_shared::setup_attempt_payment_method_details_bacs_debit::*;
pub use stripe_shared::setup_attempt_payment_method_details_bancontact::*;
pub use stripe_shared::setup_attempt_payment_method_details_boleto::*;
pub use stripe_shared::setup_attempt_payment_method_details_card::*;
pub use stripe_shared::setup_attempt_payment_method_details_card_checks::*;
pub use stripe_shared::setup_attempt_payment_method_details_card_present::*;
pub use stripe_shared::setup_attempt_payment_method_details_card_wallet::*;
pub use stripe_shared::setup_attempt_payment_method_details_cashapp::*;
pub use stripe_shared::setup_attempt_payment_method_details_ideal::*;
pub use stripe_shared::setup_attempt_payment_method_details_klarna::*;
pub use stripe_shared::setup_attempt_payment_method_details_link::*;
pub use stripe_shared::setup_attempt_payment_method_details_paypal::*;
pub use stripe_shared::setup_attempt_payment_method_details_sepa_debit::*;
pub use stripe_shared::setup_attempt_payment_method_details_sofort::*;
pub use stripe_shared::setup_attempt_payment_method_details_us_bank_account::*;
pub mod setup_intent;
pub use stripe_shared::setup_intent::*;
pub use stripe_shared::setup_intent_next_action::*;
pub use stripe_shared::setup_intent_next_action_redirect_to_url::*;
pub use stripe_shared::setup_intent_next_action_verify_with_microdeposits::*;
pub use stripe_shared::setup_intent_payment_method_options::*;
pub use stripe_shared::setup_intent_payment_method_options_acss_debit::*;
pub use stripe_shared::setup_intent_payment_method_options_card::*;
pub use stripe_shared::setup_intent_payment_method_options_card_mandate_options::*;
pub use stripe_shared::setup_intent_payment_method_options_link::*;
pub use stripe_shared::setup_intent_payment_method_options_mandate_options_acss_debit::*;
pub use stripe_shared::setup_intent_payment_method_options_mandate_options_sepa_debit::*;
pub use stripe_shared::setup_intent_payment_method_options_paypal::*;
pub use stripe_shared::setup_intent_payment_method_options_sepa_debit::*;
pub use stripe_shared::setup_intent_payment_method_options_us_bank_account::*;
pub use stripe_shared::tax_deducted_at_source::*;
pub use stripe_shared::three_d_secure_details::*;
pub use token::types::*;
pub mod token;
