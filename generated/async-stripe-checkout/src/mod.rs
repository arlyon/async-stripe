#![recursion_limit = "256"]
#![deny(clippy::large_stack_frames)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(non_camel_case_types)]

//! This crate provides Rust bindings to the Stripe HTTP API,
//! centered around [Checkout Sessions](https://stripe.com/docs/api/checkout/sessions).

extern crate self as stripe_checkout;

miniserde::make_place!(Place);
pub mod checkout_session;
pub use stripe_shared::checkout_acss_debit_mandate_options::*;
pub use stripe_shared::checkout_acss_debit_payment_method_options::*;
pub use stripe_shared::checkout_affirm_payment_method_options::*;
pub use stripe_shared::checkout_afterpay_clearpay_payment_method_options::*;
pub use stripe_shared::checkout_alipay_payment_method_options::*;
pub use stripe_shared::checkout_alma_payment_method_options::*;
pub use stripe_shared::checkout_amazon_pay_payment_method_options::*;
pub use stripe_shared::checkout_au_becs_debit_payment_method_options::*;
pub use stripe_shared::checkout_bacs_debit_payment_method_options::*;
pub use stripe_shared::checkout_bancontact_payment_method_options::*;
pub use stripe_shared::checkout_billie_payment_method_options::*;
pub use stripe_shared::checkout_boleto_payment_method_options::*;
pub use stripe_shared::checkout_card_installments_options::*;
pub use stripe_shared::checkout_card_payment_method_options::*;
pub use stripe_shared::checkout_cashapp_payment_method_options::*;
pub use stripe_shared::checkout_customer_balance_bank_transfer_payment_method_options::*;
pub use stripe_shared::checkout_customer_balance_payment_method_options::*;
pub use stripe_shared::checkout_eps_payment_method_options::*;
pub use stripe_shared::checkout_fpx_payment_method_options::*;
pub use stripe_shared::checkout_giropay_payment_method_options::*;
pub use stripe_shared::checkout_grab_pay_payment_method_options::*;
pub use stripe_shared::checkout_ideal_payment_method_options::*;
pub use stripe_shared::checkout_kakao_pay_payment_method_options::*;
pub use stripe_shared::checkout_klarna_payment_method_options::*;
pub use stripe_shared::checkout_konbini_payment_method_options::*;
pub use stripe_shared::checkout_kr_card_payment_method_options::*;
pub use stripe_shared::checkout_link_payment_method_options::*;
pub use stripe_shared::checkout_link_wallet_options::*;
pub use stripe_shared::checkout_mobilepay_payment_method_options::*;
pub use stripe_shared::checkout_multibanco_payment_method_options::*;
pub use stripe_shared::checkout_naver_pay_payment_method_options::*;
pub use stripe_shared::checkout_oxxo_payment_method_options::*;
pub use stripe_shared::checkout_p24_payment_method_options::*;
pub use stripe_shared::checkout_payco_payment_method_options::*;
pub use stripe_shared::checkout_payment_method_options_mandate_options_bacs_debit::*;
pub use stripe_shared::checkout_payment_method_options_mandate_options_sepa_debit::*;
pub use stripe_shared::checkout_paynow_payment_method_options::*;
pub use stripe_shared::checkout_paypal_payment_method_options::*;
pub use stripe_shared::checkout_payto_payment_method_options::*;
pub use stripe_shared::checkout_pix_payment_method_options::*;
pub use stripe_shared::checkout_revolut_pay_payment_method_options::*;
pub use stripe_shared::checkout_samsung_pay_payment_method_options::*;
pub use stripe_shared::checkout_satispay_payment_method_options::*;
pub use stripe_shared::checkout_sepa_debit_payment_method_options::*;
pub use stripe_shared::checkout_session::*;
pub use stripe_shared::checkout_session_payment_method_options::*;
pub use stripe_shared::checkout_session_wallet_options::*;
pub use stripe_shared::checkout_sofort_payment_method_options::*;
pub use stripe_shared::checkout_swish_payment_method_options::*;
pub use stripe_shared::checkout_twint_payment_method_options::*;
pub use stripe_shared::checkout_us_bank_account_payment_method_options::*;
pub use stripe_shared::payment_pages_checkout_session_adaptive_pricing::*;
pub use stripe_shared::payment_pages_checkout_session_after_expiration::*;
pub use stripe_shared::payment_pages_checkout_session_after_expiration_recovery::*;
pub use stripe_shared::payment_pages_checkout_session_automatic_tax::*;
pub use stripe_shared::payment_pages_checkout_session_branding_settings::*;
pub use stripe_shared::payment_pages_checkout_session_branding_settings_icon::*;
pub use stripe_shared::payment_pages_checkout_session_branding_settings_logo::*;
pub use stripe_shared::payment_pages_checkout_session_business_name::*;
pub use stripe_shared::payment_pages_checkout_session_checkout_address_details::*;
pub use stripe_shared::payment_pages_checkout_session_collected_information::*;
pub use stripe_shared::payment_pages_checkout_session_consent::*;
pub use stripe_shared::payment_pages_checkout_session_consent_collection::*;
pub use stripe_shared::payment_pages_checkout_session_currency_conversion::*;
pub use stripe_shared::payment_pages_checkout_session_custom_fields::*;
pub use stripe_shared::payment_pages_checkout_session_custom_fields_dropdown::*;
pub use stripe_shared::payment_pages_checkout_session_custom_fields_label::*;
pub use stripe_shared::payment_pages_checkout_session_custom_fields_numeric::*;
pub use stripe_shared::payment_pages_checkout_session_custom_fields_option::*;
pub use stripe_shared::payment_pages_checkout_session_custom_fields_text::*;
pub use stripe_shared::payment_pages_checkout_session_custom_text::*;
pub use stripe_shared::payment_pages_checkout_session_custom_text_position::*;
pub use stripe_shared::payment_pages_checkout_session_customer_details::*;
pub use stripe_shared::payment_pages_checkout_session_discount::*;
pub use stripe_shared::payment_pages_checkout_session_individual_name::*;
pub use stripe_shared::payment_pages_checkout_session_invoice_creation::*;
pub use stripe_shared::payment_pages_checkout_session_invoice_settings::*;
pub use stripe_shared::payment_pages_checkout_session_name_collection::*;
pub use stripe_shared::payment_pages_checkout_session_optional_item::*;
pub use stripe_shared::payment_pages_checkout_session_optional_item_adjustable_quantity::*;
pub use stripe_shared::payment_pages_checkout_session_payment_method_reuse_agreement::*;
pub use stripe_shared::payment_pages_checkout_session_permissions::*;
pub use stripe_shared::payment_pages_checkout_session_phone_number_collection::*;
pub use stripe_shared::payment_pages_checkout_session_saved_payment_method_options::*;
pub use stripe_shared::payment_pages_checkout_session_shipping_address_collection::*;
pub use stripe_shared::payment_pages_checkout_session_shipping_cost::*;
pub use stripe_shared::payment_pages_checkout_session_shipping_option::*;
pub use stripe_shared::payment_pages_checkout_session_tax_id::*;
pub use stripe_shared::payment_pages_checkout_session_tax_id_collection::*;
pub use stripe_shared::payment_pages_checkout_session_total_details::*;
pub use stripe_shared::payment_pages_checkout_session_total_details_resource_breakdown::*;
pub use stripe_shared::payment_pages_private_card_payment_method_options_resource_restrictions::*;
