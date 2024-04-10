#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API,
//! centered around [Checkout Sessions](https://stripe.com/docs/api/checkout/sessions).

extern crate self as stripe_checkout;

miniserde::make_place!(Place);
pub use checkout_session::types::*;
#[doc(hidden)]
pub mod checkout_acss_debit_mandate_options;
pub mod checkout_session;
#[doc(inline)]
pub use checkout_acss_debit_mandate_options::*;
#[doc(hidden)]
pub mod checkout_acss_debit_payment_method_options;
#[doc(inline)]
pub use checkout_acss_debit_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_affirm_payment_method_options;
#[doc(inline)]
pub use checkout_affirm_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_afterpay_clearpay_payment_method_options;
#[doc(inline)]
pub use checkout_afterpay_clearpay_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_alipay_payment_method_options;
#[doc(inline)]
pub use checkout_alipay_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_au_becs_debit_payment_method_options;
#[doc(inline)]
pub use checkout_au_becs_debit_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_bacs_debit_payment_method_options;
#[doc(inline)]
pub use checkout_bacs_debit_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_bancontact_payment_method_options;
#[doc(inline)]
pub use checkout_bancontact_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_boleto_payment_method_options;
#[doc(inline)]
pub use checkout_boleto_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_card_installments_options;
#[doc(inline)]
pub use checkout_card_installments_options::*;
#[doc(hidden)]
pub mod checkout_card_payment_method_options;
#[doc(inline)]
pub use checkout_card_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_cashapp_payment_method_options;
#[doc(inline)]
pub use checkout_cashapp_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_customer_balance_bank_transfer_payment_method_options;
#[doc(inline)]
pub use checkout_customer_balance_bank_transfer_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_customer_balance_payment_method_options;
#[doc(inline)]
pub use checkout_customer_balance_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_eps_payment_method_options;
#[doc(inline)]
pub use checkout_eps_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_fpx_payment_method_options;
#[doc(inline)]
pub use checkout_fpx_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_giropay_payment_method_options;
#[doc(inline)]
pub use checkout_giropay_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_grab_pay_payment_method_options;
#[doc(inline)]
pub use checkout_grab_pay_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_ideal_payment_method_options;
#[doc(inline)]
pub use checkout_ideal_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_klarna_payment_method_options;
#[doc(inline)]
pub use checkout_klarna_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_konbini_payment_method_options;
#[doc(inline)]
pub use checkout_konbini_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_link_payment_method_options;
#[doc(inline)]
pub use checkout_link_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_oxxo_payment_method_options;
#[doc(inline)]
pub use checkout_oxxo_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_p24_payment_method_options;
#[doc(inline)]
pub use checkout_p24_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_paynow_payment_method_options;
#[doc(inline)]
pub use checkout_paynow_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_paypal_payment_method_options;
#[doc(inline)]
pub use checkout_paypal_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_pix_payment_method_options;
#[doc(inline)]
pub use checkout_pix_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_revolut_pay_payment_method_options;
#[doc(inline)]
pub use checkout_revolut_pay_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_sepa_debit_payment_method_options;
#[doc(inline)]
pub use checkout_sepa_debit_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_session_payment_method_options;
#[doc(inline)]
pub use checkout_session_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_sofort_payment_method_options;
#[doc(inline)]
pub use checkout_sofort_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_swish_payment_method_options;
#[doc(inline)]
pub use checkout_swish_payment_method_options::*;
#[doc(hidden)]
pub mod checkout_us_bank_account_payment_method_options;
#[doc(inline)]
pub use checkout_us_bank_account_payment_method_options::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_after_expiration;
#[doc(inline)]
pub use payment_pages_checkout_session_after_expiration::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_after_expiration_recovery;
#[doc(inline)]
pub use payment_pages_checkout_session_after_expiration_recovery::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_automatic_tax;
#[doc(inline)]
pub use payment_pages_checkout_session_automatic_tax::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_consent;
#[doc(inline)]
pub use payment_pages_checkout_session_consent::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_consent_collection;
#[doc(inline)]
pub use payment_pages_checkout_session_consent_collection::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_currency_conversion;
#[doc(inline)]
pub use payment_pages_checkout_session_currency_conversion::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_fields;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_fields::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_fields_dropdown;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_fields_dropdown::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_fields_label;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_fields_label::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_fields_numeric;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_fields_numeric::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_fields_option;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_fields_option::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_fields_text;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_fields_text::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_text;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_text::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_custom_text_position;
#[doc(inline)]
pub use payment_pages_checkout_session_custom_text_position::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_customer_details;
#[doc(inline)]
pub use payment_pages_checkout_session_customer_details::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_invoice_creation;
#[doc(inline)]
pub use payment_pages_checkout_session_invoice_creation::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_invoice_settings;
#[doc(inline)]
pub use payment_pages_checkout_session_invoice_settings::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_payment_method_reuse_agreement;
#[doc(inline)]
pub use payment_pages_checkout_session_payment_method_reuse_agreement::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_phone_number_collection;
#[doc(inline)]
pub use payment_pages_checkout_session_phone_number_collection::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_shipping_address_collection;
#[doc(inline)]
pub use payment_pages_checkout_session_shipping_address_collection::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_shipping_cost;
#[doc(inline)]
pub use payment_pages_checkout_session_shipping_cost::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_shipping_option;
#[doc(inline)]
pub use payment_pages_checkout_session_shipping_option::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_tax_id;
#[doc(inline)]
pub use payment_pages_checkout_session_tax_id::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_tax_id_collection;
#[doc(inline)]
pub use payment_pages_checkout_session_tax_id_collection::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_total_details;
#[doc(inline)]
pub use payment_pages_checkout_session_total_details::*;
#[doc(hidden)]
pub mod payment_pages_checkout_session_total_details_resource_breakdown;
#[doc(inline)]
pub use payment_pages_checkout_session_total_details_resource_breakdown::*;
