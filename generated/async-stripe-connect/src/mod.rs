#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(non_camel_case_types)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Connect` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_connect;

miniserde::make_place!(Place);
pub mod account;
pub use account_link::types::*;
pub use stripe_shared::account::*;
pub use stripe_shared::account_annual_revenue::*;
pub use stripe_shared::account_bacs_debit_payments_settings::*;
pub use stripe_shared::account_branding_settings::*;
pub use stripe_shared::account_business_profile::*;
pub use stripe_shared::account_capabilities::*;
pub use stripe_shared::account_capability_future_requirements::*;
pub use stripe_shared::account_capability_requirements::*;
pub use stripe_shared::account_card_issuing_settings::*;
pub use stripe_shared::account_card_payments_settings::*;
pub use stripe_shared::account_dashboard_settings::*;
pub use stripe_shared::account_decline_charge_on::*;
pub use stripe_shared::account_future_requirements::*;
pub use stripe_shared::account_group_membership::*;
pub use stripe_shared::account_invoices_settings::*;
pub mod account_link;
pub use account_session::types::*;
pub use stripe_shared::account_monthly_estimated_revenue::*;
pub use stripe_shared::account_payments_settings::*;
pub use stripe_shared::account_payout_settings::*;
pub use stripe_shared::account_requirements::*;
pub use stripe_shared::account_requirements_alternative::*;
pub use stripe_shared::account_requirements_error::*;
pub use stripe_shared::account_sepa_debit_payments_settings::*;
pub mod account_session;
pub use stripe_shared::account_settings::*;
pub use stripe_shared::account_terms_of_service::*;
pub use stripe_shared::account_tos_acceptance::*;
pub use stripe_shared::account_treasury_settings::*;
pub use stripe_shared::account_unification_account_controller::*;
pub use stripe_shared::account_unification_account_controller_fees::*;
pub use stripe_shared::account_unification_account_controller_losses::*;
pub use stripe_shared::account_unification_account_controller_stripe_dashboard::*;
pub use stripe_shared::application::*;
pub mod application_fee;
pub use apps_secret::types::*;
pub use stripe_shared::application_fee::*;
pub mod apps_secret;
pub mod capability;
pub use stripe_shared::capability::*;
pub use stripe_shared::connect_account_reference::*;
#[doc(hidden)]
pub mod connect_embedded_account_config_claim;
#[doc(inline)]
pub use connect_embedded_account_config_claim::*;
#[doc(hidden)]
pub mod connect_embedded_account_features_claim;
#[doc(inline)]
pub use connect_embedded_account_features_claim::*;
#[doc(hidden)]
pub mod connect_embedded_account_session_create_components;
#[doc(inline)]
pub use connect_embedded_account_session_create_components::*;
#[doc(hidden)]
pub mod connect_embedded_base_config_claim;
#[doc(inline)]
pub use connect_embedded_base_config_claim::*;
#[doc(hidden)]
pub mod connect_embedded_base_features;
#[doc(inline)]
pub use connect_embedded_base_features::*;
#[doc(hidden)]
pub mod connect_embedded_disputes_list_config;
#[doc(inline)]
pub use connect_embedded_disputes_list_config::*;
#[doc(hidden)]
pub mod connect_embedded_disputes_list_features;
#[doc(inline)]
pub use connect_embedded_disputes_list_features::*;
#[doc(hidden)]
pub mod connect_embedded_financial_account_config_claim;
#[doc(inline)]
pub use connect_embedded_financial_account_config_claim::*;
#[doc(hidden)]
pub mod connect_embedded_financial_account_features;
#[doc(inline)]
pub use connect_embedded_financial_account_features::*;
#[doc(hidden)]
pub mod connect_embedded_financial_account_transactions_config_claim;
#[doc(inline)]
pub use connect_embedded_financial_account_transactions_config_claim::*;
#[doc(hidden)]
pub mod connect_embedded_financial_account_transactions_features;
#[doc(inline)]
pub use connect_embedded_financial_account_transactions_features::*;
#[doc(hidden)]
pub mod connect_embedded_issuing_card_config_claim;
#[doc(inline)]
pub use connect_embedded_issuing_card_config_claim::*;
#[doc(hidden)]
pub mod connect_embedded_issuing_card_features;
#[doc(inline)]
pub use connect_embedded_issuing_card_features::*;
#[doc(hidden)]
pub mod connect_embedded_issuing_cards_list_config_claim;
#[doc(inline)]
pub use connect_embedded_issuing_cards_list_config_claim::*;
#[doc(hidden)]
pub mod connect_embedded_issuing_cards_list_features;
#[doc(inline)]
pub use connect_embedded_issuing_cards_list_features::*;
#[doc(hidden)]
pub mod connect_embedded_payment_disputes_config;
#[doc(inline)]
pub use connect_embedded_payment_disputes_config::*;
#[doc(hidden)]
pub mod connect_embedded_payment_disputes_features;
#[doc(inline)]
pub use connect_embedded_payment_disputes_features::*;
#[doc(hidden)]
pub mod connect_embedded_payments_config_claim;
#[doc(inline)]
pub use connect_embedded_payments_config_claim::*;
#[doc(hidden)]
pub mod connect_embedded_payments_features;
#[doc(inline)]
pub use connect_embedded_payments_features::*;
#[doc(hidden)]
pub mod connect_embedded_payouts_config;
#[doc(inline)]
pub use connect_embedded_payouts_config::*;
#[doc(hidden)]
pub mod connect_embedded_payouts_features;
#[doc(inline)]
pub use connect_embedded_payouts_features::*;
pub use country_spec::types::*;
pub mod country_spec;
#[doc(hidden)]
pub mod country_spec_verification_field_details;
#[doc(inline)]
pub use country_spec_verification_field_details::*;
#[doc(hidden)]
pub mod country_spec_verification_fields;
#[doc(inline)]
pub use country_spec_verification_fields::*;
pub use stripe_shared::deleted_account::*;
pub use stripe_shared::deleted_person::*;
pub mod external_account;
pub use stripe_shared::external_account::*;
pub mod application_fee_refund;
pub use login_link::types::*;
pub use stripe_shared::application_fee_refund::*;
pub use stripe_shared::legal_entity_company::*;
pub use stripe_shared::legal_entity_company_verification::*;
pub use stripe_shared::legal_entity_company_verification_document::*;
pub use stripe_shared::legal_entity_directorship_declaration::*;
pub use stripe_shared::legal_entity_dob::*;
pub use stripe_shared::legal_entity_japan_address::*;
pub use stripe_shared::legal_entity_person_verification::*;
pub use stripe_shared::legal_entity_person_verification_document::*;
pub use stripe_shared::legal_entity_registration_date::*;
pub use stripe_shared::legal_entity_ubo_declaration::*;
pub mod login_link;
pub mod person;
pub use stripe_shared::person::*;
pub use stripe_shared::person_additional_tos_acceptance::*;
pub use stripe_shared::person_additional_tos_acceptances::*;
pub use stripe_shared::person_ethnicity_details::*;
pub use stripe_shared::person_future_requirements::*;
pub use stripe_shared::person_race_details::*;
pub use stripe_shared::person_relationship::*;
pub use stripe_shared::person_requirements::*;
pub use stripe_shared::person_us_cfpb_data::*;
pub use stripe_shared::platform_earning_fee_source::*;
#[doc(hidden)]
pub mod secret_service_resource_scope;
#[doc(inline)]
pub use secret_service_resource_scope::*;
pub mod topup;
pub use stripe_shared::topup::*;
pub mod transfer;
pub use stripe_shared::transfer::*;
pub use stripe_shared::transfer_data::*;
pub mod transfer_reversal;
pub use stripe_shared::transfer_reversal::*;
pub use stripe_shared::transfer_schedule::*;
