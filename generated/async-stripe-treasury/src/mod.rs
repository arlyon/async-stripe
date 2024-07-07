#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//! This crate provides Rust bindings to the Stripe HTTP API, specifically
//! for requests mentioned in the `Treasury` section of the [Stripe API docs](https://stripe.com/docs/api)

extern crate self as stripe_treasury;

miniserde::make_place!(Place);
#[doc(hidden)]
pub mod inbound_transfers;
#[doc(inline)]
pub use inbound_transfers::*;
#[doc(hidden)]
pub mod inbound_transfers_payment_method_details_us_bank_account;
#[doc(inline)]
pub use inbound_transfers_payment_method_details_us_bank_account::*;
#[doc(hidden)]
pub mod outbound_payments_payment_method_details;
#[doc(inline)]
pub use outbound_payments_payment_method_details::*;
#[doc(hidden)]
pub mod outbound_payments_payment_method_details_financial_account;
#[doc(inline)]
pub use outbound_payments_payment_method_details_financial_account::*;
#[doc(hidden)]
pub mod outbound_payments_payment_method_details_us_bank_account;
#[doc(inline)]
pub use outbound_payments_payment_method_details_us_bank_account::*;
#[doc(hidden)]
pub mod outbound_transfers_payment_method_details;
#[doc(inline)]
pub use outbound_transfers_payment_method_details::*;
#[doc(hidden)]
pub mod outbound_transfers_payment_method_details_us_bank_account;
#[doc(inline)]
pub use outbound_transfers_payment_method_details_us_bank_account::*;
#[doc(hidden)]
pub mod received_payment_method_details_financial_account;
#[doc(inline)]
pub use received_payment_method_details_financial_account::*;
pub use treasury_credit_reversal::types::*;
pub mod treasury_credit_reversal;
pub use treasury_debit_reversal::types::*;
pub mod treasury_debit_reversal;
pub use treasury_financial_account::types::*;
pub mod treasury_financial_account;
#[doc(hidden)]
pub mod treasury_financial_account_features;
#[doc(inline)]
pub use treasury_financial_account_features::*;
pub use treasury_inbound_transfer::types::*;
pub mod treasury_inbound_transfer;
pub use treasury_outbound_payment::types::*;
pub mod treasury_outbound_payment;
pub use treasury_outbound_transfer::types::*;
pub mod treasury_outbound_transfer;
pub use treasury_received_credit::types::*;
pub mod treasury_received_credit;
pub use treasury_received_debit::types::*;
pub mod treasury_received_debit;
pub use treasury_transaction::types::*;
pub mod treasury_transaction;
pub use treasury_transaction_entry::types::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_aba_record;
pub mod treasury_transaction_entry;
#[doc(inline)]
pub use treasury_financial_accounts_resource_aba_record::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_aba_toggle_settings;
#[doc(inline)]
pub use treasury_financial_accounts_resource_aba_toggle_settings::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_ach_toggle_settings;
#[doc(inline)]
pub use treasury_financial_accounts_resource_ach_toggle_settings::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_balance;
#[doc(inline)]
pub use treasury_financial_accounts_resource_balance::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_closed_status_details;
#[doc(inline)]
pub use treasury_financial_accounts_resource_closed_status_details::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_financial_address;
#[doc(inline)]
pub use treasury_financial_accounts_resource_financial_address::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_financial_addresses_features;
#[doc(inline)]
pub use treasury_financial_accounts_resource_financial_addresses_features::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_inbound_transfers;
#[doc(inline)]
pub use treasury_financial_accounts_resource_inbound_transfers::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_outbound_payments;
#[doc(inline)]
pub use treasury_financial_accounts_resource_outbound_payments::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_outbound_transfers;
#[doc(inline)]
pub use treasury_financial_accounts_resource_outbound_transfers::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_platform_restrictions;
#[doc(inline)]
pub use treasury_financial_accounts_resource_platform_restrictions::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_status_details;
#[doc(inline)]
pub use treasury_financial_accounts_resource_status_details::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_toggle_settings;
#[doc(inline)]
pub use treasury_financial_accounts_resource_toggle_settings::*;
#[doc(hidden)]
pub mod treasury_financial_accounts_resource_toggles_setting_status_details;
#[doc(inline)]
pub use treasury_financial_accounts_resource_toggles_setting_status_details::*;
#[doc(hidden)]
pub mod treasury_inbound_transfers_resource_failure_details;
#[doc(inline)]
pub use treasury_inbound_transfers_resource_failure_details::*;
#[doc(hidden)]
pub mod treasury_inbound_transfers_resource_inbound_transfer_resource_linked_flows;
#[doc(inline)]
pub use treasury_inbound_transfers_resource_inbound_transfer_resource_linked_flows::*;
#[doc(hidden)]
pub mod treasury_inbound_transfers_resource_inbound_transfer_resource_status_transitions;
#[doc(inline)]
pub use treasury_inbound_transfers_resource_inbound_transfer_resource_status_transitions::*;
#[doc(hidden)]
pub mod treasury_outbound_payments_resource_outbound_payment_resource_end_user_details;
#[doc(inline)]
pub use treasury_outbound_payments_resource_outbound_payment_resource_end_user_details::*;
#[doc(hidden)]
pub mod treasury_outbound_payments_resource_outbound_payment_resource_status_transitions;
#[doc(inline)]
pub use treasury_outbound_payments_resource_outbound_payment_resource_status_transitions::*;
#[doc(hidden)]
pub mod treasury_outbound_payments_resource_returned_status;
#[doc(inline)]
pub use treasury_outbound_payments_resource_returned_status::*;
#[doc(hidden)]
pub mod treasury_outbound_transfers_resource_returned_details;
#[doc(inline)]
pub use treasury_outbound_transfers_resource_returned_details::*;
#[doc(hidden)]
pub mod treasury_outbound_transfers_resource_status_transitions;
#[doc(inline)]
pub use treasury_outbound_transfers_resource_status_transitions::*;
#[doc(hidden)]
pub mod treasury_received_credits_resource_linked_flows;
#[doc(inline)]
pub use treasury_received_credits_resource_linked_flows::*;
#[doc(hidden)]
pub mod treasury_received_credits_resource_reversal_details;
#[doc(inline)]
pub use treasury_received_credits_resource_reversal_details::*;
#[doc(hidden)]
pub mod treasury_received_credits_resource_source_flows_details;
#[doc(inline)]
pub use treasury_received_credits_resource_source_flows_details::*;
#[doc(hidden)]
pub mod treasury_received_credits_resource_status_transitions;
#[doc(inline)]
pub use treasury_received_credits_resource_status_transitions::*;
#[doc(hidden)]
pub mod treasury_received_debits_resource_debit_reversal_linked_flows;
#[doc(inline)]
pub use treasury_received_debits_resource_debit_reversal_linked_flows::*;
#[doc(hidden)]
pub mod treasury_received_debits_resource_linked_flows;
#[doc(inline)]
pub use treasury_received_debits_resource_linked_flows::*;
#[doc(hidden)]
pub mod treasury_received_debits_resource_reversal_details;
#[doc(inline)]
pub use treasury_received_debits_resource_reversal_details::*;
#[doc(hidden)]
pub mod treasury_received_debits_resource_status_transitions;
#[doc(inline)]
pub use treasury_received_debits_resource_status_transitions::*;
#[doc(hidden)]
pub mod treasury_shared_resource_billing_details;
#[doc(inline)]
pub use treasury_shared_resource_billing_details::*;
#[doc(hidden)]
pub mod treasury_shared_resource_initiating_payment_method_details_initiating_payment_method_details;
#[doc(inline)]
pub use treasury_shared_resource_initiating_payment_method_details_initiating_payment_method_details::*;
#[doc(hidden)]
pub mod treasury_shared_resource_initiating_payment_method_details_us_bank_account;
#[doc(inline)]
pub use treasury_shared_resource_initiating_payment_method_details_us_bank_account::*;
#[doc(hidden)]
pub mod treasury_transactions_resource_abstract_transaction_resource_status_transitions;
#[doc(inline)]
pub use treasury_transactions_resource_abstract_transaction_resource_status_transitions::*;
#[doc(hidden)]
pub mod treasury_transactions_resource_balance_impact;
#[doc(inline)]
pub use treasury_transactions_resource_balance_impact::*;
#[doc(hidden)]
pub mod treasury_transactions_resource_flow_details;
#[doc(inline)]
pub use treasury_transactions_resource_flow_details::*;
