#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Payment Methods` and `Payment Links` sections
//! of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_payment;

miniserde::make_place!(Place);
pub mod bank_account;
pub use stripe_shared::bank_account::*;
pub mod card;
pub use stripe_shared::card::*;pub use stripe_shared::card_issuing_account_terms_of_service::*;pub use stripe_shared::card_mandate_payment_method_details::*;pub use stripe_shared::deleted_bank_account::*;pub use stripe_shared::deleted_card::*;pub use stripe_shared::deleted_external_account::*;pub use stripe_shared::deleted_payment_source::*;pub use stripe_shared::external_account_requirements::*;pub use stripe_shared::networks::*;pub use stripe_shared::payment_flows_private_payment_methods_alipay::*;pub use stripe_shared::payment_flows_private_payment_methods_alipay_details::*;pub use stripe_shared::payment_flows_private_payment_methods_card_details_api_resource_enterprise_features_extended_authorization_extended_authorization::*;pub use stripe_shared::payment_flows_private_payment_methods_card_details_api_resource_enterprise_features_incremental_authorization_incremental_authorization::*;pub use stripe_shared::payment_flows_private_payment_methods_card_details_api_resource_enterprise_features_overcapture_overcapture::*;pub use stripe_shared::payment_flows_private_payment_methods_card_details_api_resource_multicapture::*;pub use stripe_shared::payment_flows_private_payment_methods_klarna_dob::*;
pub mod payment_link;
pub use stripe_shared::payment_link::*;
pub use stripe_shared::payment_links_resource_after_completion::*;
pub use stripe_shared::payment_links_resource_automatic_tax::*;
pub use stripe_shared::payment_links_resource_completed_sessions::*;
pub use stripe_shared::payment_links_resource_completion_behavior_confirmation_page::*;
pub use stripe_shared::payment_links_resource_completion_behavior_redirect::*;
pub use stripe_shared::payment_links_resource_consent_collection::*;
pub use stripe_shared::payment_links_resource_custom_fields::*;
pub use stripe_shared::payment_links_resource_custom_fields_dropdown::*;
pub use stripe_shared::payment_links_resource_custom_fields_dropdown_option::*;
pub use stripe_shared::payment_links_resource_custom_fields_label::*;
pub use stripe_shared::payment_links_resource_custom_fields_numeric::*;
pub use stripe_shared::payment_links_resource_custom_fields_text::*;
pub use stripe_shared::payment_links_resource_custom_text::*;
pub use stripe_shared::payment_links_resource_custom_text_position::*;
pub use stripe_shared::payment_links_resource_invoice_creation::*;
pub use stripe_shared::payment_links_resource_invoice_settings::*;
pub use stripe_shared::payment_links_resource_payment_intent_data::*;
pub use stripe_shared::payment_links_resource_payment_method_reuse_agreement::*;
pub use stripe_shared::payment_links_resource_phone_number_collection::*;
pub use stripe_shared::payment_links_resource_restrictions::*;
pub use stripe_shared::payment_links_resource_shipping_address_collection::*;
pub use stripe_shared::payment_links_resource_shipping_option::*;
pub use stripe_shared::payment_links_resource_subscription_data::*;
pub use stripe_shared::payment_links_resource_subscription_data_invoice_settings::*;
pub use stripe_shared::payment_links_resource_tax_id_collection::*;
pub use stripe_shared::payment_links_resource_transfer_data::*;
pub mod payment_method;
pub use stripe_shared::payment_method::*;
pub use stripe_shared::payment_method_acss_debit::*;
pub use stripe_shared::payment_method_affirm::*;
pub use stripe_shared::payment_method_afterpay_clearpay::*;
pub use stripe_shared::payment_method_au_becs_debit::*;
pub use stripe_shared::payment_method_bacs_debit::*;
pub use stripe_shared::payment_method_bancontact::*;
pub use stripe_shared::payment_method_blik::*;
pub use stripe_shared::payment_method_boleto::*;
pub use stripe_shared::payment_method_card::*;
pub use stripe_shared::payment_method_card_checks::*;
pub use stripe_shared::payment_method_card_present::*;
pub use stripe_shared::payment_method_card_present_networks::*;
pub use stripe_shared::payment_method_card_wallet::*;
pub use stripe_shared::payment_method_card_wallet_amex_express_checkout::*;
pub use stripe_shared::payment_method_card_wallet_apple_pay::*;
pub use stripe_shared::payment_method_card_wallet_google_pay::*;
pub use stripe_shared::payment_method_card_wallet_link::*;
pub use stripe_shared::payment_method_card_wallet_masterpass::*;
pub use stripe_shared::payment_method_card_wallet_samsung_pay::*;
pub use stripe_shared::payment_method_card_wallet_visa_checkout::*;
pub use stripe_shared::payment_method_cashapp::*;
pub use stripe_shared::payment_method_config_biz_payment_method_configuration_details::*;
#[doc(hidden)]
pub mod payment_method_config_resource_display_preference;
#[doc(inline)]
pub use payment_method_config_resource_display_preference::*;
#[doc(hidden)]
pub mod payment_method_config_resource_payment_method_properties;
#[doc(inline)]
pub use payment_method_config_resource_payment_method_properties::*;
pub use payment_method_configuration::types::*;
pub mod payment_method_configuration;
pub use payment_method_domain::types::*;
pub use stripe_shared::payment_method_customer_balance::*;
pub use stripe_shared::payment_method_details::*;
pub use stripe_shared::payment_method_details_ach_credit_transfer::*;
pub use stripe_shared::payment_method_details_ach_debit::*;
pub use stripe_shared::payment_method_details_acss_debit::*;
pub use stripe_shared::payment_method_details_affirm::*;
pub use stripe_shared::payment_method_details_afterpay_clearpay::*;
pub use stripe_shared::payment_method_details_au_becs_debit::*;
pub use stripe_shared::payment_method_details_bacs_debit::*;
pub use stripe_shared::payment_method_details_bancontact::*;
pub use stripe_shared::payment_method_details_blik::*;
pub use stripe_shared::payment_method_details_boleto::*;
pub use stripe_shared::payment_method_details_card::*;
pub use stripe_shared::payment_method_details_card_checks::*;
pub use stripe_shared::payment_method_details_card_installments::*;
pub use stripe_shared::payment_method_details_card_installments_plan::*;
pub use stripe_shared::payment_method_details_card_network_token::*;
pub use stripe_shared::payment_method_details_card_present::*;
pub use stripe_shared::payment_method_details_card_present_offline::*;
pub use stripe_shared::payment_method_details_card_present_receipt::*;
pub use stripe_shared::payment_method_details_card_wallet::*;
pub use stripe_shared::payment_method_details_card_wallet_amex_express_checkout::*;
pub use stripe_shared::payment_method_details_card_wallet_apple_pay::*;
pub use stripe_shared::payment_method_details_card_wallet_google_pay::*;
pub use stripe_shared::payment_method_details_card_wallet_link::*;
pub use stripe_shared::payment_method_details_card_wallet_masterpass::*;
pub use stripe_shared::payment_method_details_card_wallet_samsung_pay::*;
pub use stripe_shared::payment_method_details_card_wallet_visa_checkout::*;
pub use stripe_shared::payment_method_details_cashapp::*;
pub use stripe_shared::payment_method_details_customer_balance::*;
pub use stripe_shared::payment_method_details_eps::*;
pub use stripe_shared::payment_method_details_fpx::*;
pub use stripe_shared::payment_method_details_giropay::*;
pub use stripe_shared::payment_method_details_grabpay::*;
pub use stripe_shared::payment_method_details_ideal::*;
pub use stripe_shared::payment_method_details_interac_present::*;
pub use stripe_shared::payment_method_details_interac_present_receipt::*;
pub use stripe_shared::payment_method_details_klarna::*;
pub use stripe_shared::payment_method_details_konbini::*;
pub use stripe_shared::payment_method_details_konbini_store::*;
pub use stripe_shared::payment_method_details_link::*;
pub use stripe_shared::payment_method_details_multibanco::*;
pub use stripe_shared::payment_method_details_oxxo::*;
pub use stripe_shared::payment_method_details_p24::*;
pub use stripe_shared::payment_method_details_paynow::*;
pub use stripe_shared::payment_method_details_paypal::*;
pub use stripe_shared::payment_method_details_pix::*;
pub use stripe_shared::payment_method_details_promptpay::*;
pub use stripe_shared::payment_method_details_revolut_pay::*;
pub use stripe_shared::payment_method_details_sepa_credit_transfer::*;
pub use stripe_shared::payment_method_details_sepa_debit::*;
pub use stripe_shared::payment_method_details_sofort::*;
pub use stripe_shared::payment_method_details_stripe_account::*;
pub use stripe_shared::payment_method_details_swish::*;
pub use stripe_shared::payment_method_details_us_bank_account::*;
pub use stripe_shared::payment_method_details_wechat::*;
pub use stripe_shared::payment_method_details_wechat_pay::*;
pub use stripe_shared::payment_method_details_zip::*;
pub mod payment_method_domain;
#[doc(hidden)]
pub mod payment_method_domain_resource_payment_method_status;
#[doc(inline)]
pub use payment_method_domain_resource_payment_method_status::*;
#[doc(hidden)]
pub mod payment_method_domain_resource_payment_method_status_details;
#[doc(inline)]
pub use payment_method_domain_resource_payment_method_status_details::*;
pub use stripe_shared::payment_method_eps::*;
pub use stripe_shared::payment_method_fpx::*;
pub use stripe_shared::payment_method_giropay::*;
pub use stripe_shared::payment_method_grabpay::*;
pub use stripe_shared::payment_method_ideal::*;
pub use stripe_shared::payment_method_interac_present::*;
pub use stripe_shared::payment_method_klarna::*;
pub use stripe_shared::payment_method_konbini::*;
pub use stripe_shared::payment_method_link::*;
pub use stripe_shared::payment_method_options_affirm::*;
pub use stripe_shared::payment_method_options_afterpay_clearpay::*;
pub use stripe_shared::payment_method_options_alipay::*;
pub use stripe_shared::payment_method_options_bacs_debit::*;
pub use stripe_shared::payment_method_options_bancontact::*;
pub use stripe_shared::payment_method_options_boleto::*;
pub use stripe_shared::payment_method_options_card_installments::*;
pub use stripe_shared::payment_method_options_card_mandate_options::*;
pub use stripe_shared::payment_method_options_card_present::*;
pub use stripe_shared::payment_method_options_cashapp::*;
pub use stripe_shared::payment_method_options_customer_balance::*;
pub use stripe_shared::payment_method_options_customer_balance_bank_transfer::*;
pub use stripe_shared::payment_method_options_customer_balance_eu_bank_account::*;
pub use stripe_shared::payment_method_options_fpx::*;
pub use stripe_shared::payment_method_options_giropay::*;
pub use stripe_shared::payment_method_options_grabpay::*;
pub use stripe_shared::payment_method_options_ideal::*;
pub use stripe_shared::payment_method_options_interac_present::*;
pub use stripe_shared::payment_method_options_klarna::*;
pub use stripe_shared::payment_method_options_konbini::*;
pub use stripe_shared::payment_method_options_oxxo::*;
pub use stripe_shared::payment_method_options_p24::*;
pub use stripe_shared::payment_method_options_paynow::*;
pub use stripe_shared::payment_method_options_paypal::*;
pub use stripe_shared::payment_method_options_pix::*;
pub use stripe_shared::payment_method_options_promptpay::*;
pub use stripe_shared::payment_method_options_revolut_pay::*;
pub use stripe_shared::payment_method_options_sofort::*;
pub use stripe_shared::payment_method_options_us_bank_account_mandate_options::*;
pub use stripe_shared::payment_method_options_wechat_pay::*;
pub use stripe_shared::payment_method_options_zip::*;
pub use stripe_shared::payment_method_oxxo::*;
pub use stripe_shared::payment_method_p24::*;
pub use stripe_shared::payment_method_paynow::*;
pub use stripe_shared::payment_method_paypal::*;
pub use stripe_shared::payment_method_pix::*;
pub use stripe_shared::payment_method_promptpay::*;
pub use stripe_shared::payment_method_revolut_pay::*;
pub use stripe_shared::payment_method_sepa_debit::*;
pub use stripe_shared::payment_method_sofort::*;
pub use stripe_shared::payment_method_swish::*;
pub use stripe_shared::payment_method_us_bank_account::*;
pub use stripe_shared::payment_method_us_bank_account_blocked::*;
pub use stripe_shared::payment_method_us_bank_account_status_details::*;
pub use stripe_shared::payment_method_wechat_pay::*;
pub use stripe_shared::payment_method_zip::*;
pub use stripe_shared::paypal_seller_protection::*;
pub use stripe_shared::sepa_debit_generated_from::*;
pub mod source;
pub use stripe_shared::source::*;
pub use stripe_shared::source_code_verification_flow::*;
#[doc(hidden)]
pub mod source_mandate_notification;
#[doc(inline)]
pub use source_mandate_notification::*;
#[doc(hidden)]
pub mod source_mandate_notification_acss_debit_data;
#[doc(inline)]
pub use source_mandate_notification_acss_debit_data::*;
#[doc(hidden)]
pub mod source_mandate_notification_bacs_debit_data;
#[doc(inline)]
pub use source_mandate_notification_bacs_debit_data::*;
#[doc(hidden)]
pub mod source_mandate_notification_sepa_debit_data;
#[doc(inline)]
pub use source_mandate_notification_sepa_debit_data::*;
pub use stripe_shared::source_order::*;
pub use stripe_shared::source_order_item::*;
pub use stripe_shared::source_owner::*;
pub use stripe_shared::source_receiver_flow::*;
pub use stripe_shared::source_redirect_flow::*;
pub use stripe_shared::source_transaction::*;
pub use stripe_shared::source_transaction_ach_credit_transfer_data::*;
pub use stripe_shared::source_transaction_chf_credit_transfer_data::*;
pub use stripe_shared::source_transaction_gbp_credit_transfer_data::*;
pub use stripe_shared::source_transaction_paper_check_data::*;
pub use stripe_shared::source_transaction_sepa_credit_transfer_data::*;
pub use stripe_shared::source_type_ach_credit_transfer::*;
pub use stripe_shared::source_type_ach_debit::*;
pub use stripe_shared::source_type_acss_debit::*;
pub use stripe_shared::source_type_alipay::*;
pub use stripe_shared::source_type_au_becs_debit::*;
pub use stripe_shared::source_type_bancontact::*;
pub use stripe_shared::source_type_card::*;
pub use stripe_shared::source_type_card_present::*;
pub use stripe_shared::source_type_eps::*;
pub use stripe_shared::source_type_giropay::*;
pub use stripe_shared::source_type_ideal::*;
pub use stripe_shared::source_type_klarna::*;
pub use stripe_shared::source_type_multibanco::*;
pub use stripe_shared::source_type_p24::*;
pub use stripe_shared::source_type_sepa_credit_transfer::*;
pub use stripe_shared::source_type_sepa_debit::*;
pub use stripe_shared::source_type_sofort::*;
pub use stripe_shared::source_type_three_d_secure::*;
pub use stripe_shared::source_type_wechat::*;
pub use stripe_shared::three_d_secure_details_charge::*;
pub use stripe_shared::three_d_secure_usage::*;
pub use stripe_shared::us_bank_account_networks::*;
